import * as grpcWeb from 'grpc-web';

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';
import * as idp_pb from './idp_pb';


export class UsersClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: idp_pb.CreateUserRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: idp_pb.User) => void
  ): grpcWeb.ClientReadableStream<idp_pb.User>;

  get(
    request: idp_pb.GetUserRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: idp_pb.User) => void
  ): grpcWeb.ClientReadableStream<idp_pb.User>;

  update(
    request: idp_pb.UpdateUserRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: idp_pb.User) => void
  ): grpcWeb.ClientReadableStream<idp_pb.User>;

  delete(
    request: idp_pb.DeleteUserRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: idp_pb.User) => void
  ): grpcWeb.ClientReadableStream<idp_pb.User>;

  list(
    request: idp_pb.ListUsersRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<idp_pb.User>;

}

export class ServiceAccountsClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: idp_pb.CreateServiceAccountRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: idp_pb.CreateServiceAccountResponse) => void
  ): grpcWeb.ClientReadableStream<idp_pb.CreateServiceAccountResponse>;

  get(
    request: idp_pb.GetServiceAccountRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: idp_pb.ServiceAccount) => void
  ): grpcWeb.ClientReadableStream<idp_pb.ServiceAccount>;

  update(
    request: idp_pb.UpdateServiceAccountRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: idp_pb.UpdateServiceAccountResponse) => void
  ): grpcWeb.ClientReadableStream<idp_pb.UpdateServiceAccountResponse>;

  delete(
    request: idp_pb.DeleteServiceAccountRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: idp_pb.ServiceAccount) => void
  ): grpcWeb.ClientReadableStream<idp_pb.ServiceAccount>;

  list(
    request: idp_pb.ListServiceAccountsRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<idp_pb.ServiceAccount>;

}

export class AuthenticationClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  login(
    request: idp_pb.LoginRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: idp_pb.LoginResponse) => void
  ): grpcWeb.ClientReadableStream<idp_pb.LoginResponse>;

  refresh(
    request: idp_pb.RefreshRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: idp_pb.LoginResponse) => void
  ): grpcWeb.ClientReadableStream<idp_pb.LoginResponse>;

}

export class GroupsClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: idp_pb.CreateGroupRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: idp_pb.Group) => void
  ): grpcWeb.ClientReadableStream<idp_pb.Group>;

  get(
    request: idp_pb.GetGroupRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: idp_pb.Group) => void
  ): grpcWeb.ClientReadableStream<idp_pb.Group>;

  update(
    request: idp_pb.UpdateGroupRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: idp_pb.Group) => void
  ): grpcWeb.ClientReadableStream<idp_pb.Group>;

  delete(
    request: idp_pb.DeleteGroupRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: idp_pb.Group) => void
  ): grpcWeb.ClientReadableStream<idp_pb.Group>;

  list(
    request: idp_pb.ListGroupsRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<idp_pb.Group>;

  addUser(
    request: idp_pb.AddUserToGroupRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  delUser(
    request: idp_pb.DelUserFromGroupRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  listMembers(
    request: idp_pb.ListGroupMembersRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<idp_pb.GroupMember>;

}

export class UsersPromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: idp_pb.CreateUserRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<idp_pb.User>;

  get(
    request: idp_pb.GetUserRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<idp_pb.User>;

  update(
    request: idp_pb.UpdateUserRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<idp_pb.User>;

  delete(
    request: idp_pb.DeleteUserRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<idp_pb.User>;

  list(
    request: idp_pb.ListUsersRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<idp_pb.User>;

}

export class ServiceAccountsPromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: idp_pb.CreateServiceAccountRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<idp_pb.CreateServiceAccountResponse>;

  get(
    request: idp_pb.GetServiceAccountRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<idp_pb.ServiceAccount>;

  update(
    request: idp_pb.UpdateServiceAccountRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<idp_pb.UpdateServiceAccountResponse>;

  delete(
    request: idp_pb.DeleteServiceAccountRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<idp_pb.ServiceAccount>;

  list(
    request: idp_pb.ListServiceAccountsRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<idp_pb.ServiceAccount>;

}

export class AuthenticationPromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  login(
    request: idp_pb.LoginRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<idp_pb.LoginResponse>;

  refresh(
    request: idp_pb.RefreshRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<idp_pb.LoginResponse>;

}

export class GroupsPromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: idp_pb.CreateGroupRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<idp_pb.Group>;

  get(
    request: idp_pb.GetGroupRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<idp_pb.Group>;

  update(
    request: idp_pb.UpdateGroupRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<idp_pb.Group>;

  delete(
    request: idp_pb.DeleteGroupRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<idp_pb.Group>;

  list(
    request: idp_pb.ListGroupsRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<idp_pb.Group>;

  addUser(
    request: idp_pb.AddUserToGroupRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  delUser(
    request: idp_pb.DelUserFromGroupRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  listMembers(
    request: idp_pb.ListGroupMembersRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<idp_pb.GroupMember>;

}

