// Original file: szdiagnostic.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { CheckDatastorePerformanceRequest as _szdiagnostic_CheckDatastorePerformanceRequest, CheckDatastorePerformanceRequest__Output as _szdiagnostic_CheckDatastorePerformanceRequest__Output } from '../szdiagnostic/CheckDatastorePerformanceRequest';
import type { CheckDatastorePerformanceResponse as _szdiagnostic_CheckDatastorePerformanceResponse, CheckDatastorePerformanceResponse__Output as _szdiagnostic_CheckDatastorePerformanceResponse__Output } from '../szdiagnostic/CheckDatastorePerformanceResponse';
import type { GetDatastoreInfoRequest as _szdiagnostic_GetDatastoreInfoRequest, GetDatastoreInfoRequest__Output as _szdiagnostic_GetDatastoreInfoRequest__Output } from '../szdiagnostic/GetDatastoreInfoRequest';
import type { GetDatastoreInfoResponse as _szdiagnostic_GetDatastoreInfoResponse, GetDatastoreInfoResponse__Output as _szdiagnostic_GetDatastoreInfoResponse__Output } from '../szdiagnostic/GetDatastoreInfoResponse';
import type { GetFeatureRequest as _szdiagnostic_GetFeatureRequest, GetFeatureRequest__Output as _szdiagnostic_GetFeatureRequest__Output } from '../szdiagnostic/GetFeatureRequest';
import type { GetFeatureResponse as _szdiagnostic_GetFeatureResponse, GetFeatureResponse__Output as _szdiagnostic_GetFeatureResponse__Output } from '../szdiagnostic/GetFeatureResponse';
import type { PurgeRepositoryRequest as _szdiagnostic_PurgeRepositoryRequest, PurgeRepositoryRequest__Output as _szdiagnostic_PurgeRepositoryRequest__Output } from '../szdiagnostic/PurgeRepositoryRequest';
import type { PurgeRepositoryResponse as _szdiagnostic_PurgeRepositoryResponse, PurgeRepositoryResponse__Output as _szdiagnostic_PurgeRepositoryResponse__Output } from '../szdiagnostic/PurgeRepositoryResponse';
import type { ReinitializeRequest as _szdiagnostic_ReinitializeRequest, ReinitializeRequest__Output as _szdiagnostic_ReinitializeRequest__Output } from '../szdiagnostic/ReinitializeRequest';
import type { ReinitializeResponse as _szdiagnostic_ReinitializeResponse, ReinitializeResponse__Output as _szdiagnostic_ReinitializeResponse__Output } from '../szdiagnostic/ReinitializeResponse';

export interface SzDiagnosticClient extends grpc.Client {
  CheckDatastorePerformance(argument: _szdiagnostic_CheckDatastorePerformanceRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_CheckDatastorePerformanceResponse__Output>): grpc.ClientUnaryCall;
  CheckDatastorePerformance(argument: _szdiagnostic_CheckDatastorePerformanceRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szdiagnostic_CheckDatastorePerformanceResponse__Output>): grpc.ClientUnaryCall;
  CheckDatastorePerformance(argument: _szdiagnostic_CheckDatastorePerformanceRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_CheckDatastorePerformanceResponse__Output>): grpc.ClientUnaryCall;
  CheckDatastorePerformance(argument: _szdiagnostic_CheckDatastorePerformanceRequest, callback: grpc.requestCallback<_szdiagnostic_CheckDatastorePerformanceResponse__Output>): grpc.ClientUnaryCall;
  checkDatastorePerformance(argument: _szdiagnostic_CheckDatastorePerformanceRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_CheckDatastorePerformanceResponse__Output>): grpc.ClientUnaryCall;
  checkDatastorePerformance(argument: _szdiagnostic_CheckDatastorePerformanceRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szdiagnostic_CheckDatastorePerformanceResponse__Output>): grpc.ClientUnaryCall;
  checkDatastorePerformance(argument: _szdiagnostic_CheckDatastorePerformanceRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_CheckDatastorePerformanceResponse__Output>): grpc.ClientUnaryCall;
  checkDatastorePerformance(argument: _szdiagnostic_CheckDatastorePerformanceRequest, callback: grpc.requestCallback<_szdiagnostic_CheckDatastorePerformanceResponse__Output>): grpc.ClientUnaryCall;
  
