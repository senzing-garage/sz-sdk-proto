// Original file: szconfig.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { AddDataSourceRequest as _szconfig_AddDataSourceRequest, AddDataSourceRequest__Output as _szconfig_AddDataSourceRequest__Output } from '../szconfig/AddDataSourceRequest';
import type { AddDataSourceResponse as _szconfig_AddDataSourceResponse, AddDataSourceResponse__Output as _szconfig_AddDataSourceResponse__Output } from '../szconfig/AddDataSourceResponse';
import type { CloseConfigRequest as _szconfig_CloseConfigRequest, CloseConfigRequest__Output as _szconfig_CloseConfigRequest__Output } from '../szconfig/CloseConfigRequest';
import type { CloseConfigResponse as _szconfig_CloseConfigResponse, CloseConfigResponse__Output as _szconfig_CloseConfigResponse__Output } from '../szconfig/CloseConfigResponse';
import type { CreateConfigRequest as _szconfig_CreateConfigRequest, CreateConfigRequest__Output as _szconfig_CreateConfigRequest__Output } from '../szconfig/CreateConfigRequest';
import type { CreateConfigResponse as _szconfig_CreateConfigResponse, CreateConfigResponse__Output as _szconfig_CreateConfigResponse__Output } from '../szconfig/CreateConfigResponse';
import type { DeleteDataSourceRequest as _szconfig_DeleteDataSourceRequest, DeleteDataSourceRequest__Output as _szconfig_DeleteDataSourceRequest__Output } from '../szconfig/DeleteDataSourceRequest';
import type { DeleteDataSourceResponse as _szconfig_DeleteDataSourceResponse, DeleteDataSourceResponse__Output as _szconfig_DeleteDataSourceResponse__Output } from '../szconfig/DeleteDataSourceResponse';
import type { ExportConfigRequest as _szconfig_ExportConfigRequest, ExportConfigRequest__Output as _szconfig_ExportConfigRequest__Output } from '../szconfig/ExportConfigRequest';
import type { ExportConfigResponse as _szconfig_ExportConfigResponse, ExportConfigResponse__Output as _szconfig_ExportConfigResponse__Output } from '../szconfig/ExportConfigResponse';
import type { GetDataSourcesRequest as _szconfig_GetDataSourcesRequest, GetDataSourcesRequest__Output as _szconfig_GetDataSourcesRequest__Output } from '../szconfig/GetDataSourcesRequest';
import type { GetDataSourcesResponse as _szconfig_GetDataSourcesResponse, GetDataSourcesResponse__Output as _szconfig_GetDataSourcesResponse__Output } from '../szconfig/GetDataSourcesResponse';
import type { ImportConfigRequest as _szconfig_ImportConfigRequest, ImportConfigRequest__Output as _szconfig_ImportConfigRequest__Output } from '../szconfig/ImportConfigRequest';
import type { ImportConfigResponse as _szconfig_ImportConfigResponse, ImportConfigResponse__Output as _szconfig_ImportConfigResponse__Output } from '../szconfig/ImportConfigResponse';

export interface SzConfigClient extends grpc.Client {
  AddDataSource(argument: _szconfig_AddDataSourceRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_AddDataSourceResponse__Output>): grpc.ClientUnaryCall;
  AddDataSource(argument: _szconfig_AddDataSourceRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfig_AddDataSourceResponse__Output>): grpc.ClientUnaryCall;
  AddDataSource(argument: _szconfig_AddDataSourceRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_AddDataSourceResponse__Output>): grpc.ClientUnaryCall;
  AddDataSource(argument: _szconfig_AddDataSourceRequest, callback: grpc.requestCallback<_szconfig_AddDataSourceResponse__Output>): grpc.ClientUnaryCall;
  addDataSource(argument: _szconfig_AddDataSourceRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_AddDataSourceResponse__Output>): grpc.ClientUnaryCall;
  addDataSource(argument: _szconfig_AddDataSourceRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfig_AddDataSourceResponse__Output>): grpc.ClientUnaryCall;
  addDataSource(argument: _szconfig_AddDataSourceRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_AddDataSourceResponse__Output>): grpc.ClientUnaryCall;
  addDataSource(argument: _szconfig_AddDataSourceRequest, callback: grpc.requestCallback<_szconfig_AddDataSourceResponse__Output>): grpc.ClientUnaryCall;
  
