/**
 * @fileoverview gRPC-Web generated client stub for catalog
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');


var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js')
const proto = {};
proto.catalog = require('./catalog_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.catalog.ResourcesClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.catalog.ResourcesPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.CreateResourceRequest,
 *   !proto.catalog.Resource>}
 */
const methodDescriptor_Resources_Create = new grpc.web.MethodDescriptor(
  '/catalog.Resources/Create',
  grpc.web.MethodType.UNARY,
  proto.catalog.CreateResourceRequest,
  proto.catalog.Resource,
  /**
   * @param {!proto.catalog.CreateResourceRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Resource.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.CreateResourceRequest,
 *   !proto.catalog.Resource>}
 */
const methodInfo_Resources_Create = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.Resource,
  /**
   * @param {!proto.catalog.CreateResourceRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Resource.deserializeBinary
);


/**
 * @param {!proto.catalog.CreateResourceRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.catalog.Resource)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Resource>|undefined}
 *     The XHR Node Readable Stream
 */
proto.catalog.ResourcesClient.prototype.create =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/catalog.Resources/Create',
      request,
      metadata || {},
      methodDescriptor_Resources_Create,
      callback);
};


/**
 * @param {!proto.catalog.CreateResourceRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.catalog.Resource>}
 *     Promise that resolves to the response
 */
proto.catalog.ResourcesPromiseClient.prototype.create =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/catalog.Resources/Create',
      request,
      metadata || {},
      methodDescriptor_Resources_Create);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.GetResourceRequest,
 *   !proto.catalog.Resource>}
 */
const methodDescriptor_Resources_Get = new grpc.web.MethodDescriptor(
  '/catalog.Resources/Get',
  grpc.web.MethodType.UNARY,
  proto.catalog.GetResourceRequest,
  proto.catalog.Resource,
  /**
   * @param {!proto.catalog.GetResourceRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Resource.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.GetResourceRequest,
 *   !proto.catalog.Resource>}
 */
const methodInfo_Resources_Get = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.Resource,
  /**
   * @param {!proto.catalog.GetResourceRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Resource.deserializeBinary
);


/**
 * @param {!proto.catalog.GetResourceRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.catalog.Resource)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Resource>|undefined}
 *     The XHR Node Readable Stream
 */
proto.catalog.ResourcesClient.prototype.get =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/catalog.Resources/Get',
      request,
      metadata || {},
      methodDescriptor_Resources_Get,
      callback);
};


/**
 * @param {!proto.catalog.GetResourceRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.catalog.Resource>}
 *     Promise that resolves to the response
 */
proto.catalog.ResourcesPromiseClient.prototype.get =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/catalog.Resources/Get',
      request,
      metadata || {},
      methodDescriptor_Resources_Get);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.UpdateResourceRequest,
 *   !proto.catalog.Resource>}
 */
const methodDescriptor_Resources_Update = new grpc.web.MethodDescriptor(
  '/catalog.Resources/Update',
  grpc.web.MethodType.UNARY,
  proto.catalog.UpdateResourceRequest,
  proto.catalog.Resource,
  /**
   * @param {!proto.catalog.UpdateResourceRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Resource.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.UpdateResourceRequest,
 *   !proto.catalog.Resource>}
 */
const methodInfo_Resources_Update = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.Resource,
  /**
   * @param {!proto.catalog.UpdateResourceRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Resource.deserializeBinary
);


/**
 * @param {!proto.catalog.UpdateResourceRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.catalog.Resource)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Resource>|undefined}
 *     The XHR Node Readable Stream
 */
proto.catalog.ResourcesClient.prototype.update =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/catalog.Resources/Update',
      request,
      metadata || {},
      methodDescriptor_Resources_Update,
      callback);
};


/**
 * @param {!proto.catalog.UpdateResourceRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.catalog.Resource>}
 *     Promise that resolves to the response
 */
proto.catalog.ResourcesPromiseClient.prototype.update =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/catalog.Resources/Update',
      request,
      metadata || {},
      methodDescriptor_Resources_Update);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.DeleteResourceRequest,
 *   !proto.catalog.Resource>}
 */