  GetDatastoreInfo(argument: _szdiagnostic_GetDatastoreInfoRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_GetDatastoreInfoResponse__Output>): grpc.ClientUnaryCall;
  GetDatastoreInfo(argument: _szdiagnostic_GetDatastoreInfoRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szdiagnostic_GetDatastoreInfoResponse__Output>): grpc.ClientUnaryCall;
  GetDatastoreInfo(argument: _szdiagnostic_GetDatastoreInfoRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_GetDatastoreInfoResponse__Output>): grpc.ClientUnaryCall;
  GetDatastoreInfo(argument: _szdiagnostic_GetDatastoreInfoRequest, callback: grpc.requestCallback<_szdiagnostic_GetDatastoreInfoResponse__Output>): grpc.ClientUnaryCall;
  getDatastoreInfo(argument: _szdiagnostic_GetDatastoreInfoRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_GetDatastoreInfoResponse__Output>): grpc.ClientUnaryCall;
  getDatastoreInfo(argument: _szdiagnostic_GetDatastoreInfoRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szdiagnostic_GetDatastoreInfoResponse__Output>): grpc.ClientUnaryCall;
  getDatastoreInfo(argument: _szdiagnostic_GetDatastoreInfoRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_GetDatastoreInfoResponse__Output>): grpc.ClientUnaryCall;
  getDatastoreInfo(argument: _szdiagnostic_GetDatastoreInfoRequest, callback: grpc.requestCallback<_szdiagnostic_GetDatastoreInfoResponse__Output>): grpc.ClientUnaryCall;
  
  GetFeature(argument: _szdiagnostic_GetFeatureRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_GetFeatureResponse__Output>): grpc.ClientUnaryCall;
  GetFeature(argument: _szdiagnostic_GetFeatureRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szdiagnostic_GetFeatureResponse__Output>): grpc.ClientUnaryCall;
  GetFeature(argument: _szdiagnostic_GetFeatureRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_GetFeatureResponse__Output>): grpc.ClientUnaryCall;
  GetFeature(argument: _szdiagnostic_GetFeatureRequest, callback: grpc.requestCallback<_szdiagnostic_GetFeatureResponse__Output>): grpc.ClientUnaryCall;
  getFeature(argument: _szdiagnostic_GetFeatureRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_GetFeatureResponse__Output>): grpc.ClientUnaryCall;
  getFeature(argument: _szdiagnostic_GetFeatureRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szdiagnostic_GetFeatureResponse__Output>): grpc.ClientUnaryCall;
  getFeature(argument: _szdiagnostic_GetFeatureRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_GetFeatureResponse__Output>): grpc.ClientUnaryCall;
  getFeature(argument: _szdiagnostic_GetFeatureRequest, callback: grpc.requestCallback<_szdiagnostic_GetFeatureResponse__Output>): grpc.ClientUnaryCall;
  
  PurgeRepository(argument: _szdiagnostic_PurgeRepositoryRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_PurgeRepositoryResponse__Output>): grpc.ClientUnaryCall;
  PurgeRepository(argument: _szdiagnostic_PurgeRepositoryRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szdiagnostic_PurgeRepositoryResponse__Output>): grpc.ClientUnaryCall;
  PurgeRepository(argument: _szdiagnostic_PurgeRepositoryRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_PurgeRepositoryResponse__Output>): grpc.ClientUnaryCall;
  PurgeRepository(argument: _szdiagnostic_PurgeRepositoryRequest, callback: grpc.requestCallback<_szdiagnostic_PurgeRepositoryResponse__Output>): grpc.ClientUnaryCall;
  purgeRepository(argument: _szdiagnostic_PurgeRepositoryRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_PurgeRepositoryResponse__Output>): grpc.ClientUnaryCall;
  purgeRepository(argument: _szdiagnostic_PurgeRepositoryRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szdiagnostic_PurgeRepositoryResponse__Output>): grpc.ClientUnaryCall;
  purgeRepository(argument: _szdiagnostic_PurgeRepositoryRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_PurgeRepositoryResponse__Output>): grpc.ClientUnaryCall;
  purgeRepository(argument: _szdiagnostic_PurgeRepositoryRequest, callback: grpc.requestCallback<_szdiagnostic_PurgeRepositoryResponse__Output>): grpc.ClientUnaryCall;
  
