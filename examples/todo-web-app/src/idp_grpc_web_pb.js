/**
 * @fileoverview gRPC-Web generated client stub for idp
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');


var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js')

var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js')
const proto = {};
proto.idp = require('./idp_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.idp.UsersClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

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
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.idp.UsersPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

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
 *   !proto.idp.CreateUserRequest,
 *   !proto.idp.User>}
 */
const methodDescriptor_Users_Create = new grpc.web.MethodDescriptor(
  '/idp.Users/Create',
  grpc.web.MethodType.UNARY,
  proto.idp.CreateUserRequest,
  proto.idp.User,
  /**
   * @param {!proto.idp.CreateUserRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.User.deserializeBinary
);


/**
 * @param {!proto.idp.CreateUserRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.idp.User)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.idp.User>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.UsersClient.prototype.create =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.Users/Create',
      request,
      metadata || {},
      methodDescriptor_Users_Create,
      callback);
};


/**
 * @param {!proto.idp.CreateUserRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.idp.User>}
 *     Promise that resolves to the response
 */
proto.idp.UsersPromiseClient.prototype.create =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.Users/Create',
      request,
      metadata || {},
      methodDescriptor_Users_Create);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.GetUserRequest,
 *   !proto.idp.User>}
 */
const methodDescriptor_Users_Get = new grpc.web.MethodDescriptor(
  '/idp.Users/Get',
  grpc.web.MethodType.UNARY,
  proto.idp.GetUserRequest,
  proto.idp.User,
  /**
   * @param {!proto.idp.GetUserRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.User.deserializeBinary
);


/**
 * @param {!proto.idp.GetUserRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.idp.User)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.idp.User>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.UsersClient.prototype.get =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.Users/Get',
      request,
      metadata || {},
      methodDescriptor_Users_Get,
      callback);
};


/**
 * @param {!proto.idp.GetUserRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.idp.User>}
 *     Promise that resolves to the response
 */
proto.idp.UsersPromiseClient.prototype.get =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.Users/Get',
      request,
      metadata || {},
      methodDescriptor_Users_Get);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.UpdateUserRequest,
 *   !proto.idp.User>}
 */
const methodDescriptor_Users_Update = new grpc.web.MethodDescriptor(
  '/idp.Users/Update',
  grpc.web.MethodType.UNARY,
  proto.idp.UpdateUserRequest,
  proto.idp.User,
  /**
   * @param {!proto.idp.UpdateUserRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.User.deserializeBinary
);


/**
 * @param {!proto.idp.UpdateUserRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.idp.User)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.idp.User>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.UsersClient.prototype.update =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.Users/Update',
      request,
      metadata || {},
      methodDescriptor_Users_Update,
      callback);
};


/**
 * @param {!proto.idp.UpdateUserRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.idp.User>}
 *     Promise that resolves to the response
 */
proto.idp.UsersPromiseClient.prototype.update =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.Users/Update',
      request,
      metadata || {},
      methodDescriptor_Users_Update);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.DeleteUserRequest,
 *   !proto.idp.User>}
 */
const methodDescriptor_Users_Delete = new grpc.web.MethodDescriptor(
  '/idp.Users/Delete',
  grpc.web.MethodType.UNARY,
  proto.idp.DeleteUserRequest,
  proto.idp.User,
  /**
   * @param {!proto.idp.DeleteUserRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.User.deserializeBinary
);


/**
 * @param {!proto.idp.DeleteUserRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.idp.User)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.idp.User>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.UsersClient.prototype.delete =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.Users/Delete',
      request,
      metadata || {},
      methodDescriptor_Users_Delete,
      callback);
};


/**
 * @param {!proto.idp.DeleteUserRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.idp.User>}
 *     Promise that resolves to the response
 */
proto.idp.UsersPromiseClient.prototype.delete =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.Users/Delete',
      request,
      metadata || {},
      methodDescriptor_Users_Delete);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.ListUsersRequest,
 *   !proto.idp.User>}
 */
