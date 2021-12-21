import * as grpcWeb from 'grpc-web';

import * as catalog_pb from './catalog_pb';


export class ResourcesClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: catalog_pb.CreateResourceRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: catalog_pb.Resource) => void
  ): grpcWeb.ClientReadableStream<catalog_pb.Resource>;

  get(
    request: catalog_pb.GetResourceRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: catalog_pb.Resource) => void
  ): grpcWeb.ClientReadableStream<catalog_pb.Resource>;

  update(
    request: catalog_pb.UpdateResourceRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: catalog_pb.Resource) => void
  ): grpcWeb.ClientReadableStream<catalog_pb.Resource>;

  delete(
    request: catalog_pb.DeleteResourceRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: catalog_pb.Resource) => void
  ): grpcWeb.ClientReadableStream<catalog_pb.Resource>;

  list(
    request: catalog_pb.ListResourcesRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<catalog_pb.Resource>;

}

export class SchemasClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: catalog_pb.CreateSchemaRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: catalog_pb.Schema) => void
  ): grpcWeb.ClientReadableStream<catalog_pb.Schema>;

  get(
    request: catalog_pb.GetSchemaRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: catalog_pb.Schema) => void
  ): grpcWeb.ClientReadableStream<catalog_pb.Schema>;

  update(
    request: catalog_pb.UpdateSchemaRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: catalog_pb.Schema) => void
  ): grpcWeb.ClientReadableStream<catalog_pb.Schema>;

  delete(
    request: catalog_pb.DeleteSchemaRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: catalog_pb.Schema) => void
  ): grpcWeb.ClientReadableStream<catalog_pb.Schema>;

  list(
    request: catalog_pb.ListSchemasRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<catalog_pb.Schema>;

}

export class PermissionsClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  share(
    request: catalog_pb.ShareRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: catalog_pb.PermissionInfo) => void
  ): grpcWeb.ClientReadableStream<catalog_pb.PermissionInfo>;

  unshare(
    request: catalog_pb.UnshareRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: catalog_pb.PermissionInfo) => void
  ): grpcWeb.ClientReadableStream<catalog_pb.PermissionInfo>;

  get(
    request: catalog_pb.GetPermissionInfoRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: catalog_pb.PermissionInfo) => void
  ): grpcWeb.ClientReadableStream<catalog_pb.PermissionInfo>;

  list(
    request: catalog_pb.ListPermissionsRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<catalog_pb.PermissionInfo>;

  check(
    request: catalog_pb.PermissionCheckRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: catalog_pb.PermissionCheckResponse) => void
  ): grpcWeb.ClientReadableStream<catalog_pb.PermissionCheckResponse>;

}

export class EventsClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  publish(
    request: catalog_pb.PublishRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: catalog_pb.Event) => void
  ): grpcWeb.ClientReadableStream<catalog_pb.Event>;

  subscribe(
    request: catalog_pb.SubscribeRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<catalog_pb.Event>;

}

export class LocksClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  lock(
    request: catalog_pb.LockRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<catalog_pb.LockResponse>;

  tryLock(
    request: catalog_pb.LockRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<catalog_pb.LockResponse>;

}

export class ResourcesPromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: catalog_pb.CreateResourceRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<catalog_pb.Resource>;

  get(
    request: catalog_pb.GetResourceRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<catalog_pb.Resource>;

  update(
    request: catalog_pb.UpdateResourceRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<catalog_pb.Resource>;

  delete(
    request: catalog_pb.DeleteResourceRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<catalog_pb.Resource>;

  list(
    request: catalog_pb.ListResourcesRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<catalog_pb.Resource>;

}

export class SchemasPromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: catalog_pb.CreateSchemaRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<catalog_pb.Schema>;

  get(
    request: catalog_pb.GetSchemaRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<catalog_pb.Schema>;

  update(
    request: catalog_pb.UpdateSchemaRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<catalog_pb.Schema>;

  delete(
    request: catalog_pb.DeleteSchemaRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<catalog_pb.Schema>;

  list(
    request: catalog_pb.ListSchemasRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<catalog_pb.Schema>;

}

export class PermissionsPromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  share(
    request: catalog_pb.ShareRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<catalog_pb.PermissionInfo>;

  unshare(
    request: catalog_pb.UnshareRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<catalog_pb.PermissionInfo>;

  get(
    request: catalog_pb.GetPermissionInfoRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<catalog_pb.PermissionInfo>;

  list(
    request: catalog_pb.ListPermissionsRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<catalog_pb.PermissionInfo>;

  check(
    request: catalog_pb.PermissionCheckRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<catalog_pb.PermissionCheckResponse>;

}

export class EventsPromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  publish(
    request: catalog_pb.PublishRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<catalog_pb.Event>;

  subscribe(
    request: catalog_pb.SubscribeRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<catalog_pb.Event>;

}

export class LocksPromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  lock(
    request: catalog_pb.LockRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<catalog_pb.LockResponse>;

  tryLock(
    request: catalog_pb.LockRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<catalog_pb.LockResponse>;

}

