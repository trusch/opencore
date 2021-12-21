import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';


export class Resource extends jspb.Message {
  getId(): string;
  setId(value: string): Resource;

  getParentId(): string;
  setParentId(value: string): Resource;

  getPermissionParentId(): string;
  setPermissionParentId(value: string): Resource;

  getCreatorId(): string;
  setCreatorId(value: string): Resource;

  getKind(): string;
  setKind(value: string): Resource;

  getData(): string;
  setData(value: string): Resource;

  getLabelsMap(): jspb.Map<string, string>;
  clearLabelsMap(): Resource;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Resource;
  hasCreatedAt(): boolean;
  clearCreatedAt(): Resource;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Resource;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): Resource;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Resource.AsObject;
  static toObject(includeInstance: boolean, msg: Resource): Resource.AsObject;
  static serializeBinaryToWriter(message: Resource, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Resource;
  static deserializeBinaryFromReader(message: Resource, reader: jspb.BinaryReader): Resource;
}

export namespace Resource {
  export type AsObject = {
    id: string,
    parentId: string,
    permissionParentId: string,
    creatorId: string,
    kind: string,
    data: string,
    labelsMap: Array<[string, string]>,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class CreateResourceRequest extends jspb.Message {
  getKind(): string;
  setKind(value: string): CreateResourceRequest;

  getParentId(): string;
  setParentId(value: string): CreateResourceRequest;

  getPermissionParentId(): string;
  setPermissionParentId(value: string): CreateResourceRequest;

  getData(): string;
  setData(value: string): CreateResourceRequest;

  getLabelsMap(): jspb.Map<string, string>;
  clearLabelsMap(): CreateResourceRequest;

  getSharesList(): Array<ShareRequest>;
  setSharesList(value: Array<ShareRequest>): CreateResourceRequest;
  clearSharesList(): CreateResourceRequest;
  addShares(value?: ShareRequest, index?: number): ShareRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateResourceRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateResourceRequest): CreateResourceRequest.AsObject;
  static serializeBinaryToWriter(message: CreateResourceRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateResourceRequest;
  static deserializeBinaryFromReader(message: CreateResourceRequest, reader: jspb.BinaryReader): CreateResourceRequest;
}

export namespace CreateResourceRequest {
  export type AsObject = {
    kind: string,
    parentId: string,
    permissionParentId: string,
    data: string,
    labelsMap: Array<[string, string]>,
    sharesList: Array<ShareRequest.AsObject>,
  }
}

export class GetResourceRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetResourceRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetResourceRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetResourceRequest): GetResourceRequest.AsObject;
  static serializeBinaryToWriter(message: GetResourceRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetResourceRequest;
  static deserializeBinaryFromReader(message: GetResourceRequest, reader: jspb.BinaryReader): GetResourceRequest;
}

export namespace GetResourceRequest {
  export type AsObject = {
    id: string,
  }
}

export class DeleteResourceRequest extends jspb.Message {
  getId(): string;
  setId(value: string): DeleteResourceRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteResourceRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteResourceRequest): DeleteResourceRequest.AsObject;
  static serializeBinaryToWriter(message: DeleteResourceRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteResourceRequest;
  static deserializeBinaryFromReader(message: DeleteResourceRequest, reader: jspb.BinaryReader): DeleteResourceRequest;
}

export namespace DeleteResourceRequest {
  export type AsObject = {
    id: string,
  }
}

export class UpdateResourceRequest extends jspb.Message {
  getId(): string;
  setId(value: string): UpdateResourceRequest;

  getData(): string;
  setData(value: string): UpdateResourceRequest;

  getLabelsMap(): jspb.Map<string, string>;
  clearLabelsMap(): UpdateResourceRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateResourceRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateResourceRequest): UpdateResourceRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateResourceRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateResourceRequest;
  static deserializeBinaryFromReader(message: UpdateResourceRequest, reader: jspb.BinaryReader): UpdateResourceRequest;
}

export namespace UpdateResourceRequest {
  export type AsObject = {
    id: string,
    data: string,
    labelsMap: Array<[string, string]>,
  }
}

export class ListResourcesRequest extends jspb.Message {
  getLabelsMap(): jspb.Map<string, string>;
  clearLabelsMap(): ListResourcesRequest;

  getFilter(): string;
  setFilter(value: string): ListResourcesRequest;

  getSkip(): number;
  setSkip(value: number): ListResourcesRequest;