const methodDescriptor_Resources_Delete = new grpc.web.MethodDescriptor(
  '/catalog.Resources/Delete',
  grpc.web.MethodType.UNARY,
  proto.catalog.DeleteResourceRequest,
  proto.catalog.Resource,
  /**
   * @param {!proto.catalog.DeleteResourceRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Resource.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.DeleteResourceRequest,
 *   !proto.catalog.Resource>}
 */
const methodInfo_Resources_Delete = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.Resource,
  /**
   * @param {!proto.catalog.DeleteResourceRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Resource.deserializeBinary
);


/**
 * @param {!proto.catalog.DeleteResourceRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.catalog.Resource)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Resource>|undefined}
 *     The XHR Node Readable Stream
 */
proto.catalog.ResourcesClient.prototype.delete =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/catalog.Resources/Delete',
      request,
      metadata || {},
      methodDescriptor_Resources_Delete,
      callback);
};


/**
 * @param {!proto.catalog.DeleteResourceRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.catalog.Resource>}
 *     Promise that resolves to the response
 */
proto.catalog.ResourcesPromiseClient.prototype.delete =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/catalog.Resources/Delete',
      request,
      metadata || {},
      methodDescriptor_Resources_Delete);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.ListResourcesRequest,
 *   !proto.catalog.Resource>}
 */
const methodDescriptor_Resources_List = new grpc.web.MethodDescriptor(
  '/catalog.Resources/List',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.catalog.ListResourcesRequest,
  proto.catalog.Resource,
  /**
   * @param {!proto.catalog.ListResourcesRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Resource.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.ListResourcesRequest,
 *   !proto.catalog.Resource>}
 */
const methodInfo_Resources_List = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.Resource,
  /**
   * @param {!proto.catalog.ListResourcesRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Resource.deserializeBinary
);


/**
 * @param {!proto.catalog.ListResourcesRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Resource>}
 *     The XHR Node Readable Stream
 */
proto.catalog.ResourcesClient.prototype.list =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/catalog.Resources/List',
      request,
      metadata || {},
      methodDescriptor_Resources_List);
};


/**
 * @param {!proto.catalog.ListResourcesRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Resource>}
 *     The XHR Node Readable Stream
 */
proto.catalog.ResourcesPromiseClient.prototype.list =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/catalog.Resources/List',
      request,
      metadata || {},
      methodDescriptor_Resources_List);
};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.catalog.SchemasClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.catalog.SchemasPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.CreateSchemaRequest,
 *   !proto.catalog.Schema>}
 */
const methodDescriptor_Schemas_Create = new grpc.web.MethodDescriptor(
  '/catalog.Schemas/Create',
  grpc.web.MethodType.UNARY,
  proto.catalog.CreateSchemaRequest,
  proto.catalog.Schema,
  /**
   * @param {!proto.catalog.CreateSchemaRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Schema.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.CreateSchemaRequest,
 *   !proto.catalog.Schema>}
 */
const methodInfo_Schemas_Create = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.Schema,
  /**
   * @param {!proto.catalog.CreateSchemaRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Schema.deserializeBinary
);


/**
 * @param {!proto.catalog.CreateSchemaRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.catalog.Schema)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Schema>|undefined}
 *     The XHR Node Readable Stream
 */
proto.catalog.SchemasClient.prototype.create =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/catalog.Schemas/Create',
      request,
      metadata || {},
      methodDescriptor_Schemas_Create,
      callback);
};


/**
 * @param {!proto.catalog.CreateSchemaRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.catalog.Schema>}
 *     Promise that resolves to the response
 */
proto.catalog.SchemasPromiseClient.prototype.create =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/catalog.Schemas/Create',
      request,
      metadata || {},
      methodDescriptor_Schemas_Create);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.GetSchemaRequest,
 *   !proto.catalog.Schema>}
 */
