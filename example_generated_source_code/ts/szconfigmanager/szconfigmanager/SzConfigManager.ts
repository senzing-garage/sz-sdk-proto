// Original file: szconfigmanager.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { AddConfigRequest as _szconfigmanager_AddConfigRequest, AddConfigRequest__Output as _szconfigmanager_AddConfigRequest__Output } from '../szconfigmanager/AddConfigRequest';
import type { AddConfigResponse as _szconfigmanager_AddConfigResponse, AddConfigResponse__Output as _szconfigmanager_AddConfigResponse__Output } from '../szconfigmanager/AddConfigResponse';
import type { GetConfigRequest as _szconfigmanager_GetConfigRequest, GetConfigRequest__Output as _szconfigmanager_GetConfigRequest__Output } from '../szconfigmanager/GetConfigRequest';
import type { GetConfigResponse as _szconfigmanager_GetConfigResponse, GetConfigResponse__Output as _szconfigmanager_GetConfigResponse__Output } from '../szconfigmanager/GetConfigResponse';
import type { GetConfigsRequest as _szconfigmanager_GetConfigsRequest, GetConfigsRequest__Output as _szconfigmanager_GetConfigsRequest__Output } from '../szconfigmanager/GetConfigsRequest';
import type { GetConfigsResponse as _szconfigmanager_GetConfigsResponse, GetConfigsResponse__Output as _szconfigmanager_GetConfigsResponse__Output } from '../szconfigmanager/GetConfigsResponse';
import type { GetDefaultConfigIdRequest as _szconfigmanager_GetDefaultConfigIdRequest, GetDefaultConfigIdRequest__Output as _szconfigmanager_GetDefaultConfigIdRequest__Output } from '../szconfigmanager/GetDefaultConfigIdRequest';
import type { GetDefaultConfigIdResponse as _szconfigmanager_GetDefaultConfigIdResponse, GetDefaultConfigIdResponse__Output as _szconfigmanager_GetDefaultConfigIdResponse__Output } from '../szconfigmanager/GetDefaultConfigIdResponse';
import type { ReplaceDefaultConfigIdRequest as _szconfigmanager_ReplaceDefaultConfigIdRequest, ReplaceDefaultConfigIdRequest__Output as _szconfigmanager_ReplaceDefaultConfigIdRequest__Output } from '../szconfigmanager/ReplaceDefaultConfigIdRequest';
import type { ReplaceDefaultConfigIdResponse as _szconfigmanager_ReplaceDefaultConfigIdResponse, ReplaceDefaultConfigIdResponse__Output as _szconfigmanager_ReplaceDefaultConfigIdResponse__Output } from '../szconfigmanager/ReplaceDefaultConfigIdResponse';
import type { SetDefaultConfigIdRequest as _szconfigmanager_SetDefaultConfigIdRequest, SetDefaultConfigIdRequest__Output as _szconfigmanager_SetDefaultConfigIdRequest__Output } from '../szconfigmanager/SetDefaultConfigIdRequest';
import type { SetDefaultConfigIdResponse as _szconfigmanager_SetDefaultConfigIdResponse, SetDefaultConfigIdResponse__Output as _szconfigmanager_SetDefaultConfigIdResponse__Output } from '../szconfigmanager/SetDefaultConfigIdResponse';

export interface SzConfigManagerClient extends grpc.Client {
  AddConfig(argument: _szconfigmanager_AddConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_AddConfigResponse__Output>): grpc.ClientUnaryCall;
  AddConfig(argument: _szconfigmanager_AddConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfigmanager_AddConfigResponse__Output>): grpc.ClientUnaryCall;
  AddConfig(argument: _szconfigmanager_AddConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_AddConfigResponse__Output>): grpc.ClientUnaryCall;
  AddConfig(argument: _szconfigmanager_AddConfigRequest, callback: grpc.requestCallback<_szconfigmanager_AddConfigResponse__Output>): grpc.ClientUnaryCall;
  addConfig(argument: _szconfigmanager_AddConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_AddConfigResponse__Output>): grpc.ClientUnaryCall;
  addConfig(argument: _szconfigmanager_AddConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfigmanager_AddConfigResponse__Output>): grpc.ClientUnaryCall;
  addConfig(argument: _szconfigmanager_AddConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_AddConfigResponse__Output>): grpc.ClientUnaryCall;
  addConfig(argument: _szconfigmanager_AddConfigRequest, callback: grpc.requestCallback<_szconfigmanager_AddConfigResponse__Output>): grpc.ClientUnaryCall;
  