  getKind(): string;
  setKind(value: string): ListResourcesRequest;

  getQuery(): string;
  setQuery(value: string): ListResourcesRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListResourcesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListResourcesRequest): ListResourcesRequest.AsObject;
  static serializeBinaryToWriter(message: ListResourcesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListResourcesRequest;
  static deserializeBinaryFromReader(message: ListResourcesRequest, reader: jspb.BinaryReader): ListResourcesRequest;
}

export namespace ListResourcesRequest {
  export type AsObject = {
    labelsMap: Array<[string, string]>,
    filter: string,
    skip: number,
    kind: string,
    query: string,
  }
}

export class Schema extends jspb.Message {
  getId(): string;
  setId(value: string): Schema;

  getKind(): string;
  setKind(value: string): Schema;

  getData(): string;
  setData(value: string): Schema;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Schema;
  hasCreatedAt(): boolean;
  clearCreatedAt(): Schema;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Schema;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): Schema;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Schema.AsObject;
  static toObject(includeInstance: boolean, msg: Schema): Schema.AsObject;
  static serializeBinaryToWriter(message: Schema, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Schema;
  static deserializeBinaryFromReader(message: Schema, reader: jspb.BinaryReader): Schema;
}

export namespace Schema {
  export type AsObject = {
    id: string,
    kind: string,
    data: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class CreateSchemaRequest extends jspb.Message {
  getKind(): string;
  setKind(value: string): CreateSchemaRequest;

  getData(): string;
  setData(value: string): CreateSchemaRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateSchemaRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateSchemaRequest): CreateSchemaRequest.AsObject;
  static serializeBinaryToWriter(message: CreateSchemaRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateSchemaRequest;
  static deserializeBinaryFromReader(message: CreateSchemaRequest, reader: jspb.BinaryReader): CreateSchemaRequest;
}

export namespace CreateSchemaRequest {
  export type AsObject = {
    kind: string,
    data: string,
  }
}

export class GetSchemaRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetSchemaRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSchemaRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetSchemaRequest): GetSchemaRequest.AsObject;
  static serializeBinaryToWriter(message: GetSchemaRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSchemaRequest;
  static deserializeBinaryFromReader(message: GetSchemaRequest, reader: jspb.BinaryReader): GetSchemaRequest;
}

export namespace GetSchemaRequest {
  export type AsObject = {
    id: string,
  }
}

export class DeleteSchemaRequest extends jspb.Message {
  getId(): string;
  setId(value: string): DeleteSchemaRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteSchemaRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteSchemaRequest): DeleteSchemaRequest.AsObject;
  static serializeBinaryToWriter(message: DeleteSchemaRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteSchemaRequest;
  static deserializeBinaryFromReader(message: DeleteSchemaRequest, reader: jspb.BinaryReader): DeleteSchemaRequest;
}

export namespace DeleteSchemaRequest {
  export type AsObject = {
    id: string,
  }
}

export class UpdateSchemaRequest extends jspb.Message {
  getId(): string;
  setId(value: string): UpdateSchemaRequest;

  getData(): string;
  setData(value: string): UpdateSchemaRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateSchemaRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateSchemaRequest): UpdateSchemaRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateSchemaRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateSchemaRequest;
  static deserializeBinaryFromReader(message: UpdateSchemaRequest, reader: jspb.BinaryReader): UpdateSchemaRequest;
}

export namespace UpdateSchemaRequest {
  export type AsObject = {
    id: string,
    data: string,
  }
}

export class ListSchemasRequest extends jspb.Message {
  getFilter(): string;
  setFilter(value: string): ListSchemasRequest;

  getPage(): number;
  setPage(value: number): ListSchemasRequest;

  getPageSize(): number;
  setPageSize(value: number): ListSchemasRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListSchemasRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListSchemasRequest): ListSchemasRequest.AsObject;
  static serializeBinaryToWriter(message: ListSchemasRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListSchemasRequest;
  static deserializeBinaryFromReader(message: ListSchemasRequest, reader: jspb.BinaryReader): ListSchemasRequest;
}

export namespace ListSchemasRequest {
  export type AsObject = {
    filter: string,
    page: number,
    pageSize: number,
  }
}

export class PermissionCheckRequest extends jspb.Message {
  getResourceId(): string;
  setResourceId(value: string): PermissionCheckRequest;