  CloseConfig(argument: _szconfig_CloseConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_CloseConfigResponse__Output>): grpc.ClientUnaryCall;
  CloseConfig(argument: _szconfig_CloseConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfig_CloseConfigResponse__Output>): grpc.ClientUnaryCall;
  CloseConfig(argument: _szconfig_CloseConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_CloseConfigResponse__Output>): grpc.ClientUnaryCall;
  CloseConfig(argument: _szconfig_CloseConfigRequest, callback: grpc.requestCallback<_szconfig_CloseConfigResponse__Output>): grpc.ClientUnaryCall;
  closeConfig(argument: _szconfig_CloseConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_CloseConfigResponse__Output>): grpc.ClientUnaryCall;
  closeConfig(argument: _szconfig_CloseConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfig_CloseConfigResponse__Output>): grpc.ClientUnaryCall;
  closeConfig(argument: _szconfig_CloseConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_CloseConfigResponse__Output>): grpc.ClientUnaryCall;
  closeConfig(argument: _szconfig_CloseConfigRequest, callback: grpc.requestCallback<_szconfig_CloseConfigResponse__Output>): grpc.ClientUnaryCall;
  
  CreateConfig(argument: _szconfig_CreateConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_CreateConfigResponse__Output>): grpc.ClientUnaryCall;
  CreateConfig(argument: _szconfig_CreateConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfig_CreateConfigResponse__Output>): grpc.ClientUnaryCall;
  CreateConfig(argument: _szconfig_CreateConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_CreateConfigResponse__Output>): grpc.ClientUnaryCall;
  CreateConfig(argument: _szconfig_CreateConfigRequest, callback: grpc.requestCallback<_szconfig_CreateConfigResponse__Output>): grpc.ClientUnaryCall;
  createConfig(argument: _szconfig_CreateConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_CreateConfigResponse__Output>): grpc.ClientUnaryCall;
  createConfig(argument: _szconfig_CreateConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfig_CreateConfigResponse__Output>): grpc.ClientUnaryCall;
  createConfig(argument: _szconfig_CreateConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_CreateConfigResponse__Output>): grpc.ClientUnaryCall;
  createConfig(argument: _szconfig_CreateConfigRequest, callback: grpc.requestCallback<_szconfig_CreateConfigResponse__Output>): grpc.ClientUnaryCall;
  
  DeleteDataSource(argument: _szconfig_DeleteDataSourceRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_DeleteDataSourceResponse__Output>): grpc.ClientUnaryCall;
  DeleteDataSource(argument: _szconfig_DeleteDataSourceRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfig_DeleteDataSourceResponse__Output>): grpc.ClientUnaryCall;
  DeleteDataSource(argument: _szconfig_DeleteDataSourceRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_DeleteDataSourceResponse__Output>): grpc.ClientUnaryCall;
  DeleteDataSource(argument: _szconfig_DeleteDataSourceRequest, callback: grpc.requestCallback<_szconfig_DeleteDataSourceResponse__Output>): grpc.ClientUnaryCall;
  deleteDataSource(argument: _szconfig_DeleteDataSourceRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_DeleteDataSourceResponse__Output>): grpc.ClientUnaryCall;
  deleteDataSource(argument: _szconfig_DeleteDataSourceRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfig_DeleteDataSourceResponse__Output>): grpc.ClientUnaryCall;
  deleteDataSource(argument: _szconfig_DeleteDataSourceRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_DeleteDataSourceResponse__Output>): grpc.ClientUnaryCall;
  deleteDataSource(argument: _szconfig_DeleteDataSourceRequest, callback: grpc.requestCallback<_szconfig_DeleteDataSourceResponse__Output>): grpc.ClientUnaryCall;
  
  ExportConfig(argument: _szconfig_ExportConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_ExportConfigResponse__Output>): grpc.ClientUnaryCall;
  ExportConfig(argument: _szconfig_ExportConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfig_ExportConfigResponse__Output>): grpc.ClientUnaryCall;
  ExportConfig(argument: _szconfig_ExportConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_ExportConfigResponse__Output>): grpc.ClientUnaryCall;
  ExportConfig(argument: _szconfig_ExportConfigRequest, callback: grpc.requestCallback<_szconfig_ExportConfigResponse__Output>): grpc.ClientUnaryCall;
  exportConfig(argument: _szconfig_ExportConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_ExportConfigResponse__Output>): grpc.ClientUnaryCall;
  exportConfig(argument: _szconfig_ExportConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfig_ExportConfigResponse__Output>): grpc.ClientUnaryCall;
  exportConfig(argument: _szconfig_ExportConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_ExportConfigResponse__Output>): grpc.ClientUnaryCall;
  exportConfig(argument: _szconfig_ExportConfigRequest, callback: grpc.requestCallback<_szconfig_ExportConfigResponse__Output>): grpc.ClientUnaryCall;
  
