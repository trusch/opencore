import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class User extends jspb.Message {
  getId(): string;
  setId(value: string): User;

  getName(): string;
  setName(value: string): User;

  getExternalId(): string;
  setExternalId(value: string): User;

  getIsAdmin(): boolean;
  setIsAdmin(value: boolean): User;

  getPasswordHash(): string;
  setPasswordHash(value: string): User;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): User;
  hasCreatedAt(): boolean;
  clearCreatedAt(): User;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): User;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): User;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): User.AsObject;
  static toObject(includeInstance: boolean, msg: User): User.AsObject;
  static serializeBinaryToWriter(message: User, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): User;
  static deserializeBinaryFromReader(message: User, reader: jspb.BinaryReader): User;
}

export namespace User {
  export type AsObject = {
    id: string,
    name: string,
    externalId: string,
    isAdmin: boolean,
    passwordHash: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class CreateUserRequest extends jspb.Message {
  getName(): string;
  setName(value: string): CreateUserRequest;

  getExternalId(): string;
  setExternalId(value: string): CreateUserRequest;

  getPassword(): string;
  setPassword(value: string): CreateUserRequest;

  getIsAdmin(): boolean;
  setIsAdmin(value: boolean): CreateUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateUserRequest): CreateUserRequest.AsObject;
  static serializeBinaryToWriter(message: CreateUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateUserRequest;
  static deserializeBinaryFromReader(message: CreateUserRequest, reader: jspb.BinaryReader): CreateUserRequest;
}

export namespace CreateUserRequest {
  export type AsObject = {
    name: string,
    externalId: string,
    password: string,
    isAdmin: boolean,
  }
}

export class GetUserRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetUserRequest;

  getExternalId(): string;
  setExternalId(value: string): GetUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetUserRequest): GetUserRequest.AsObject;
  static serializeBinaryToWriter(message: GetUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetUserRequest;
  static deserializeBinaryFromReader(message: GetUserRequest, reader: jspb.BinaryReader): GetUserRequest;
}

export namespace GetUserRequest {
  export type AsObject = {
    id: string,
    externalId: string,
  }
}

export class DeleteUserRequest extends jspb.Message {
  getId(): string;
  setId(value: string): DeleteUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteUserRequest): DeleteUserRequest.AsObject;
  static serializeBinaryToWriter(message: DeleteUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteUserRequest;
  static deserializeBinaryFromReader(message: DeleteUserRequest, reader: jspb.BinaryReader): DeleteUserRequest;
}

export namespace DeleteUserRequest {
  export type AsObject = {
    id: string,
  }
}

export class UpdateUserRequest extends jspb.Message {
  getId(): string;
  setId(value: string): UpdateUserRequest;

  getName(): string;
  setName(value: string): UpdateUserRequest;

  getExternalId(): string;
  setExternalId(value: string): UpdateUserRequest;

  getPassword(): string;
  setPassword(value: string): UpdateUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateUserRequest): UpdateUserRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateUserRequest;
  static deserializeBinaryFromReader(message: UpdateUserRequest, reader: jspb.BinaryReader): UpdateUserRequest;
}

export namespace UpdateUserRequest {
  export type AsObject = {
    id: string,
    name: string,
    externalId: string,
    password: string,
  }
}

export class ListUsersRequest extends jspb.Message {
  getFilter(): string;
  setFilter(value: string): ListUsersRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListUsersRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListUsersRequest): ListUsersRequest.AsObject;
  static serializeBinaryToWriter(message: ListUsersRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListUsersRequest;
  static deserializeBinaryFromReader(message: ListUsersRequest, reader: jspb.BinaryReader): ListUsersRequest;
}

export namespace ListUsersRequest {
  export type AsObject = {
    filter: string,
  }
}

export class ServiceAccount extends jspb.Message {
  getId(): string;
  setId(value: string): ServiceAccount;

  getName(): string;
  setName(value: string): ServiceAccount;

  getSecretKeyHash(): string;
  setSecretKeyHash(value: string): ServiceAccount;

  getIsAdmin(): boolean;
  setIsAdmin(value: boolean): ServiceAccount;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): ServiceAccount;
  hasCreatedAt(): boolean;
  clearCreatedAt(): ServiceAccount;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): ServiceAccount;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): ServiceAccount;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ServiceAccount.AsObject;
  static toObject(includeInstance: boolean, msg: ServiceAccount): ServiceAccount.AsObject;
  static serializeBinaryToWriter(message: ServiceAccount, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ServiceAccount;
  static deserializeBinaryFromReader(message: ServiceAccount, reader: jspb.BinaryReader): ServiceAccount;
}