  getPrincipalId(): string;
  setPrincipalId(value: string): PermissionCheckRequest;

  getAction(): string;
  setAction(value: string): PermissionCheckRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PermissionCheckRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PermissionCheckRequest): PermissionCheckRequest.AsObject;
  static serializeBinaryToWriter(message: PermissionCheckRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PermissionCheckRequest;
  static deserializeBinaryFromReader(message: PermissionCheckRequest, reader: jspb.BinaryReader): PermissionCheckRequest;
}

export namespace PermissionCheckRequest {
  export type AsObject = {
    resourceId: string,
    principalId: string,
    action: string,
  }
}

export class PermissionCheckResponse extends jspb.Message {
  getGranted(): boolean;
  setGranted(value: boolean): PermissionCheckResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PermissionCheckResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PermissionCheckResponse): PermissionCheckResponse.AsObject;
  static serializeBinaryToWriter(message: PermissionCheckResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PermissionCheckResponse;
  static deserializeBinaryFromReader(message: PermissionCheckResponse, reader: jspb.BinaryReader): PermissionCheckResponse;
}

export namespace PermissionCheckResponse {
  export type AsObject = {
    granted: boolean,
  }
}

export class ShareRequest extends jspb.Message {
  getResourceId(): string;
  setResourceId(value: string): ShareRequest;

  getPrincipalId(): string;
  setPrincipalId(value: string): ShareRequest;

  getActionsList(): Array<string>;
  setActionsList(value: Array<string>): ShareRequest;
  clearActionsList(): ShareRequest;
  addActions(value: string, index?: number): ShareRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ShareRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ShareRequest): ShareRequest.AsObject;
  static serializeBinaryToWriter(message: ShareRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ShareRequest;
  static deserializeBinaryFromReader(message: ShareRequest, reader: jspb.BinaryReader): ShareRequest;
}

export namespace ShareRequest {
  export type AsObject = {
    resourceId: string,
    principalId: string,
    actionsList: Array<string>,
  }
}

export class UnshareRequest extends jspb.Message {
  getResourceId(): string;
  setResourceId(value: string): UnshareRequest;

  getPrincipalId(): string;
  setPrincipalId(value: string): UnshareRequest;

  getActionsList(): Array<string>;
  setActionsList(value: Array<string>): UnshareRequest;
  clearActionsList(): UnshareRequest;
  addActions(value: string, index?: number): UnshareRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UnshareRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UnshareRequest): UnshareRequest.AsObject;
  static serializeBinaryToWriter(message: UnshareRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UnshareRequest;
  static deserializeBinaryFromReader(message: UnshareRequest, reader: jspb.BinaryReader): UnshareRequest;
}

export namespace UnshareRequest {
  export type AsObject = {
    resourceId: string,
    principalId: string,
    actionsList: Array<string>,
  }
}

export class PermissionInfo extends jspb.Message {
  getResourceId(): string;
  setResourceId(value: string): PermissionInfo;

  getPrincipalId(): string;
  setPrincipalId(value: string): PermissionInfo;

  getActionsList(): Array<string>;
  setActionsList(value: Array<string>): PermissionInfo;
  clearActionsList(): PermissionInfo;
  addActions(value: string, index?: number): PermissionInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PermissionInfo.AsObject;
  static toObject(includeInstance: boolean, msg: PermissionInfo): PermissionInfo.AsObject;
  static serializeBinaryToWriter(message: PermissionInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PermissionInfo;
  static deserializeBinaryFromReader(message: PermissionInfo, reader: jspb.BinaryReader): PermissionInfo;
}

export namespace PermissionInfo {
  export type AsObject = {
    resourceId: string,
    principalId: string,
    actionsList: Array<string>,
  }
}

export class GetPermissionInfoRequest extends jspb.Message {
  getResourceId(): string;
  setResourceId(value: string): GetPermissionInfoRequest;