  GetConfig(argument: _szconfigmanager_GetConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_GetConfigResponse__Output>): grpc.ClientUnaryCall;
  GetConfig(argument: _szconfigmanager_GetConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfigmanager_GetConfigResponse__Output>): grpc.ClientUnaryCall;
  GetConfig(argument: _szconfigmanager_GetConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_GetConfigResponse__Output>): grpc.ClientUnaryCall;
  GetConfig(argument: _szconfigmanager_GetConfigRequest, callback: grpc.requestCallback<_szconfigmanager_GetConfigResponse__Output>): grpc.ClientUnaryCall;
  getConfig(argument: _szconfigmanager_GetConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_GetConfigResponse__Output>): grpc.ClientUnaryCall;
  getConfig(argument: _szconfigmanager_GetConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfigmanager_GetConfigResponse__Output>): grpc.ClientUnaryCall;
  getConfig(argument: _szconfigmanager_GetConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_GetConfigResponse__Output>): grpc.ClientUnaryCall;
  getConfig(argument: _szconfigmanager_GetConfigRequest, callback: grpc.requestCallback<_szconfigmanager_GetConfigResponse__Output>): grpc.ClientUnaryCall;
  
  GetConfigs(argument: _szconfigmanager_GetConfigsRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_GetConfigsResponse__Output>): grpc.ClientUnaryCall;
  GetConfigs(argument: _szconfigmanager_GetConfigsRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfigmanager_GetConfigsResponse__Output>): grpc.ClientUnaryCall;
  GetConfigs(argument: _szconfigmanager_GetConfigsRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_GetConfigsResponse__Output>): grpc.ClientUnaryCall;
  GetConfigs(argument: _szconfigmanager_GetConfigsRequest, callback: grpc.requestCallback<_szconfigmanager_GetConfigsResponse__Output>): grpc.ClientUnaryCall;
  getConfigs(argument: _szconfigmanager_GetConfigsRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_GetConfigsResponse__Output>): grpc.ClientUnaryCall;
  getConfigs(argument: _szconfigmanager_GetConfigsRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfigmanager_GetConfigsResponse__Output>): grpc.ClientUnaryCall;
  getConfigs(argument: _szconfigmanager_GetConfigsRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_GetConfigsResponse__Output>): grpc.ClientUnaryCall;
  getConfigs(argument: _szconfigmanager_GetConfigsRequest, callback: grpc.requestCallback<_szconfigmanager_GetConfigsResponse__Output>): grpc.ClientUnaryCall;
  
