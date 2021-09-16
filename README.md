opencore
========

This service exposes gRPC APIs to manage a catalog of arbitrary resources in a multi user environment. With this you will have a solid foundation to build any application you want.

Most applications today somehow work by storing and retrieving resources for their users. As a developer of these apps you create CRUD APIs for these resources all the time. Over and over again. With this service you won't have to write them anymore! By unifiying the APIs for all resources we eliminate the need to ever write a single CRUD API again. Instead you can focus on whats giving actual value to your product: the business logic. 

When building an app with opencore you basically do three things:

1. Define JSON schema specs for your resources
2. Write one or more controllers which are reacting to resource events and implementing your business logic
3. Let your users (or your frontend) call the opencore APIs to create and retrieve resources.

If you want you can also put `opencore` behind another service which may provide a domain specific API that just maps the user requests to opencore API calls. This may be beneficial if your app exceeds a certain complexity threshold.

## API overview:

### Catalog APIs

* Resources
    * manage all your resources with a unified API for Create/Retrieve/Update/Delete
    * list your resources with filter and search capabilities over all properties of your resources using jsonpath expressions
* Schemas
    * manage multiple openapi specs for the different resources kinds in your system
    * this will be used when validating create and update requests
    * create custom database indexes and constraints (@TODO)
* Permissions
    * manage who can see or use your resources
* Events
    * get live notifications when new resources are created or existing ones are updated
    * enables you to build your application following the observer pattern
* Locks
    * exposes Lock and TryLock methods
    * helps in combination with the events API building scalable workers for asyncronous tasks

### IDP APIs

* Users
    * manage users in your application
* Groups
    * users may create and manage groups for easier permission management
* Service Accounts
    * kind of special users to be used when writing controllers
    * can't be part of groups and can easily be reset
* Authentication
    * login to the platform and get a token


## Examples

The `examples` directory contains multiple examples on how to use this service. The most basic example is the `todo` example, which just shows how to use the resource API to build a simple todo app. The `calculator` example is a bit more complex, since it shows how to build controller processes which listen to resource creation events and adds additional properties to it. 

The `list` and the `locking` examples are smaller examples which just illustrate a specific API in more detail. 

Most examples are written as simple bash scripts and require `httpie` and `jq`. 

The only exection is the `web` example, which comes with its own Makefile that builds a typescript app and serves it using caddy. It builds on top of the todo example, so if you ran it before you should still have some todos in your database that this app will show you on localhost:8080.

## Build

At this point you will need `podman` and `buildah` to build and run this service locally. You don't need a rust toolchain, but if you have one I'm sure you know how to use it. Maybe I'll add a `Containerfile` and a example `compose.yaml` later.

To build the image run:
```
make image
```

If you want to run one of the examples just call 
```
make run
bash examples/<example you want>/main.sh
```
The first command will create a pod with a postgresql database and opencore exposed at `grpc://localhost:3001`.

The second command executes the example you want.
