import pyspark
import grpc
import re
import uuid
import argparse
import sys
import json
import itertools

from pyspark.sql import SparkSession

import catalog_pb2
import catalog_pb2_grpc
import idp_pb2
import idp_pb2_grpc

# init_argparse initializes the argparse module
# we will have flags for the opencore endpoint and the service account credentials we want to use
def init_argparse() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        usage="%(prog)s [OPTION]",
        description="run the spark example controller."
    )
    parser.add_argument(
        "-v", "--version", action="version",
        version = f"{parser.prog} version 1.0.0"
    )
    parser.add_argument('--service_account_id')
    parser.add_argument('--service_account_token')
    parser.add_argument('--opencore_addr')
    return parser

# Opencore is a wrapper around the opencore service which exposes domain specific methods
# like "list_pending_etl_jobs" etc.
class Opencore:

    # __init__ connects to opencore and sets up the gRPC channel and the service stubs
    # it also performs a login and saves access and refresh tokens
    def __init__(self, addr, sa_id, sa_token):
        self.addr = addr
        self.channel = grpc.insecure_channel(addr)
        self.idp_auth = idp_pb2_grpc.AuthenticationStub(self.channel)
        self.resources = catalog_pb2_grpc.ResourcesStub(self.channel)
        self.permissions = catalog_pb2_grpc.PermissionsStub(self.channel)
        self.events = catalog_pb2_grpc.EventsStub(self.channel)
        self.locks = catalog_pb2_grpc.LocksStub(self.channel)

        resp = self.idp_auth.Login(idp_pb2.LoginRequest(service_account_id=sa_id, password=sa_token))
        self.access_token = resp.access_token
        self.refresh_token = resp.refresh_token

    # refresh the access and refresh token
    def refresh(self):
        resp = self.idp_auth.Refresh(idp_pb2.RefreshRequest(refresh_token=self.refresh_token))
        self.access_token = resp.access_token
        self.refresh_token = resp.refresh_token

    # list all pending etl jobs using the list endpoint
    def list_pending_etl_jobs(self):
        metadata = (("authorization", "Bearer "+self.access_token),)
        resp = self.resources.List(catalog_pb2.ListResourcesRequest(
            kind="etl-job",
            filter='$.state == "PENDING"',
        ), metadata=metadata)
        return resp
    
    # get a datasource by its name
    def get_datasource_by_name(self, name):
        metadata = (("authorization", "Bearer "+self.access_token),)
        resp = self.resources.List(catalog_pb2.ListResourcesRequest(
            kind="datasource",
            filter=f'$.name == "{name}"',
        ), metadata=metadata)
        for ds in resp:
            return ds

    # get a datasource by its id
    def get_resource_by_id(self, id):
        metadata = (("authorization", "Bearer "+self.access_token),)
        resp = self.resources.Get(catalog_pb2.GetResourceRequest(
            id=id,
        ), metadata=metadata)
        return resp

    # check if a user has the right to do action on resource
    # This includes permissions through group memberships
    def check(self, user, resource, action):
        metadata = (("authorization", "Bearer "+self.access_token),)
        resp = self.permissions.Check(catalog_pb2.PermissionCheckRequest(
            resource_id=resource,
            principal_id=user,
            action=action,
        ), metadata=metadata)
        return resp.granted

    # listen for new etl jobs using the events api
    def listen_for_new_etl_jobs(self):
        metadata = (("authorization", "Bearer "+self.access_token),)
        events = self.events.Subscribe(catalog_pb2.SubscribeRequest(
            event_type=catalog_pb2.CREATE, 
            resource_kind="etl-job",
        ), metadata=metadata)    
        return events

    # set_state sets the state of a etl job
    def set_state(self, job_id, state):
        metadata = (("authorization", "Bearer "+self.access_token),)
        data=json.dumps({"state": state})
        resp = self.resources.Update(catalog_pb2.UpdateResourceRequest(
            id=job_id,
            data=data,
        ), metadata=metadata)
        return resp
    
    # lock locks a resource
    def lock(self, id):
        return self.locks.TryLock(catalog_pb2.LockRequest(lock_id=id))