const methodDescriptor_Schemas_Get = new grpc.web.MethodDescriptor(
  '/catalog.Schemas/Get',
  grpc.web.MethodType.UNARY,
  proto.catalog.GetSchemaRequest,
  proto.catalog.Schema,
  /**
   * @param {!proto.catalog.GetSchemaRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Schema.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.GetSchemaRequest,
 *   !proto.catalog.Schema>}
 */
const methodInfo_Schemas_Get = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.Schema,
  /**
   * @param {!proto.catalog.GetSchemaRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Schema.deserializeBinary
);


/**
 * @param {!proto.catalog.GetSchemaRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.catalog.Schema)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Schema>|undefined}
 *     The XHR Node Readable Stream
 */
proto.catalog.SchemasClient.prototype.get =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/catalog.Schemas/Get',
      request,
      metadata || {},
      methodDescriptor_Schemas_Get,
      callback);
};


/**
 * @param {!proto.catalog.GetSchemaRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.catalog.Schema>}
 *     Promise that resolves to the response
 */
proto.catalog.SchemasPromiseClient.prototype.get =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/catalog.Schemas/Get',
      request,
      metadata || {},
      methodDescriptor_Schemas_Get);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.UpdateSchemaRequest,
 *   !proto.catalog.Schema>}
 */
const methodDescriptor_Schemas_Update = new grpc.web.MethodDescriptor(
  '/catalog.Schemas/Update',
  grpc.web.MethodType.UNARY,
  proto.catalog.UpdateSchemaRequest,
  proto.catalog.Schema,
  /**
   * @param {!proto.catalog.UpdateSchemaRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Schema.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.UpdateSchemaRequest,
 *   !proto.catalog.Schema>}
 */
const methodInfo_Schemas_Update = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.Schema,
  /**
   * @param {!proto.catalog.UpdateSchemaRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Schema.deserializeBinary
);


/**
 * @param {!proto.catalog.UpdateSchemaRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.catalog.Schema)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Schema>|undefined}
 *     The XHR Node Readable Stream
 */
proto.catalog.SchemasClient.prototype.update =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/catalog.Schemas/Update',
      request,
      metadata || {},
      methodDescriptor_Schemas_Update,
      callback);
};


/**
 * @param {!proto.catalog.UpdateSchemaRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.catalog.Schema>}
 *     Promise that resolves to the response
 */
proto.catalog.SchemasPromiseClient.prototype.update =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/catalog.Schemas/Update',
      request,
      metadata || {},
      methodDescriptor_Schemas_Update);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.DeleteSchemaRequest,
 *   !proto.catalog.Schema>}
 */
const methodDescriptor_Schemas_Delete = new grpc.web.MethodDescriptor(
  '/catalog.Schemas/Delete',
  grpc.web.MethodType.UNARY,
  proto.catalog.DeleteSchemaRequest,
  proto.catalog.Schema,
  /**
   * @param {!proto.catalog.DeleteSchemaRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Schema.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.DeleteSchemaRequest,
 *   !proto.catalog.Schema>}
 */
const methodInfo_Schemas_Delete = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.Schema,
  /**
   * @param {!proto.catalog.DeleteSchemaRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Schema.deserializeBinary
);


/**
 * @param {!proto.catalog.DeleteSchemaRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.catalog.Schema)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Schema>|undefined}
 *     The XHR Node Readable Stream
 */
proto.catalog.SchemasClient.prototype.delete =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/catalog.Schemas/Delete',
      request,
      metadata || {},
      methodDescriptor_Schemas_Delete,
      callback);
};


/**
 * @param {!proto.catalog.DeleteSchemaRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.catalog.Schema>}
 *     Promise that resolves to the response
 */
proto.catalog.SchemasPromiseClient.prototype.delete =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/catalog.Schemas/Delete',
      request,
      metadata || {},
      methodDescriptor_Schemas_Delete);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.ListSchemasRequest,
 *   !proto.catalog.Schema>}
 */
const methodDescriptor_Schemas_List = new grpc.web.MethodDescriptor(
  '/catalog.Schemas/List',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.catalog.ListSchemasRequest,
  proto.catalog.Schema,
  /**
   * @param {!proto.catalog.ListSchemasRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Schema.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.ListSchemasRequest,
 *   !proto.catalog.Schema>}
 */
const methodInfo_Schemas_List = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.Schema,
  /**
   * @param {!proto.catalog.ListSchemasRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Schema.deserializeBinary
);


/**
 * @param {!proto.catalog.ListSchemasRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Schema>}
 *     The XHR Node Readable Stream
 */
proto.catalog.SchemasClient.prototype.list =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/catalog.Schemas/List',
      request,
      metadata || {},
      methodDescriptor_Schemas_List);
};