  GetDefaultConfigId(argument: _szconfigmanager_GetDefaultConfigIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_GetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  GetDefaultConfigId(argument: _szconfigmanager_GetDefaultConfigIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfigmanager_GetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  GetDefaultConfigId(argument: _szconfigmanager_GetDefaultConfigIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_GetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  GetDefaultConfigId(argument: _szconfigmanager_GetDefaultConfigIdRequest, callback: grpc.requestCallback<_szconfigmanager_GetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  getDefaultConfigId(argument: _szconfigmanager_GetDefaultConfigIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_GetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  getDefaultConfigId(argument: _szconfigmanager_GetDefaultConfigIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfigmanager_GetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  getDefaultConfigId(argument: _szconfigmanager_GetDefaultConfigIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_GetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  getDefaultConfigId(argument: _szconfigmanager_GetDefaultConfigIdRequest, callback: grpc.requestCallback<_szconfigmanager_GetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  
  ReplaceDefaultConfigId(argument: _szconfigmanager_ReplaceDefaultConfigIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_ReplaceDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  ReplaceDefaultConfigId(argument: _szconfigmanager_ReplaceDefaultConfigIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfigmanager_ReplaceDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  ReplaceDefaultConfigId(argument: _szconfigmanager_ReplaceDefaultConfigIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_ReplaceDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  ReplaceDefaultConfigId(argument: _szconfigmanager_ReplaceDefaultConfigIdRequest, callback: grpc.requestCallback<_szconfigmanager_ReplaceDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  replaceDefaultConfigId(argument: _szconfigmanager_ReplaceDefaultConfigIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_ReplaceDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  replaceDefaultConfigId(argument: _szconfigmanager_ReplaceDefaultConfigIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfigmanager_ReplaceDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  replaceDefaultConfigId(argument: _szconfigmanager_ReplaceDefaultConfigIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_ReplaceDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  replaceDefaultConfigId(argument: _szconfigmanager_ReplaceDefaultConfigIdRequest, callback: grpc.requestCallback<_szconfigmanager_ReplaceDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  
  SetDefaultConfigId(argument: _szconfigmanager_SetDefaultConfigIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_SetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  SetDefaultConfigId(argument: _szconfigmanager_SetDefaultConfigIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfigmanager_SetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  SetDefaultConfigId(argument: _szconfigmanager_SetDefaultConfigIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_SetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  SetDefaultConfigId(argument: _szconfigmanager_SetDefaultConfigIdRequest, callback: grpc.requestCallback<_szconfigmanager_SetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  setDefaultConfigId(argument: _szconfigmanager_SetDefaultConfigIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_SetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  setDefaultConfigId(argument: _szconfigmanager_SetDefaultConfigIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfigmanager_SetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  setDefaultConfigId(argument: _szconfigmanager_SetDefaultConfigIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfigmanager_SetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  setDefaultConfigId(argument: _szconfigmanager_SetDefaultConfigIdRequest, callback: grpc.requestCallback<_szconfigmanager_SetDefaultConfigIdResponse__Output>): grpc.ClientUnaryCall;
  
}

export interface SzConfigManagerHandlers extends grpc.UntypedServiceImplementation {
  AddConfig: grpc.handleUnaryCall<_szconfigmanager_AddConfigRequest__Output, _szconfigmanager_AddConfigResponse>;
  
  GetConfig: grpc.handleUnaryCall<_szconfigmanager_GetConfigRequest__Output, _szconfigmanager_GetConfigResponse>;
  
  GetConfigs: grpc.handleUnaryCall<_szconfigmanager_GetConfigsRequest__Output, _szconfigmanager_GetConfigsResponse>;
  
  GetDefaultConfigId: grpc.handleUnaryCall<_szconfigmanager_GetDefaultConfigIdRequest__Output, _szconfigmanager_GetDefaultConfigIdResponse>;
  
  ReplaceDefaultConfigId: grpc.handleUnaryCall<_szconfigmanager_ReplaceDefaultConfigIdRequest__Output, _szconfigmanager_ReplaceDefaultConfigIdResponse>;
  
  SetDefaultConfigId: grpc.handleUnaryCall<_szconfigmanager_SetDefaultConfigIdRequest__Output, _szconfigmanager_SetDefaultConfigIdResponse>;
  
}

export interface SzConfigManagerDefinition extends grpc.ServiceDefinition {
  AddConfig: MethodDefinition<_szconfigmanager_AddConfigRequest, _szconfigmanager_AddConfigResponse, _szconfigmanager_AddConfigRequest__Output, _szconfigmanager_AddConfigResponse__Output>
  GetConfig: MethodDefinition<_szconfigmanager_GetConfigRequest, _szconfigmanager_GetConfigResponse, _szconfigmanager_GetConfigRequest__Output, _szconfigmanager_GetConfigResponse__Output>
  GetConfigs: MethodDefinition<_szconfigmanager_GetConfigsRequest, _szconfigmanager_GetConfigsResponse, _szconfigmanager_GetConfigsRequest__Output, _szconfigmanager_GetConfigsResponse__Output>
  GetDefaultConfigId: MethodDefinition<_szconfigmanager_GetDefaultConfigIdRequest, _szconfigmanager_GetDefaultConfigIdResponse, _szconfigmanager_GetDefaultConfigIdRequest__Output, _szconfigmanager_GetDefaultConfigIdResponse__Output>
  ReplaceDefaultConfigId: MethodDefinition<_szconfigmanager_ReplaceDefaultConfigIdRequest, _szconfigmanager_ReplaceDefaultConfigIdResponse, _szconfigmanager_ReplaceDefaultConfigIdRequest__Output, _szconfigmanager_ReplaceDefaultConfigIdResponse__Output>
  SetDefaultConfigId: MethodDefinition<_szconfigmanager_SetDefaultConfigIdRequest, _szconfigmanager_SetDefaultConfigIdResponse, _szconfigmanager_SetDefaultConfigIdRequest__Output, _szconfigmanager_SetDefaultConfigIdResponse__Output>
}