# random_table_name returns a random table name whic hcan be used in sql
def random_table_name():
    return "table_"+str(uuid.uuid4()).replace('-','_')

# prepare_query scans a query for datasource references like "@my_data_source.my_table"
# When it finds one, it retrieves the data source and checks if the creator of the etl job is 
# allowed to "read" the data source. If so, it replaces the "@my_data_source.my_table" with
# a random table name and injects the table into spark using the jdbc credentials from the data source.
# The datasource reference is then replaced with the new table name.
def prepare_query(query, opencore, spark, creator_id):
    print("prepare query")
    expr = re.compile('"?@[a-zA-Z0-9_]+"?\."?[a-zA-Z0-9_]+"?')
    for hit in expr.finditer(query):
        print("got hit", hit.group())
        parts = hit.group().split('.')
        local_name = random_table_name()
        try:
            ds_name = parts[0].replace('"','').replace('@','')
            table_name = parts[1].replace('"','')
            print(f"found source table {table_name} in data source {ds_name}, resolving...")
            resource = opencore.get_datasource_by_name(ds_name)
            if not opencore.check(creator_id, resource.id, "read"):
                raise "auth check failed"
            ds = json.loads(resource.data)
            df = spark.read.jdbc(ds["url"], table_name, properties=ds["properties"])
            df.createOrReplaceTempView(local_name)
            print("found an external table and registered it")
            query = query.replace(hit.group(), local_name)
        except Exception as e:
            print("failed to find external table", hit.group(), e)
            raise e
    return query    

# exec_query executes a prepared query
# This will also check if the creator of the etl job is allowed to "write" to the target data source.
def exec_query(query, opencore, spark, target, creator_id):
    print("executing query", query)
    df = spark.sql(query)
    parts = target.split('.')
    ds_name = parts[0].replace('"','').replace('@','')
    table_name = parts[1].replace('"','')
    resource = opencore.get_datasource_by_name(ds_name)
    if not opencore.check(creator_id, resource.id, "write"):
        raise "auth check failed"
    ds = json.loads(resource.data)
    df.write.jdbc(ds["url"], table_name, mode="overwrite", properties=ds["properties"])

# entrypoint of the app
def main():
    print(sys.argv)
    parser = init_argparse()
    args = parser.parse_args()

    # connect to opencore
    opencore = Opencore(args.opencore_addr, args.service_account_id, args.service_account_token)

    # connect to spark
    spark = SparkSession \
        .builder \
        .appName("spark example") \
        .config("spark.some.config.option", "some-value") \
        .getOrCreate()
    
    while True:
        try:
            # get pending etl jobs and listen for newly created ones
            jobs = opencore.list_pending_etl_jobs()
            events = opencore.listen_for_new_etl_jobs()
            for doc in itertools.chain(jobs, events):
                id = doc.id
                if hasattr(doc, 'resource_id'):
                    id = doc.resource_id
                try:
                    # new job, get a lock for it
                    lock = opencore.lock(id)
                    # get creator of the job
                    doc = opencore.get_resource_by_id(id)
                    creator_id = doc.creator_id
                    # parse the job data
                    data = json.loads(doc.data)
                    print("Got new etl-job:", data)
                    # update job status
                    opencore.set_state(id, "RUNNING")
                    # prepare the query
                    query = prepare_query(data["sql"], opencore, spark, creator_id)
                    print("prepared query:", query)
                    try:
                        # execute the query
                        exec_query(query, opencore, spark, data["target"], creator_id)
                        opencore.set_state(id, "FINISHED")
                    except Exception as e:
                        print(e)
                        opencore.set_state(id, "FAILED")
                    lock.cancel()
                except Exception as e:
                    print("failed to process:", e)
                    opencore.set_state(id, "FAILED")
                    continue
                print("lock released")
        except grpc.RpcError as e:
            print(e)
            opencore.refresh()
            

print(__name__)
if __name__ == "__main__":
    main()