/**
 * @param {!proto.catalog.ListSchemasRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Schema>}
 *     The XHR Node Readable Stream
 */
proto.catalog.SchemasPromiseClient.prototype.list =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/catalog.Schemas/List',
      request,
      metadata || {},
      methodDescriptor_Schemas_List);
};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.catalog.PermissionsClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.catalog.PermissionsPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.ShareRequest,
 *   !proto.catalog.PermissionInfo>}
 */
const methodDescriptor_Permissions_Share = new grpc.web.MethodDescriptor(
  '/catalog.Permissions/Share',
  grpc.web.MethodType.UNARY,
  proto.catalog.ShareRequest,
  proto.catalog.PermissionInfo,
  /**
   * @param {!proto.catalog.ShareRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.PermissionInfo.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.ShareRequest,
 *   !proto.catalog.PermissionInfo>}
 */
const methodInfo_Permissions_Share = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.PermissionInfo,
  /**
   * @param {!proto.catalog.ShareRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.PermissionInfo.deserializeBinary
);


/**
 * @param {!proto.catalog.ShareRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.catalog.PermissionInfo)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.PermissionInfo>|undefined}
 *     The XHR Node Readable Stream
 */
proto.catalog.PermissionsClient.prototype.share =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/catalog.Permissions/Share',
      request,
      metadata || {},
      methodDescriptor_Permissions_Share,
      callback);
};


/**
 * @param {!proto.catalog.ShareRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.catalog.PermissionInfo>}
 *     Promise that resolves to the response
 */
proto.catalog.PermissionsPromiseClient.prototype.share =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/catalog.Permissions/Share',
      request,
      metadata || {},
      methodDescriptor_Permissions_Share);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.UnshareRequest,
 *   !proto.catalog.PermissionInfo>}
 */
const methodDescriptor_Permissions_Unshare = new grpc.web.MethodDescriptor(
  '/catalog.Permissions/Unshare',
  grpc.web.MethodType.UNARY,
  proto.catalog.UnshareRequest,
  proto.catalog.PermissionInfo,
  /**
   * @param {!proto.catalog.UnshareRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.PermissionInfo.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.UnshareRequest,
 *   !proto.catalog.PermissionInfo>}
 */
const methodInfo_Permissions_Unshare = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.PermissionInfo,
  /**
   * @param {!proto.catalog.UnshareRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.PermissionInfo.deserializeBinary
);


/**
 * @param {!proto.catalog.UnshareRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.catalog.PermissionInfo)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.PermissionInfo>|undefined}
 *     The XHR Node Readable Stream
 */
proto.catalog.PermissionsClient.prototype.unshare =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/catalog.Permissions/Unshare',
      request,
      metadata || {},
      methodDescriptor_Permissions_Unshare,
      callback);
};


/**
 * @param {!proto.catalog.UnshareRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.catalog.PermissionInfo>}
 *     Promise that resolves to the response
 */
proto.catalog.PermissionsPromiseClient.prototype.unshare =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/catalog.Permissions/Unshare',
      request,
      metadata || {},
      methodDescriptor_Permissions_Unshare);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.GetPermissionInfoRequest,
 *   !proto.catalog.PermissionInfo>}
 */
const methodDescriptor_Permissions_Get = new grpc.web.MethodDescriptor(
  '/catalog.Permissions/Get',
  grpc.web.MethodType.UNARY,
  proto.catalog.GetPermissionInfoRequest,
  proto.catalog.PermissionInfo,
  /**
   * @param {!proto.catalog.GetPermissionInfoRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.PermissionInfo.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.GetPermissionInfoRequest,
 *   !proto.catalog.PermissionInfo>}
 */
const methodInfo_Permissions_Get = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.PermissionInfo,
  /**
   * @param {!proto.catalog.GetPermissionInfoRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.PermissionInfo.deserializeBinary
);


/**
 * @param {!proto.catalog.GetPermissionInfoRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.catalog.PermissionInfo)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.PermissionInfo>|undefined}
 *     The XHR Node Readable Stream
 */
proto.catalog.PermissionsClient.prototype.get =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/catalog.Permissions/Get',
      request,
      metadata || {},
      methodDescriptor_Permissions_Get,
      callback);
};