const methodDescriptor_Users_List = new grpc.web.MethodDescriptor(
  '/idp.Users/List',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.idp.ListUsersRequest,
  proto.idp.User,
  /**
   * @param {!proto.idp.ListUsersRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.User.deserializeBinary
);


/**
 * @param {!proto.idp.ListUsersRequest} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.idp.User>}
 *     The XHR Node Readable Stream
 */
proto.idp.UsersClient.prototype.list =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/idp.Users/List',
      request,
      metadata || {},
      methodDescriptor_Users_List);
};


/**
 * @param {!proto.idp.ListUsersRequest} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.idp.User>}
 *     The XHR Node Readable Stream
 */
proto.idp.UsersPromiseClient.prototype.list =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/idp.Users/List',
      request,
      metadata || {},
      methodDescriptor_Users_List);
};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.idp.ServiceAccountsClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

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
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.idp.ServiceAccountsPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

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
 *   !proto.idp.CreateServiceAccountRequest,
 *   !proto.idp.CreateServiceAccountResponse>}
 */
const methodDescriptor_ServiceAccounts_Create = new grpc.web.MethodDescriptor(
  '/idp.ServiceAccounts/Create',
  grpc.web.MethodType.UNARY,
  proto.idp.CreateServiceAccountRequest,
  proto.idp.CreateServiceAccountResponse,
  /**
   * @param {!proto.idp.CreateServiceAccountRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.CreateServiceAccountResponse.deserializeBinary
);


/**
 * @param {!proto.idp.CreateServiceAccountRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.idp.CreateServiceAccountResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.idp.CreateServiceAccountResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.ServiceAccountsClient.prototype.create =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.ServiceAccounts/Create',
      request,
      metadata || {},
      methodDescriptor_ServiceAccounts_Create,
      callback);
};


/**
 * @param {!proto.idp.CreateServiceAccountRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.idp.CreateServiceAccountResponse>}
 *     Promise that resolves to the response
 */
proto.idp.ServiceAccountsPromiseClient.prototype.create =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.ServiceAccounts/Create',
      request,
      metadata || {},
      methodDescriptor_ServiceAccounts_Create);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.GetServiceAccountRequest,
 *   !proto.idp.ServiceAccount>}
 */
const methodDescriptor_ServiceAccounts_Get = new grpc.web.MethodDescriptor(
  '/idp.ServiceAccounts/Get',
  grpc.web.MethodType.UNARY,
  proto.idp.GetServiceAccountRequest,
  proto.idp.ServiceAccount,
  /**
   * @param {!proto.idp.GetServiceAccountRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.ServiceAccount.deserializeBinary
);


/**
 * @param {!proto.idp.GetServiceAccountRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.idp.ServiceAccount)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.idp.ServiceAccount>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.ServiceAccountsClient.prototype.get =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.ServiceAccounts/Get',
      request,
      metadata || {},
      methodDescriptor_ServiceAccounts_Get,
      callback);
};


/**
 * @param {!proto.idp.GetServiceAccountRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.idp.ServiceAccount>}
 *     Promise that resolves to the response
 */
proto.idp.ServiceAccountsPromiseClient.prototype.get =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.ServiceAccounts/Get',
      request,
      metadata || {},
      methodDescriptor_ServiceAccounts_Get);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.UpdateServiceAccountRequest,
 *   !proto.idp.UpdateServiceAccountResponse>}
 */
const methodDescriptor_ServiceAccounts_Update = new grpc.web.MethodDescriptor(
  '/idp.ServiceAccounts/Update',
  grpc.web.MethodType.UNARY,
  proto.idp.UpdateServiceAccountRequest,
  proto.idp.UpdateServiceAccountResponse,
  /**
   * @param {!proto.idp.UpdateServiceAccountRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.UpdateServiceAccountResponse.deserializeBinary
);


/**
 * @param {!proto.idp.UpdateServiceAccountRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.idp.UpdateServiceAccountResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.idp.UpdateServiceAccountResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.ServiceAccountsClient.prototype.update =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.ServiceAccounts/Update',
      request,
      metadata || {},
      methodDescriptor_ServiceAccounts_Update,
      callback);
};


/**
 * @param {!proto.idp.UpdateServiceAccountRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.idp.UpdateServiceAccountResponse>}
 *     Promise that resolves to the response
 */
proto.idp.ServiceAccountsPromiseClient.prototype.update =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.ServiceAccounts/Update',
      request,
      metadata || {},
      methodDescriptor_ServiceAccounts_Update);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.DeleteServiceAccountRequest,
 *   !proto.idp.ServiceAccount>}
 */