  getPrincipalId(): string;
  setPrincipalId(value: string): GetPermissionInfoRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetPermissionInfoRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetPermissionInfoRequest): GetPermissionInfoRequest.AsObject;
  static serializeBinaryToWriter(message: GetPermissionInfoRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetPermissionInfoRequest;
  static deserializeBinaryFromReader(message: GetPermissionInfoRequest, reader: jspb.BinaryReader): GetPermissionInfoRequest;
}

export namespace GetPermissionInfoRequest {
  export type AsObject = {
    resourceId: string,
    principalId: string,
  }
}

export class ListPermissionsRequest extends jspb.Message {
  getResourceId(): string;
  setResourceId(value: string): ListPermissionsRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListPermissionsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListPermissionsRequest): ListPermissionsRequest.AsObject;
  static serializeBinaryToWriter(message: ListPermissionsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListPermissionsRequest;
  static deserializeBinaryFromReader(message: ListPermissionsRequest, reader: jspb.BinaryReader): ListPermissionsRequest;
}

export namespace ListPermissionsRequest {
  export type AsObject = {
    resourceId: string,
  }
}

export class Event extends jspb.Message {
  getId(): string;
  setId(value: string): Event;

  getResourceId(): string;
  setResourceId(value: string): Event;

  getResourceKind(): string;
  setResourceKind(value: string): Event;

  getResourceLabelsMap(): jspb.Map<string, string>;
  clearResourceLabelsMap(): Event;

  getEventType(): EventType;
  setEventType(value: EventType): Event;

  getData(): string;
  setData(value: string): Event;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Event;
  hasCreatedAt(): boolean;
  clearCreatedAt(): Event;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Event.AsObject;
  static toObject(includeInstance: boolean, msg: Event): Event.AsObject;
  static serializeBinaryToWriter(message: Event, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Event;
  static deserializeBinaryFromReader(message: Event, reader: jspb.BinaryReader): Event;
}

export namespace Event {
  export type AsObject = {
    id: string,
    resourceId: string,
    resourceKind: string,
    resourceLabelsMap: Array<[string, string]>,
    eventType: EventType,
    data: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class PublishRequest extends jspb.Message {
  getResourceId(): string;
  setResourceId(value: string): PublishRequest;

  getResourceKind(): string;
  setResourceKind(value: string): PublishRequest;

  getEventType(): EventType;
  setEventType(value: EventType): PublishRequest;

  getData(): string;
  setData(value: string): PublishRequest;

  getLabelsMap(): jspb.Map<string, string>;
  clearLabelsMap(): PublishRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PublishRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PublishRequest): PublishRequest.AsObject;
  static serializeBinaryToWriter(message: PublishRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PublishRequest;
  static deserializeBinaryFromReader(message: PublishRequest, reader: jspb.BinaryReader): PublishRequest;
}

export namespace PublishRequest {
  export type AsObject = {
    resourceId: string,
    resourceKind: string,
    eventType: EventType,
    data: string,
    labelsMap: Array<[string, string]>,
  }
}

export class SubscribeRequest extends jspb.Message {
  getResourceId(): string;
  setResourceId(value: string): SubscribeRequest;

  getResourceKind(): string;
  setResourceKind(value: string): SubscribeRequest;

  getEventType(): EventType;
  setEventType(value: EventType): SubscribeRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SubscribeRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SubscribeRequest): SubscribeRequest.AsObject;
  static serializeBinaryToWriter(message: SubscribeRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SubscribeRequest;
  static deserializeBinaryFromReader(message: SubscribeRequest, reader: jspb.BinaryReader): SubscribeRequest;
}

export namespace SubscribeRequest {
  export type AsObject = {
    resourceId: string,
    resourceKind: string,
    eventType: EventType,
  }
}

export class LockRequest extends jspb.Message {
  getLockId(): string;
  setLockId(value: string): LockRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LockRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LockRequest): LockRequest.AsObject;
  static serializeBinaryToWriter(message: LockRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LockRequest;
  static deserializeBinaryFromReader(message: LockRequest, reader: jspb.BinaryReader): LockRequest;
}

export namespace LockRequest {
  export type AsObject = {
    lockId: string,
  }
}

export class LockResponse extends jspb.Message {
  getLockId(): string;
  setLockId(value: string): LockResponse;

  getFencingToken(): number;
  setFencingToken(value: number): LockResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LockResponse.AsObject;
  static toObject(includeInstance: boolean, msg: LockResponse): LockResponse.AsObject;
  static serializeBinaryToWriter(message: LockResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LockResponse;
  static deserializeBinaryFromReader(message: LockResponse, reader: jspb.BinaryReader): LockResponse;
}

export namespace LockResponse {
  export type AsObject = {
    lockId: string,
    fencingToken: number,
  }
}

export enum EventType { 
  NONE = 0,
  CREATE = 1,
  UPDATE = 2,
  DELETE = 3,
}