export namespace ServiceAccount {
  export type AsObject = {
    id: string,
    name: string,
    secretKeyHash: string,
    isAdmin: boolean,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class CreateServiceAccountRequest extends jspb.Message {
  getName(): string;
  setName(value: string): CreateServiceAccountRequest;

  getIsAdmin(): boolean;
  setIsAdmin(value: boolean): CreateServiceAccountRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateServiceAccountRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateServiceAccountRequest): CreateServiceAccountRequest.AsObject;
  static serializeBinaryToWriter(message: CreateServiceAccountRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateServiceAccountRequest;
  static deserializeBinaryFromReader(message: CreateServiceAccountRequest, reader: jspb.BinaryReader): CreateServiceAccountRequest;
}

export namespace CreateServiceAccountRequest {
  export type AsObject = {
    name: string,
    isAdmin: boolean,
  }
}

export class CreateServiceAccountResponse extends jspb.Message {
  getId(): string;
  setId(value: string): CreateServiceAccountResponse;

  getName(): string;
  setName(value: string): CreateServiceAccountResponse;

  getSecretKey(): string;
  setSecretKey(value: string): CreateServiceAccountResponse;

  getIsAdmin(): boolean;
  setIsAdmin(value: boolean): CreateServiceAccountResponse;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): CreateServiceAccountResponse;
  hasCreatedAt(): boolean;
  clearCreatedAt(): CreateServiceAccountResponse;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): CreateServiceAccountResponse;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): CreateServiceAccountResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateServiceAccountResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateServiceAccountResponse): CreateServiceAccountResponse.AsObject;
  static serializeBinaryToWriter(message: CreateServiceAccountResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateServiceAccountResponse;
  static deserializeBinaryFromReader(message: CreateServiceAccountResponse, reader: jspb.BinaryReader): CreateServiceAccountResponse;
}

export namespace CreateServiceAccountResponse {
  export type AsObject = {
    id: string,
    name: string,
    secretKey: string,
    isAdmin: boolean,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class GetServiceAccountRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetServiceAccountRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetServiceAccountRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetServiceAccountRequest): GetServiceAccountRequest.AsObject;
  static serializeBinaryToWriter(message: GetServiceAccountRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetServiceAccountRequest;
  static deserializeBinaryFromReader(message: GetServiceAccountRequest, reader: jspb.BinaryReader): GetServiceAccountRequest;
}

export namespace GetServiceAccountRequest {
  export type AsObject = {
    id: string,
  }
}

export class DeleteServiceAccountRequest extends jspb.Message {
  getId(): string;
  setId(value: string): DeleteServiceAccountRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteServiceAccountRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteServiceAccountRequest): DeleteServiceAccountRequest.AsObject;
  static serializeBinaryToWriter(message: DeleteServiceAccountRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteServiceAccountRequest;
  static deserializeBinaryFromReader(message: DeleteServiceAccountRequest, reader: jspb.BinaryReader): DeleteServiceAccountRequest;
}

export namespace DeleteServiceAccountRequest {
  export type AsObject = {
    id: string,
  }
}

export class UpdateServiceAccountRequest extends jspb.Message {
  getId(): string;
  setId(value: string): UpdateServiceAccountRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateServiceAccountRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateServiceAccountRequest): UpdateServiceAccountRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateServiceAccountRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateServiceAccountRequest;
  static deserializeBinaryFromReader(message: UpdateServiceAccountRequest, reader: jspb.BinaryReader): UpdateServiceAccountRequest;
}

export namespace UpdateServiceAccountRequest {
  export type AsObject = {
    id: string,
  }
}

export class UpdateServiceAccountResponse extends jspb.Message {
  getId(): string;
  setId(value: string): UpdateServiceAccountResponse;

  getName(): string;
  setName(value: string): UpdateServiceAccountResponse;

  getSecretKey(): string;
  setSecretKey(value: string): UpdateServiceAccountResponse;

  getIsAdmin(): boolean;
  setIsAdmin(value: boolean): UpdateServiceAccountResponse;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): UpdateServiceAccountResponse;
  hasCreatedAt(): boolean;
  clearCreatedAt(): UpdateServiceAccountResponse;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): UpdateServiceAccountResponse;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): UpdateServiceAccountResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateServiceAccountResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateServiceAccountResponse): UpdateServiceAccountResponse.AsObject;
  static serializeBinaryToWriter(message: UpdateServiceAccountResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateServiceAccountResponse;
  static deserializeBinaryFromReader(message: UpdateServiceAccountResponse, reader: jspb.BinaryReader): UpdateServiceAccountResponse;
}