const methodDescriptor_ServiceAccounts_Delete = new grpc.web.MethodDescriptor(
  '/idp.ServiceAccounts/Delete',
  grpc.web.MethodType.UNARY,
  proto.idp.DeleteServiceAccountRequest,
  proto.idp.ServiceAccount,
  /**
   * @param {!proto.idp.DeleteServiceAccountRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.ServiceAccount.deserializeBinary
);


/**
 * @param {!proto.idp.DeleteServiceAccountRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.idp.ServiceAccount)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.idp.ServiceAccount>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.ServiceAccountsClient.prototype.delete =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.ServiceAccounts/Delete',
      request,
      metadata || {},
      methodDescriptor_ServiceAccounts_Delete,
      callback);
};


/**
 * @param {!proto.idp.DeleteServiceAccountRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.idp.ServiceAccount>}
 *     Promise that resolves to the response
 */
proto.idp.ServiceAccountsPromiseClient.prototype.delete =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.ServiceAccounts/Delete',
      request,
      metadata || {},
      methodDescriptor_ServiceAccounts_Delete);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.ListServiceAccountsRequest,
 *   !proto.idp.ServiceAccount>}
 */
const methodDescriptor_ServiceAccounts_List = new grpc.web.MethodDescriptor(
  '/idp.ServiceAccounts/List',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.idp.ListServiceAccountsRequest,
  proto.idp.ServiceAccount,
  /**
   * @param {!proto.idp.ListServiceAccountsRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.ServiceAccount.deserializeBinary
);


/**
 * @param {!proto.idp.ListServiceAccountsRequest} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.idp.ServiceAccount>}
 *     The XHR Node Readable Stream
 */
proto.idp.ServiceAccountsClient.prototype.list =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/idp.ServiceAccounts/List',
      request,
      metadata || {},
      methodDescriptor_ServiceAccounts_List);
};


/**
 * @param {!proto.idp.ListServiceAccountsRequest} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.idp.ServiceAccount>}
 *     The XHR Node Readable Stream
 */
proto.idp.ServiceAccountsPromiseClient.prototype.list =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/idp.ServiceAccounts/List',
      request,
      metadata || {},
      methodDescriptor_ServiceAccounts_List);
};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.idp.AuthenticationClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

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
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.idp.AuthenticationPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

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
 *   !proto.idp.LoginRequest,
 *   !proto.idp.LoginResponse>}
 */
const methodDescriptor_Authentication_Login = new grpc.web.MethodDescriptor(
  '/idp.Authentication/Login',
  grpc.web.MethodType.UNARY,
  proto.idp.LoginRequest,
  proto.idp.LoginResponse,
  /**
   * @param {!proto.idp.LoginRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.LoginResponse.deserializeBinary
);


/**
 * @param {!proto.idp.LoginRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.idp.LoginResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.idp.LoginResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.AuthenticationClient.prototype.login =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.Authentication/Login',
      request,
      metadata || {},
      methodDescriptor_Authentication_Login,
      callback);
};


/**
 * @param {!proto.idp.LoginRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.idp.LoginResponse>}
 *     Promise that resolves to the response
 */
proto.idp.AuthenticationPromiseClient.prototype.login =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.Authentication/Login',
      request,
      metadata || {},
      methodDescriptor_Authentication_Login);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.RefreshRequest,
 *   !proto.idp.LoginResponse>}
 */