  Reinitialize(argument: _szdiagnostic_ReinitializeRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  Reinitialize(argument: _szdiagnostic_ReinitializeRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szdiagnostic_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  Reinitialize(argument: _szdiagnostic_ReinitializeRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  Reinitialize(argument: _szdiagnostic_ReinitializeRequest, callback: grpc.requestCallback<_szdiagnostic_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  reinitialize(argument: _szdiagnostic_ReinitializeRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  reinitialize(argument: _szdiagnostic_ReinitializeRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szdiagnostic_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  reinitialize(argument: _szdiagnostic_ReinitializeRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szdiagnostic_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  reinitialize(argument: _szdiagnostic_ReinitializeRequest, callback: grpc.requestCallback<_szdiagnostic_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  
}

export interface SzDiagnosticHandlers extends grpc.UntypedServiceImplementation {
  CheckDatastorePerformance: grpc.handleUnaryCall<_szdiagnostic_CheckDatastorePerformanceRequest__Output, _szdiagnostic_CheckDatastorePerformanceResponse>;
  
  GetDatastoreInfo: grpc.handleUnaryCall<_szdiagnostic_GetDatastoreInfoRequest__Output, _szdiagnostic_GetDatastoreInfoResponse>;
  
  GetFeature: grpc.handleUnaryCall<_szdiagnostic_GetFeatureRequest__Output, _szdiagnostic_GetFeatureResponse>;
  
  PurgeRepository: grpc.handleUnaryCall<_szdiagnostic_PurgeRepositoryRequest__Output, _szdiagnostic_PurgeRepositoryResponse>;
  
  Reinitialize: grpc.handleUnaryCall<_szdiagnostic_ReinitializeRequest__Output, _szdiagnostic_ReinitializeResponse>;
  
}

export interface SzDiagnosticDefinition extends grpc.ServiceDefinition {
  CheckDatastorePerformance: MethodDefinition<_szdiagnostic_CheckDatastorePerformanceRequest, _szdiagnostic_CheckDatastorePerformanceResponse, _szdiagnostic_CheckDatastorePerformanceRequest__Output, _szdiagnostic_CheckDatastorePerformanceResponse__Output>
  GetDatastoreInfo: MethodDefinition<_szdiagnostic_GetDatastoreInfoRequest, _szdiagnostic_GetDatastoreInfoResponse, _szdiagnostic_GetDatastoreInfoRequest__Output, _szdiagnostic_GetDatastoreInfoResponse__Output>
  GetFeature: MethodDefinition<_szdiagnostic_GetFeatureRequest, _szdiagnostic_GetFeatureResponse, _szdiagnostic_GetFeatureRequest__Output, _szdiagnostic_GetFeatureResponse__Output>
  PurgeRepository: MethodDefinition<_szdiagnostic_PurgeRepositoryRequest, _szdiagnostic_PurgeRepositoryResponse, _szdiagnostic_PurgeRepositoryRequest__Output, _szdiagnostic_PurgeRepositoryResponse__Output>
  Reinitialize: MethodDefinition<_szdiagnostic_ReinitializeRequest, _szdiagnostic_ReinitializeResponse, _szdiagnostic_ReinitializeRequest__Output, _szdiagnostic_ReinitializeResponse__Output>
}