/**
 * @param {!proto.catalog.GetPermissionInfoRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.catalog.PermissionInfo>}
 *     Promise that resolves to the response
 */
proto.catalog.PermissionsPromiseClient.prototype.get =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/catalog.Permissions/Get',
      request,
      metadata || {},
      methodDescriptor_Permissions_Get);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.ListPermissionsRequest,
 *   !proto.catalog.PermissionInfo>}
 */
const methodDescriptor_Permissions_List = new grpc.web.MethodDescriptor(
  '/catalog.Permissions/List',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.catalog.ListPermissionsRequest,
  proto.catalog.PermissionInfo,
  /**
   * @param {!proto.catalog.ListPermissionsRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.PermissionInfo.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.ListPermissionsRequest,
 *   !proto.catalog.PermissionInfo>}
 */
const methodInfo_Permissions_List = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.PermissionInfo,
  /**
   * @param {!proto.catalog.ListPermissionsRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.PermissionInfo.deserializeBinary
);


/**
 * @param {!proto.catalog.ListPermissionsRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.PermissionInfo>}
 *     The XHR Node Readable Stream
 */
proto.catalog.PermissionsClient.prototype.list =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/catalog.Permissions/List',
      request,
      metadata || {},
      methodDescriptor_Permissions_List);
};


/**
 * @param {!proto.catalog.ListPermissionsRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.PermissionInfo>}
 *     The XHR Node Readable Stream
 */
proto.catalog.PermissionsPromiseClient.prototype.list =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/catalog.Permissions/List',
      request,
      metadata || {},
      methodDescriptor_Permissions_List);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.PermissionCheckRequest,
 *   !proto.catalog.PermissionCheckResponse>}
 */
const methodDescriptor_Permissions_Check = new grpc.web.MethodDescriptor(
  '/catalog.Permissions/Check',
  grpc.web.MethodType.UNARY,
  proto.catalog.PermissionCheckRequest,
  proto.catalog.PermissionCheckResponse,
  /**
   * @param {!proto.catalog.PermissionCheckRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.PermissionCheckResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.PermissionCheckRequest,
 *   !proto.catalog.PermissionCheckResponse>}
 */
const methodInfo_Permissions_Check = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.PermissionCheckResponse,
  /**
   * @param {!proto.catalog.PermissionCheckRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.PermissionCheckResponse.deserializeBinary
);


/**
 * @param {!proto.catalog.PermissionCheckRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.catalog.PermissionCheckResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.PermissionCheckResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.catalog.PermissionsClient.prototype.check =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/catalog.Permissions/Check',
      request,
      metadata || {},
      methodDescriptor_Permissions_Check,
      callback);
};


/**
 * @param {!proto.catalog.PermissionCheckRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.catalog.PermissionCheckResponse>}
 *     Promise that resolves to the response
 */
proto.catalog.PermissionsPromiseClient.prototype.check =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/catalog.Permissions/Check',
      request,
      metadata || {},
      methodDescriptor_Permissions_Check);
};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.catalog.EventsClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.catalog.EventsPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.PublishRequest,
 *   !proto.catalog.Event>}
 */
const methodDescriptor_Events_Publish = new grpc.web.MethodDescriptor(
  '/catalog.Events/Publish',
  grpc.web.MethodType.UNARY,
  proto.catalog.PublishRequest,
  proto.catalog.Event,
  /**
   * @param {!proto.catalog.PublishRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Event.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.PublishRequest,
 *   !proto.catalog.Event>}
 */
const methodInfo_Events_Publish = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.Event,
  /**
   * @param {!proto.catalog.PublishRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Event.deserializeBinary
);


/**
 * @param {!proto.catalog.PublishRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.catalog.Event)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Event>|undefined}
 *     The XHR Node Readable Stream
 */
proto.catalog.EventsClient.prototype.publish =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/catalog.Events/Publish',
      request,
      metadata || {},
      methodDescriptor_Events_Publish,
      callback);
};