const methodDescriptor_Authentication_Refresh = new grpc.web.MethodDescriptor(
  '/idp.Authentication/Refresh',
  grpc.web.MethodType.UNARY,
  proto.idp.RefreshRequest,
  proto.idp.LoginResponse,
  /**
   * @param {!proto.idp.RefreshRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.LoginResponse.deserializeBinary
);


/**
 * @param {!proto.idp.RefreshRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.idp.LoginResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.idp.LoginResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.AuthenticationClient.prototype.refresh =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.Authentication/Refresh',
      request,
      metadata || {},
      methodDescriptor_Authentication_Refresh,
      callback);
};


/**
 * @param {!proto.idp.RefreshRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.idp.LoginResponse>}
 *     Promise that resolves to the response
 */
proto.idp.AuthenticationPromiseClient.prototype.refresh =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.Authentication/Refresh',
      request,
      metadata || {},
      methodDescriptor_Authentication_Refresh);
};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.idp.GroupsClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

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
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.idp.GroupsPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

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
 *   !proto.idp.CreateGroupRequest,
 *   !proto.idp.Group>}
 */
const methodDescriptor_Groups_Create = new grpc.web.MethodDescriptor(
  '/idp.Groups/Create',
  grpc.web.MethodType.UNARY,
  proto.idp.CreateGroupRequest,
  proto.idp.Group,
  /**
   * @param {!proto.idp.CreateGroupRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.Group.deserializeBinary
);


/**
 * @param {!proto.idp.CreateGroupRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.idp.Group)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.idp.Group>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.GroupsClient.prototype.create =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.Groups/Create',
      request,
      metadata || {},
      methodDescriptor_Groups_Create,
      callback);
};


/**
 * @param {!proto.idp.CreateGroupRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.idp.Group>}
 *     Promise that resolves to the response
 */
proto.idp.GroupsPromiseClient.prototype.create =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.Groups/Create',
      request,
      metadata || {},
      methodDescriptor_Groups_Create);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.GetGroupRequest,
 *   !proto.idp.Group>}
 */
const methodDescriptor_Groups_Get = new grpc.web.MethodDescriptor(
  '/idp.Groups/Get',
  grpc.web.MethodType.UNARY,
  proto.idp.GetGroupRequest,
  proto.idp.Group,
  /**
   * @param {!proto.idp.GetGroupRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.Group.deserializeBinary
);


/**
 * @param {!proto.idp.GetGroupRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.idp.Group)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.idp.Group>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.GroupsClient.prototype.get =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.Groups/Get',
      request,
      metadata || {},
      methodDescriptor_Groups_Get,
      callback);
};


/**
 * @param {!proto.idp.GetGroupRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.idp.Group>}
 *     Promise that resolves to the response
 */
proto.idp.GroupsPromiseClient.prototype.get =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.Groups/Get',
      request,
      metadata || {},
      methodDescriptor_Groups_Get);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.UpdateGroupRequest,
 *   !proto.idp.Group>}
 */
const methodDescriptor_Groups_Update = new grpc.web.MethodDescriptor(
  '/idp.Groups/Update',
  grpc.web.MethodType.UNARY,
  proto.idp.UpdateGroupRequest,
  proto.idp.Group,
  /**
   * @param {!proto.idp.UpdateGroupRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.Group.deserializeBinary
);


/**
 * @param {!proto.idp.UpdateGroupRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.idp.Group)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.idp.Group>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.GroupsClient.prototype.update =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.Groups/Update',
      request,
      metadata || {},
      methodDescriptor_Groups_Update,
      callback);
};


/**
 * @param {!proto.idp.UpdateGroupRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.idp.Group>}
 *     Promise that resolves to the response
 */
proto.idp.GroupsPromiseClient.prototype.update =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.Groups/Update',
      request,
      metadata || {},
      methodDescriptor_Groups_Update);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.DeleteGroupRequest,
 *   !proto.idp.Group>}
 */
const methodDescriptor_Groups_Delete = new grpc.web.MethodDescriptor(
  '/idp.Groups/Delete',
  grpc.web.MethodType.UNARY,
  proto.idp.DeleteGroupRequest,
  proto.idp.Group,
  /**
   * @param {!proto.idp.DeleteGroupRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.Group.deserializeBinary
);


/**
 * @param {!proto.idp.DeleteGroupRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.idp.Group)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.idp.Group>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.GroupsClient.prototype.delete =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.Groups/Delete',
      request,
      metadata || {},
      methodDescriptor_Groups_Delete,
      callback);
};


/**
 * @param {!proto.idp.DeleteGroupRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.idp.Group>}
 *     Promise that resolves to the response
 */
proto.idp.GroupsPromiseClient.prototype.delete =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.Groups/Delete',
      request,
      metadata || {},
      methodDescriptor_Groups_Delete);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.ListGroupsRequest,
 *   !proto.idp.Group>}
 */