export namespace UpdateServiceAccountResponse {
  export type AsObject = {
    id: string,
    name: string,
    secretKey: string,
    isAdmin: boolean,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class ListServiceAccountsRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListServiceAccountsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListServiceAccountsRequest): ListServiceAccountsRequest.AsObject;
  static serializeBinaryToWriter(message: ListServiceAccountsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListServiceAccountsRequest;
  static deserializeBinaryFromReader(message: ListServiceAccountsRequest, reader: jspb.BinaryReader): ListServiceAccountsRequest;
}

export namespace ListServiceAccountsRequest {
  export type AsObject = {
  }
}

export class LoginRequest extends jspb.Message {
  getExternalId(): string;
  setExternalId(value: string): LoginRequest;

  getServiceAccountId(): string;
  setServiceAccountId(value: string): LoginRequest;

  getPassword(): string;
  setPassword(value: string): LoginRequest;

  getDidLogin(): DidLogin | undefined;
  setDidLogin(value?: DidLogin): LoginRequest;
  hasDidLogin(): boolean;
  clearDidLogin(): LoginRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoginRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LoginRequest): LoginRequest.AsObject;
  static serializeBinaryToWriter(message: LoginRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoginRequest;
  static deserializeBinaryFromReader(message: LoginRequest, reader: jspb.BinaryReader): LoginRequest;
}

export namespace LoginRequest {
  export type AsObject = {
    externalId: string,
    serviceAccountId: string,
    password: string,
    didLogin?: DidLogin.AsObject,
  }
}

export class DidLogin extends jspb.Message {
  getMessage(): string;
  setMessage(value: string): DidLogin;

  getSignature(): string;
  setSignature(value: string): DidLogin;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DidLogin.AsObject;
  static toObject(includeInstance: boolean, msg: DidLogin): DidLogin.AsObject;
  static serializeBinaryToWriter(message: DidLogin, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DidLogin;
  static deserializeBinaryFromReader(message: DidLogin, reader: jspb.BinaryReader): DidLogin;
}

export namespace DidLogin {
  export type AsObject = {
    message: string,
    signature: string,
  }
}

export class LoginResponse extends jspb.Message {
  getAccessToken(): string;
  setAccessToken(value: string): LoginResponse;

  getRefreshToken(): string;
  setRefreshToken(value: string): LoginResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoginResponse.AsObject;
  static toObject(includeInstance: boolean, msg: LoginResponse): LoginResponse.AsObject;
  static serializeBinaryToWriter(message: LoginResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoginResponse;
  static deserializeBinaryFromReader(message: LoginResponse, reader: jspb.BinaryReader): LoginResponse;
}

export namespace LoginResponse {
  export type AsObject = {
    accessToken: string,
    refreshToken: string,
  }
}

export class RefreshRequest extends jspb.Message {
  getRefreshToken(): string;
  setRefreshToken(value: string): RefreshRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RefreshRequest.AsObject;
  static toObject(includeInstance: boolean, msg: RefreshRequest): RefreshRequest.AsObject;
  static serializeBinaryToWriter(message: RefreshRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RefreshRequest;
  static deserializeBinaryFromReader(message: RefreshRequest, reader: jspb.BinaryReader): RefreshRequest;
}

export namespace RefreshRequest {
  export type AsObject = {
    refreshToken: string,
  }
}

export class Group extends jspb.Message {
  getId(): string;
  setId(value: string): Group;

  getName(): string;
  setName(value: string): Group;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Group;
  hasCreatedAt(): boolean;
  clearCreatedAt(): Group;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Group;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): Group;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Group.AsObject;
  static toObject(includeInstance: boolean, msg: Group): Group.AsObject;
  static serializeBinaryToWriter(message: Group, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Group;
  static deserializeBinaryFromReader(message: Group, reader: jspb.BinaryReader): Group;
}

export namespace Group {
  export type AsObject = {
    id: string,
    name: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class CreateGroupRequest extends jspb.Message {
  getName(): string;
  setName(value: string): CreateGroupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateGroupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateGroupRequest): CreateGroupRequest.AsObject;
  static serializeBinaryToWriter(message: CreateGroupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateGroupRequest;
  static deserializeBinaryFromReader(message: CreateGroupRequest, reader: jspb.BinaryReader): CreateGroupRequest;
}

export namespace CreateGroupRequest {
  export type AsObject = {
    name: string,
  }
}

export class GetGroupRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetGroupRequest;

  getName(): string;
  setName(value: string): GetGroupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetGroupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetGroupRequest): GetGroupRequest.AsObject;
  static serializeBinaryToWriter(message: GetGroupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetGroupRequest;
  static deserializeBinaryFromReader(message: GetGroupRequest, reader: jspb.BinaryReader): GetGroupRequest;
}

export namespace GetGroupRequest {
  export type AsObject = {
    id: string,
    name: string,
  }
}

export class DeleteGroupRequest extends jspb.Message {
  getId(): string;
  setId(value: string): DeleteGroupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteGroupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteGroupRequest): DeleteGroupRequest.AsObject;
  static serializeBinaryToWriter(message: DeleteGroupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteGroupRequest;
  static deserializeBinaryFromReader(message: DeleteGroupRequest, reader: jspb.BinaryReader): DeleteGroupRequest;
}

export namespace DeleteGroupRequest {
  export type AsObject = {
    id: string,
  }
}

export class UpdateGroupRequest extends jspb.Message {
  getId(): string;
  setId(value: string): UpdateGroupRequest;