/**
 * @param {!proto.catalog.PublishRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.catalog.Event>}
 *     Promise that resolves to the response
 */
proto.catalog.EventsPromiseClient.prototype.publish =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/catalog.Events/Publish',
      request,
      metadata || {},
      methodDescriptor_Events_Publish);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.SubscribeRequest,
 *   !proto.catalog.Event>}
 */
const methodDescriptor_Events_Subscribe = new grpc.web.MethodDescriptor(
  '/catalog.Events/Subscribe',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.catalog.SubscribeRequest,
  proto.catalog.Event,
  /**
   * @param {!proto.catalog.SubscribeRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Event.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.SubscribeRequest,
 *   !proto.catalog.Event>}
 */
const methodInfo_Events_Subscribe = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.Event,
  /**
   * @param {!proto.catalog.SubscribeRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.Event.deserializeBinary
);


/**
 * @param {!proto.catalog.SubscribeRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Event>}
 *     The XHR Node Readable Stream
 */
proto.catalog.EventsClient.prototype.subscribe =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/catalog.Events/Subscribe',
      request,
      metadata || {},
      methodDescriptor_Events_Subscribe);
};


/**
 * @param {!proto.catalog.SubscribeRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.Event>}
 *     The XHR Node Readable Stream
 */
proto.catalog.EventsPromiseClient.prototype.subscribe =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/catalog.Events/Subscribe',
      request,
      metadata || {},
      methodDescriptor_Events_Subscribe);
};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.catalog.LocksClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.catalog.LocksPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.LockRequest,
 *   !proto.catalog.LockResponse>}
 */
const methodDescriptor_Locks_Lock = new grpc.web.MethodDescriptor(
  '/catalog.Locks/Lock',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.catalog.LockRequest,
  proto.catalog.LockResponse,
  /**
   * @param {!proto.catalog.LockRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.LockResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.LockRequest,
 *   !proto.catalog.LockResponse>}
 */
const methodInfo_Locks_Lock = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.LockResponse,
  /**
   * @param {!proto.catalog.LockRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.LockResponse.deserializeBinary
);


/**
 * @param {!proto.catalog.LockRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.LockResponse>}
 *     The XHR Node Readable Stream
 */
proto.catalog.LocksClient.prototype.lock =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/catalog.Locks/Lock',
      request,
      metadata || {},
      methodDescriptor_Locks_Lock);
};


/**
 * @param {!proto.catalog.LockRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.LockResponse>}
 *     The XHR Node Readable Stream
 */
proto.catalog.LocksPromiseClient.prototype.lock =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/catalog.Locks/Lock',
      request,
      metadata || {},
      methodDescriptor_Locks_Lock);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.catalog.LockRequest,
 *   !proto.catalog.LockResponse>}
 */
const methodDescriptor_Locks_TryLock = new grpc.web.MethodDescriptor(
  '/catalog.Locks/TryLock',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.catalog.LockRequest,
  proto.catalog.LockResponse,
  /**
   * @param {!proto.catalog.LockRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.LockResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.catalog.LockRequest,
 *   !proto.catalog.LockResponse>}
 */
const methodInfo_Locks_TryLock = new grpc.web.AbstractClientBase.MethodInfo(
  proto.catalog.LockResponse,
  /**
   * @param {!proto.catalog.LockRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.catalog.LockResponse.deserializeBinary
);


/**
 * @param {!proto.catalog.LockRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.LockResponse>}
 *     The XHR Node Readable Stream
 */
proto.catalog.LocksClient.prototype.tryLock =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/catalog.Locks/TryLock',
      request,
      metadata || {},
      methodDescriptor_Locks_TryLock);
};


/**
 * @param {!proto.catalog.LockRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.catalog.LockResponse>}
 *     The XHR Node Readable Stream
 */
proto.catalog.LocksPromiseClient.prototype.tryLock =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/catalog.Locks/TryLock',
      request,
      metadata || {},
      methodDescriptor_Locks_TryLock);
};


module.exports = proto.catalog;