const methodDescriptor_Groups_List = new grpc.web.MethodDescriptor(
  '/idp.Groups/List',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.idp.ListGroupsRequest,
  proto.idp.Group,
  /**
   * @param {!proto.idp.ListGroupsRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.Group.deserializeBinary
);


/**
 * @param {!proto.idp.ListGroupsRequest} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.idp.Group>}
 *     The XHR Node Readable Stream
 */
proto.idp.GroupsClient.prototype.list =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/idp.Groups/List',
      request,
      metadata || {},
      methodDescriptor_Groups_List);
};


/**
 * @param {!proto.idp.ListGroupsRequest} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.idp.Group>}
 *     The XHR Node Readable Stream
 */
proto.idp.GroupsPromiseClient.prototype.list =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/idp.Groups/List',
      request,
      metadata || {},
      methodDescriptor_Groups_List);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.AddUserToGroupRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodDescriptor_Groups_AddUser = new grpc.web.MethodDescriptor(
  '/idp.Groups/AddUser',
  grpc.web.MethodType.UNARY,
  proto.idp.AddUserToGroupRequest,
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.idp.AddUserToGroupRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @param {!proto.idp.AddUserToGroupRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.google.protobuf.Empty)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.google.protobuf.Empty>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.GroupsClient.prototype.addUser =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.Groups/AddUser',
      request,
      metadata || {},
      methodDescriptor_Groups_AddUser,
      callback);
};


/**
 * @param {!proto.idp.AddUserToGroupRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.google.protobuf.Empty>}
 *     Promise that resolves to the response
 */
proto.idp.GroupsPromiseClient.prototype.addUser =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.Groups/AddUser',
      request,
      metadata || {},
      methodDescriptor_Groups_AddUser);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.DelUserFromGroupRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodDescriptor_Groups_DelUser = new grpc.web.MethodDescriptor(
  '/idp.Groups/DelUser',
  grpc.web.MethodType.UNARY,
  proto.idp.DelUserFromGroupRequest,
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.idp.DelUserFromGroupRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @param {!proto.idp.DelUserFromGroupRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.google.protobuf.Empty)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.google.protobuf.Empty>|undefined}
 *     The XHR Node Readable Stream
 */
proto.idp.GroupsClient.prototype.delUser =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/idp.Groups/DelUser',
      request,
      metadata || {},
      methodDescriptor_Groups_DelUser,
      callback);
};


/**
 * @param {!proto.idp.DelUserFromGroupRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.google.protobuf.Empty>}
 *     Promise that resolves to the response
 */
proto.idp.GroupsPromiseClient.prototype.delUser =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/idp.Groups/DelUser',
      request,
      metadata || {},
      methodDescriptor_Groups_DelUser);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.idp.ListGroupMembersRequest,
 *   !proto.idp.GroupMember>}
 */
const methodDescriptor_Groups_ListMembers = new grpc.web.MethodDescriptor(
  '/idp.Groups/ListMembers',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.idp.ListGroupMembersRequest,
  proto.idp.GroupMember,
  /**
   * @param {!proto.idp.ListGroupMembersRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.idp.GroupMember.deserializeBinary
);


/**
 * @param {!proto.idp.ListGroupMembersRequest} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.idp.GroupMember>}
 *     The XHR Node Readable Stream
 */
proto.idp.GroupsClient.prototype.listMembers =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/idp.Groups/ListMembers',
      request,
      metadata || {},
      methodDescriptor_Groups_ListMembers);
};


/**
 * @param {!proto.idp.ListGroupMembersRequest} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.idp.GroupMember>}
 *     The XHR Node Readable Stream
 */
proto.idp.GroupsPromiseClient.prototype.listMembers =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/idp.Groups/ListMembers',
      request,
      metadata || {},
      methodDescriptor_Groups_ListMembers);
};


module.exports = proto.idp;

