// Original file: szproduct.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { GetLicenseRequest as _szproduct_GetLicenseRequest, GetLicenseRequest__Output as _szproduct_GetLicenseRequest__Output } from '../szproduct/GetLicenseRequest';
import type { GetLicenseResponse as _szproduct_GetLicenseResponse, GetLicenseResponse__Output as _szproduct_GetLicenseResponse__Output } from '../szproduct/GetLicenseResponse';
import type { GetVersionRequest as _szproduct_GetVersionRequest, GetVersionRequest__Output as _szproduct_GetVersionRequest__Output } from '../szproduct/GetVersionRequest';
import type { GetVersionResponse as _szproduct_GetVersionResponse, GetVersionResponse__Output as _szproduct_GetVersionResponse__Output } from '../szproduct/GetVersionResponse';

export interface SzProductClient extends grpc.Client {
  GetLicense(argument: _szproduct_GetLicenseRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szproduct_GetLicenseResponse__Output>): grpc.ClientUnaryCall;
  GetLicense(argument: _szproduct_GetLicenseRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szproduct_GetLicenseResponse__Output>): grpc.ClientUnaryCall;
  GetLicense(argument: _szproduct_GetLicenseRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szproduct_GetLicenseResponse__Output>): grpc.ClientUnaryCall;
  GetLicense(argument: _szproduct_GetLicenseRequest, callback: grpc.requestCallback<_szproduct_GetLicenseResponse__Output>): grpc.ClientUnaryCall;
  getLicense(argument: _szproduct_GetLicenseRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szproduct_GetLicenseResponse__Output>): grpc.ClientUnaryCall;
  getLicense(argument: _szproduct_GetLicenseRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szproduct_GetLicenseResponse__Output>): grpc.ClientUnaryCall;
  getLicense(argument: _szproduct_GetLicenseRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szproduct_GetLicenseResponse__Output>): grpc.ClientUnaryCall;
  getLicense(argument: _szproduct_GetLicenseRequest, callback: grpc.requestCallback<_szproduct_GetLicenseResponse__Output>): grpc.ClientUnaryCall;
  
  GetVersion(argument: _szproduct_GetVersionRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szproduct_GetVersionResponse__Output>): grpc.ClientUnaryCall;
  GetVersion(argument: _szproduct_GetVersionRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szproduct_GetVersionResponse__Output>): grpc.ClientUnaryCall;
  GetVersion(argument: _szproduct_GetVersionRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szproduct_GetVersionResponse__Output>): grpc.ClientUnaryCall;
  GetVersion(argument: _szproduct_GetVersionRequest, callback: grpc.requestCallback<_szproduct_GetVersionResponse__Output>): grpc.ClientUnaryCall;
  getVersion(argument: _szproduct_GetVersionRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szproduct_GetVersionResponse__Output>): grpc.ClientUnaryCall;
  getVersion(argument: _szproduct_GetVersionRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szproduct_GetVersionResponse__Output>): grpc.ClientUnaryCall;
  getVersion(argument: _szproduct_GetVersionRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szproduct_GetVersionResponse__Output>): grpc.ClientUnaryCall;
  getVersion(argument: _szproduct_GetVersionRequest, callback: grpc.requestCallback<_szproduct_GetVersionResponse__Output>): grpc.ClientUnaryCall;
  
}

export interface SzProductHandlers extends grpc.UntypedServiceImplementation {
  GetLicense: grpc.handleUnaryCall<_szproduct_GetLicenseRequest__Output, _szproduct_GetLicenseResponse>;
  
  GetVersion: grpc.handleUnaryCall<_szproduct_GetVersionRequest__Output, _szproduct_GetVersionResponse>;
  
}

export interface SzProductDefinition extends grpc.ServiceDefinition {
  GetLicense: MethodDefinition<_szproduct_GetLicenseRequest, _szproduct_GetLicenseResponse, _szproduct_GetLicenseRequest__Output, _szproduct_GetLicenseResponse__Output>
  GetVersion: MethodDefinition<_szproduct_GetVersionRequest, _szproduct_GetVersionResponse, _szproduct_GetVersionRequest__Output, _szproduct_GetVersionResponse__Output>
}