  GetDataSources(argument: _szconfig_GetDataSourcesRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_GetDataSourcesResponse__Output>): grpc.ClientUnaryCall;
  GetDataSources(argument: _szconfig_GetDataSourcesRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfig_GetDataSourcesResponse__Output>): grpc.ClientUnaryCall;
  GetDataSources(argument: _szconfig_GetDataSourcesRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_GetDataSourcesResponse__Output>): grpc.ClientUnaryCall;
  GetDataSources(argument: _szconfig_GetDataSourcesRequest, callback: grpc.requestCallback<_szconfig_GetDataSourcesResponse__Output>): grpc.ClientUnaryCall;
  getDataSources(argument: _szconfig_GetDataSourcesRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_GetDataSourcesResponse__Output>): grpc.ClientUnaryCall;
  getDataSources(argument: _szconfig_GetDataSourcesRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfig_GetDataSourcesResponse__Output>): grpc.ClientUnaryCall;
  getDataSources(argument: _szconfig_GetDataSourcesRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_GetDataSourcesResponse__Output>): grpc.ClientUnaryCall;
  getDataSources(argument: _szconfig_GetDataSourcesRequest, callback: grpc.requestCallback<_szconfig_GetDataSourcesResponse__Output>): grpc.ClientUnaryCall;
  
  ImportConfig(argument: _szconfig_ImportConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_ImportConfigResponse__Output>): grpc.ClientUnaryCall;
  ImportConfig(argument: _szconfig_ImportConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfig_ImportConfigResponse__Output>): grpc.ClientUnaryCall;
  ImportConfig(argument: _szconfig_ImportConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_ImportConfigResponse__Output>): grpc.ClientUnaryCall;
  ImportConfig(argument: _szconfig_ImportConfigRequest, callback: grpc.requestCallback<_szconfig_ImportConfigResponse__Output>): grpc.ClientUnaryCall;
  importConfig(argument: _szconfig_ImportConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_ImportConfigResponse__Output>): grpc.ClientUnaryCall;
  importConfig(argument: _szconfig_ImportConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szconfig_ImportConfigResponse__Output>): grpc.ClientUnaryCall;
  importConfig(argument: _szconfig_ImportConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szconfig_ImportConfigResponse__Output>): grpc.ClientUnaryCall;
  importConfig(argument: _szconfig_ImportConfigRequest, callback: grpc.requestCallback<_szconfig_ImportConfigResponse__Output>): grpc.ClientUnaryCall;
  
}

export interface SzConfigHandlers extends grpc.UntypedServiceImplementation {
  AddDataSource: grpc.handleUnaryCall<_szconfig_AddDataSourceRequest__Output, _szconfig_AddDataSourceResponse>;
  
  CloseConfig: grpc.handleUnaryCall<_szconfig_CloseConfigRequest__Output, _szconfig_CloseConfigResponse>;
  
  CreateConfig: grpc.handleUnaryCall<_szconfig_CreateConfigRequest__Output, _szconfig_CreateConfigResponse>;
  
  DeleteDataSource: grpc.handleUnaryCall<_szconfig_DeleteDataSourceRequest__Output, _szconfig_DeleteDataSourceResponse>;
  
  ExportConfig: grpc.handleUnaryCall<_szconfig_ExportConfigRequest__Output, _szconfig_ExportConfigResponse>;
  
  GetDataSources: grpc.handleUnaryCall<_szconfig_GetDataSourcesRequest__Output, _szconfig_GetDataSourcesResponse>;
  
  ImportConfig: grpc.handleUnaryCall<_szconfig_ImportConfigRequest__Output, _szconfig_ImportConfigResponse>;
  
}

export interface SzConfigDefinition extends grpc.ServiceDefinition {
  AddDataSource: MethodDefinition<_szconfig_AddDataSourceRequest, _szconfig_AddDataSourceResponse, _szconfig_AddDataSourceRequest__Output, _szconfig_AddDataSourceResponse__Output>
  CloseConfig: MethodDefinition<_szconfig_CloseConfigRequest, _szconfig_CloseConfigResponse, _szconfig_CloseConfigRequest__Output, _szconfig_CloseConfigResponse__Output>
  CreateConfig: MethodDefinition<_szconfig_CreateConfigRequest, _szconfig_CreateConfigResponse, _szconfig_CreateConfigRequest__Output, _szconfig_CreateConfigResponse__Output>
  DeleteDataSource: MethodDefinition<_szconfig_DeleteDataSourceRequest, _szconfig_DeleteDataSourceResponse, _szconfig_DeleteDataSourceRequest__Output, _szconfig_DeleteDataSourceResponse__Output>
  ExportConfig: MethodDefinition<_szconfig_ExportConfigRequest, _szconfig_ExportConfigResponse, _szconfig_ExportConfigRequest__Output, _szconfig_ExportConfigResponse__Output>
  GetDataSources: MethodDefinition<_szconfig_GetDataSourcesRequest, _szconfig_GetDataSourcesResponse, _szconfig_GetDataSourcesRequest__Output, _szconfig_GetDataSourcesResponse__Output>
  ImportConfig: MethodDefinition<_szconfig_ImportConfigRequest, _szconfig_ImportConfigResponse, _szconfig_ImportConfigRequest__Output, _szconfig_ImportConfigResponse__Output>
}