  getName(): string;
  setName(value: string): UpdateGroupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateGroupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateGroupRequest): UpdateGroupRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateGroupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateGroupRequest;
  static deserializeBinaryFromReader(message: UpdateGroupRequest, reader: jspb.BinaryReader): UpdateGroupRequest;
}

export namespace UpdateGroupRequest {
  export type AsObject = {
    id: string,
    name: string,
  }
}

export class ListGroupsRequest extends jspb.Message {
  getFilter(): string;
  setFilter(value: string): ListGroupsRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListGroupsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListGroupsRequest): ListGroupsRequest.AsObject;
  static serializeBinaryToWriter(message: ListGroupsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListGroupsRequest;
  static deserializeBinaryFromReader(message: ListGroupsRequest, reader: jspb.BinaryReader): ListGroupsRequest;
}

export namespace ListGroupsRequest {
  export type AsObject = {
    filter: string,
  }
}

export class AddUserToGroupRequest extends jspb.Message {
  getUserId(): string;
  setUserId(value: string): AddUserToGroupRequest;

  getGroupId(): string;
  setGroupId(value: string): AddUserToGroupRequest;

  getIsAdmin(): boolean;
  setIsAdmin(value: boolean): AddUserToGroupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AddUserToGroupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AddUserToGroupRequest): AddUserToGroupRequest.AsObject;
  static serializeBinaryToWriter(message: AddUserToGroupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AddUserToGroupRequest;
  static deserializeBinaryFromReader(message: AddUserToGroupRequest, reader: jspb.BinaryReader): AddUserToGroupRequest;
}

export namespace AddUserToGroupRequest {
  export type AsObject = {
    userId: string,
    groupId: string,
    isAdmin: boolean,
  }
}

export class DelUserFromGroupRequest extends jspb.Message {
  getUserId(): string;
  setUserId(value: string): DelUserFromGroupRequest;

  getGroupId(): string;
  setGroupId(value: string): DelUserFromGroupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DelUserFromGroupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DelUserFromGroupRequest): DelUserFromGroupRequest.AsObject;
  static serializeBinaryToWriter(message: DelUserFromGroupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DelUserFromGroupRequest;
  static deserializeBinaryFromReader(message: DelUserFromGroupRequest, reader: jspb.BinaryReader): DelUserFromGroupRequest;
}

export namespace DelUserFromGroupRequest {
  export type AsObject = {
    userId: string,
    groupId: string,
  }
}

export class GroupMember extends jspb.Message {
  getUserId(): string;
  setUserId(value: string): GroupMember;

  getUserName(): string;
  setUserName(value: string): GroupMember;

  getUserExternalId(): string;
  setUserExternalId(value: string): GroupMember;

  getIsAdmin(): boolean;
  setIsAdmin(value: boolean): GroupMember;

  getJoinedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setJoinedAt(value?: google_protobuf_timestamp_pb.Timestamp): GroupMember;
  hasJoinedAt(): boolean;
  clearJoinedAt(): GroupMember;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GroupMember.AsObject;
  static toObject(includeInstance: boolean, msg: GroupMember): GroupMember.AsObject;
  static serializeBinaryToWriter(message: GroupMember, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GroupMember;
  static deserializeBinaryFromReader(message: GroupMember, reader: jspb.BinaryReader): GroupMember;
}

export namespace GroupMember {
  export type AsObject = {
    userId: string,
    userName: string,
    userExternalId: string,
    isAdmin: boolean,
    joinedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class ListGroupMembersRequest extends jspb.Message {
  getGroupId(): string;
  setGroupId(value: string): ListGroupMembersRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListGroupMembersRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListGroupMembersRequest): ListGroupMembersRequest.AsObject;
  static serializeBinaryToWriter(message: ListGroupMembersRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListGroupMembersRequest;
  static deserializeBinaryFromReader(message: ListGroupMembersRequest, reader: jspb.BinaryReader): ListGroupMembersRequest;
}

export namespace ListGroupMembersRequest {
  export type AsObject = {
    groupId: string,
  }
}

