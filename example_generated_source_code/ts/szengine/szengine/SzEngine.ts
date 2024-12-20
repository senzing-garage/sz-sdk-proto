// Original file: szengine.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { AddRecordRequest as _szengine_AddRecordRequest, AddRecordRequest__Output as _szengine_AddRecordRequest__Output } from '../szengine/AddRecordRequest';
import type { AddRecordResponse as _szengine_AddRecordResponse, AddRecordResponse__Output as _szengine_AddRecordResponse__Output } from '../szengine/AddRecordResponse';
import type { CloseExportRequest as _szengine_CloseExportRequest, CloseExportRequest__Output as _szengine_CloseExportRequest__Output } from '../szengine/CloseExportRequest';
import type { CloseExportResponse as _szengine_CloseExportResponse, CloseExportResponse__Output as _szengine_CloseExportResponse__Output } from '../szengine/CloseExportResponse';
import type { CountRedoRecordsRequest as _szengine_CountRedoRecordsRequest, CountRedoRecordsRequest__Output as _szengine_CountRedoRecordsRequest__Output } from '../szengine/CountRedoRecordsRequest';
import type { CountRedoRecordsResponse as _szengine_CountRedoRecordsResponse, CountRedoRecordsResponse__Output as _szengine_CountRedoRecordsResponse__Output } from '../szengine/CountRedoRecordsResponse';
import type { DeleteRecordRequest as _szengine_DeleteRecordRequest, DeleteRecordRequest__Output as _szengine_DeleteRecordRequest__Output } from '../szengine/DeleteRecordRequest';
import type { DeleteRecordResponse as _szengine_DeleteRecordResponse, DeleteRecordResponse__Output as _szengine_DeleteRecordResponse__Output } from '../szengine/DeleteRecordResponse';
import type { ExportCsvEntityReportRequest as _szengine_ExportCsvEntityReportRequest, ExportCsvEntityReportRequest__Output as _szengine_ExportCsvEntityReportRequest__Output } from '../szengine/ExportCsvEntityReportRequest';
import type { ExportCsvEntityReportResponse as _szengine_ExportCsvEntityReportResponse, ExportCsvEntityReportResponse__Output as _szengine_ExportCsvEntityReportResponse__Output } from '../szengine/ExportCsvEntityReportResponse';
import type { ExportJsonEntityReportRequest as _szengine_ExportJsonEntityReportRequest, ExportJsonEntityReportRequest__Output as _szengine_ExportJsonEntityReportRequest__Output } from '../szengine/ExportJsonEntityReportRequest';
import type { ExportJsonEntityReportResponse as _szengine_ExportJsonEntityReportResponse, ExportJsonEntityReportResponse__Output as _szengine_ExportJsonEntityReportResponse__Output } from '../szengine/ExportJsonEntityReportResponse';
import type { FetchNextRequest as _szengine_FetchNextRequest, FetchNextRequest__Output as _szengine_FetchNextRequest__Output } from '../szengine/FetchNextRequest';
import type { FetchNextResponse as _szengine_FetchNextResponse, FetchNextResponse__Output as _szengine_FetchNextResponse__Output } from '../szengine/FetchNextResponse';
import type { FindInterestingEntitiesByEntityIdRequest as _szengine_FindInterestingEntitiesByEntityIdRequest, FindInterestingEntitiesByEntityIdRequest__Output as _szengine_FindInterestingEntitiesByEntityIdRequest__Output } from '../szengine/FindInterestingEntitiesByEntityIdRequest';
import type { FindInterestingEntitiesByEntityIdResponse as _szengine_FindInterestingEntitiesByEntityIdResponse, FindInterestingEntitiesByEntityIdResponse__Output as _szengine_FindInterestingEntitiesByEntityIdResponse__Output } from '../szengine/FindInterestingEntitiesByEntityIdResponse';
import type { FindInterestingEntitiesByRecordIdRequest as _szengine_FindInterestingEntitiesByRecordIdRequest, FindInterestingEntitiesByRecordIdRequest__Output as _szengine_FindInterestingEntitiesByRecordIdRequest__Output } from '../szengine/FindInterestingEntitiesByRecordIdRequest';
import type { FindInterestingEntitiesByRecordIdResponse as _szengine_FindInterestingEntitiesByRecordIdResponse, FindInterestingEntitiesByRecordIdResponse__Output as _szengine_FindInterestingEntitiesByRecordIdResponse__Output } from '../szengine/FindInterestingEntitiesByRecordIdResponse';
import type { FindNetworkByEntityIdRequest as _szengine_FindNetworkByEntityIdRequest, FindNetworkByEntityIdRequest__Output as _szengine_FindNetworkByEntityIdRequest__Output } from '../szengine/FindNetworkByEntityIdRequest';
import type { FindNetworkByEntityIdResponse as _szengine_FindNetworkByEntityIdResponse, FindNetworkByEntityIdResponse__Output as _szengine_FindNetworkByEntityIdResponse__Output } from '../szengine/FindNetworkByEntityIdResponse';
import type { FindNetworkByRecordIdRequest as _szengine_FindNetworkByRecordIdRequest, FindNetworkByRecordIdRequest__Output as _szengine_FindNetworkByRecordIdRequest__Output } from '../szengine/FindNetworkByRecordIdRequest';
import type { FindNetworkByRecordIdResponse as _szengine_FindNetworkByRecordIdResponse, FindNetworkByRecordIdResponse__Output as _szengine_FindNetworkByRecordIdResponse__Output } from '../szengine/FindNetworkByRecordIdResponse';
import type { FindPathByEntityIdRequest as _szengine_FindPathByEntityIdRequest, FindPathByEntityIdRequest__Output as _szengine_FindPathByEntityIdRequest__Output } from '../szengine/FindPathByEntityIdRequest';
import type { FindPathByEntityIdResponse as _szengine_FindPathByEntityIdResponse, FindPathByEntityIdResponse__Output as _szengine_FindPathByEntityIdResponse__Output } from '../szengine/FindPathByEntityIdResponse';
import type { FindPathByRecordIdRequest as _szengine_FindPathByRecordIdRequest, FindPathByRecordIdRequest__Output as _szengine_FindPathByRecordIdRequest__Output } from '../szengine/FindPathByRecordIdRequest';
import type { FindPathByRecordIdResponse as _szengine_FindPathByRecordIdResponse, FindPathByRecordIdResponse__Output as _szengine_FindPathByRecordIdResponse__Output } from '../szengine/FindPathByRecordIdResponse';
import type { GetActiveConfigIdRequest as _szengine_GetActiveConfigIdRequest, GetActiveConfigIdRequest__Output as _szengine_GetActiveConfigIdRequest__Output } from '../szengine/GetActiveConfigIdRequest';
import type { GetActiveConfigIdResponse as _szengine_GetActiveConfigIdResponse, GetActiveConfigIdResponse__Output as _szengine_GetActiveConfigIdResponse__Output } from '../szengine/GetActiveConfigIdResponse';
import type { GetEntityByEntityIdRequest as _szengine_GetEntityByEntityIdRequest, GetEntityByEntityIdRequest__Output as _szengine_GetEntityByEntityIdRequest__Output } from '../szengine/GetEntityByEntityIdRequest';
import type { GetEntityByEntityIdResponse as _szengine_GetEntityByEntityIdResponse, GetEntityByEntityIdResponse__Output as _szengine_GetEntityByEntityIdResponse__Output } from '../szengine/GetEntityByEntityIdResponse';
import type { GetEntityByRecordIdRequest as _szengine_GetEntityByRecordIdRequest, GetEntityByRecordIdRequest__Output as _szengine_GetEntityByRecordIdRequest__Output } from '../szengine/GetEntityByRecordIdRequest';
import type { GetEntityByRecordIdResponse as _szengine_GetEntityByRecordIdResponse, GetEntityByRecordIdResponse__Output as _szengine_GetEntityByRecordIdResponse__Output } from '../szengine/GetEntityByRecordIdResponse';
import type { GetRecordRequest as _szengine_GetRecordRequest, GetRecordRequest__Output as _szengine_GetRecordRequest__Output } from '../szengine/GetRecordRequest';
import type { GetRecordResponse as _szengine_GetRecordResponse, GetRecordResponse__Output as _szengine_GetRecordResponse__Output } from '../szengine/GetRecordResponse';
import type { GetRedoRecordRequest as _szengine_GetRedoRecordRequest, GetRedoRecordRequest__Output as _szengine_GetRedoRecordRequest__Output } from '../szengine/GetRedoRecordRequest';
import type { GetRedoRecordResponse as _szengine_GetRedoRecordResponse, GetRedoRecordResponse__Output as _szengine_GetRedoRecordResponse__Output } from '../szengine/GetRedoRecordResponse';
import type { GetStatsRequest as _szengine_GetStatsRequest, GetStatsRequest__Output as _szengine_GetStatsRequest__Output } from '../szengine/GetStatsRequest';
import type { GetStatsResponse as _szengine_GetStatsResponse, GetStatsResponse__Output as _szengine_GetStatsResponse__Output } from '../szengine/GetStatsResponse';
import type { GetVirtualEntityByRecordIdRequest as _szengine_GetVirtualEntityByRecordIdRequest, GetVirtualEntityByRecordIdRequest__Output as _szengine_GetVirtualEntityByRecordIdRequest__Output } from '../szengine/GetVirtualEntityByRecordIdRequest';
import type { GetVirtualEntityByRecordIdResponse as _szengine_GetVirtualEntityByRecordIdResponse, GetVirtualEntityByRecordIdResponse__Output as _szengine_GetVirtualEntityByRecordIdResponse__Output } from '../szengine/GetVirtualEntityByRecordIdResponse';
import type { HowEntityByEntityIdRequest as _szengine_HowEntityByEntityIdRequest, HowEntityByEntityIdRequest__Output as _szengine_HowEntityByEntityIdRequest__Output } from '../szengine/HowEntityByEntityIdRequest';
import type { HowEntityByEntityIdResponse as _szengine_HowEntityByEntityIdResponse, HowEntityByEntityIdResponse__Output as _szengine_HowEntityByEntityIdResponse__Output } from '../szengine/HowEntityByEntityIdResponse';
import type { PreprocessRecordRequest as _szengine_PreprocessRecordRequest, PreprocessRecordRequest__Output as _szengine_PreprocessRecordRequest__Output } from '../szengine/PreprocessRecordRequest';
import type { PreprocessRecordResponse as _szengine_PreprocessRecordResponse, PreprocessRecordResponse__Output as _szengine_PreprocessRecordResponse__Output } from '../szengine/PreprocessRecordResponse';
import type { PrimeEngineRequest as _szengine_PrimeEngineRequest, PrimeEngineRequest__Output as _szengine_PrimeEngineRequest__Output } from '../szengine/PrimeEngineRequest';
import type { PrimeEngineResponse as _szengine_PrimeEngineResponse, PrimeEngineResponse__Output as _szengine_PrimeEngineResponse__Output } from '../szengine/PrimeEngineResponse';
import type { ProcessRedoRecordRequest as _szengine_ProcessRedoRecordRequest, ProcessRedoRecordRequest__Output as _szengine_ProcessRedoRecordRequest__Output } from '../szengine/ProcessRedoRecordRequest';
import type { ProcessRedoRecordResponse as _szengine_ProcessRedoRecordResponse, ProcessRedoRecordResponse__Output as _szengine_ProcessRedoRecordResponse__Output } from '../szengine/ProcessRedoRecordResponse';
import type { ReevaluateEntityRequest as _szengine_ReevaluateEntityRequest, ReevaluateEntityRequest__Output as _szengine_ReevaluateEntityRequest__Output } from '../szengine/ReevaluateEntityRequest';
import type { ReevaluateEntityResponse as _szengine_ReevaluateEntityResponse, ReevaluateEntityResponse__Output as _szengine_ReevaluateEntityResponse__Output } from '../szengine/ReevaluateEntityResponse';
import type { ReevaluateRecordRequest as _szengine_ReevaluateRecordRequest, ReevaluateRecordRequest__Output as _szengine_ReevaluateRecordRequest__Output } from '../szengine/ReevaluateRecordRequest';
import type { ReevaluateRecordResponse as _szengine_ReevaluateRecordResponse, ReevaluateRecordResponse__Output as _szengine_ReevaluateRecordResponse__Output } from '../szengine/ReevaluateRecordResponse';
import type { ReinitializeRequest as _szengine_ReinitializeRequest, ReinitializeRequest__Output as _szengine_ReinitializeRequest__Output } from '../szengine/ReinitializeRequest';
import type { ReinitializeResponse as _szengine_ReinitializeResponse, ReinitializeResponse__Output as _szengine_ReinitializeResponse__Output } from '../szengine/ReinitializeResponse';
import type { SearchByAttributesRequest as _szengine_SearchByAttributesRequest, SearchByAttributesRequest__Output as _szengine_SearchByAttributesRequest__Output } from '../szengine/SearchByAttributesRequest';
import type { SearchByAttributesResponse as _szengine_SearchByAttributesResponse, SearchByAttributesResponse__Output as _szengine_SearchByAttributesResponse__Output } from '../szengine/SearchByAttributesResponse';
import type { StreamExportCsvEntityReportRequest as _szengine_StreamExportCsvEntityReportRequest, StreamExportCsvEntityReportRequest__Output as _szengine_StreamExportCsvEntityReportRequest__Output } from '../szengine/StreamExportCsvEntityReportRequest';
import type { StreamExportCsvEntityReportResponse as _szengine_StreamExportCsvEntityReportResponse, StreamExportCsvEntityReportResponse__Output as _szengine_StreamExportCsvEntityReportResponse__Output } from '../szengine/StreamExportCsvEntityReportResponse';
import type { StreamExportJsonEntityReportRequest as _szengine_StreamExportJsonEntityReportRequest, StreamExportJsonEntityReportRequest__Output as _szengine_StreamExportJsonEntityReportRequest__Output } from '../szengine/StreamExportJsonEntityReportRequest';
import type { StreamExportJsonEntityReportResponse as _szengine_StreamExportJsonEntityReportResponse, StreamExportJsonEntityReportResponse__Output as _szengine_StreamExportJsonEntityReportResponse__Output } from '../szengine/StreamExportJsonEntityReportResponse';
import type { WhyEntitiesRequest as _szengine_WhyEntitiesRequest, WhyEntitiesRequest__Output as _szengine_WhyEntitiesRequest__Output } from '../szengine/WhyEntitiesRequest';
import type { WhyEntitiesResponse as _szengine_WhyEntitiesResponse, WhyEntitiesResponse__Output as _szengine_WhyEntitiesResponse__Output } from '../szengine/WhyEntitiesResponse';
import type { WhyRecordInEntityRequest as _szengine_WhyRecordInEntityRequest, WhyRecordInEntityRequest__Output as _szengine_WhyRecordInEntityRequest__Output } from '../szengine/WhyRecordInEntityRequest';
import type { WhyRecordInEntityResponse as _szengine_WhyRecordInEntityResponse, WhyRecordInEntityResponse__Output as _szengine_WhyRecordInEntityResponse__Output } from '../szengine/WhyRecordInEntityResponse';
import type { WhyRecordsRequest as _szengine_WhyRecordsRequest, WhyRecordsRequest__Output as _szengine_WhyRecordsRequest__Output } from '../szengine/WhyRecordsRequest';
import type { WhyRecordsResponse as _szengine_WhyRecordsResponse, WhyRecordsResponse__Output as _szengine_WhyRecordsResponse__Output } from '../szengine/WhyRecordsResponse';

export interface SzEngineClient extends grpc.Client {
  AddRecord(argument: _szengine_AddRecordRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_AddRecordResponse__Output>): grpc.ClientUnaryCall;
  AddRecord(argument: _szengine_AddRecordRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_AddRecordResponse__Output>): grpc.ClientUnaryCall;
  AddRecord(argument: _szengine_AddRecordRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_AddRecordResponse__Output>): grpc.ClientUnaryCall;
  AddRecord(argument: _szengine_AddRecordRequest, callback: grpc.requestCallback<_szengine_AddRecordResponse__Output>): grpc.ClientUnaryCall;
  addRecord(argument: _szengine_AddRecordRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_AddRecordResponse__Output>): grpc.ClientUnaryCall;
  addRecord(argument: _szengine_AddRecordRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_AddRecordResponse__Output>): grpc.ClientUnaryCall;
  addRecord(argument: _szengine_AddRecordRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_AddRecordResponse__Output>): grpc.ClientUnaryCall;
  addRecord(argument: _szengine_AddRecordRequest, callback: grpc.requestCallback<_szengine_AddRecordResponse__Output>): grpc.ClientUnaryCall;
  
  CloseExport(argument: _szengine_CloseExportRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_CloseExportResponse__Output>): grpc.ClientUnaryCall;
  CloseExport(argument: _szengine_CloseExportRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_CloseExportResponse__Output>): grpc.ClientUnaryCall;
  CloseExport(argument: _szengine_CloseExportRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_CloseExportResponse__Output>): grpc.ClientUnaryCall;
  CloseExport(argument: _szengine_CloseExportRequest, callback: grpc.requestCallback<_szengine_CloseExportResponse__Output>): grpc.ClientUnaryCall;
  closeExport(argument: _szengine_CloseExportRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_CloseExportResponse__Output>): grpc.ClientUnaryCall;
  closeExport(argument: _szengine_CloseExportRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_CloseExportResponse__Output>): grpc.ClientUnaryCall;
  closeExport(argument: _szengine_CloseExportRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_CloseExportResponse__Output>): grpc.ClientUnaryCall;
  closeExport(argument: _szengine_CloseExportRequest, callback: grpc.requestCallback<_szengine_CloseExportResponse__Output>): grpc.ClientUnaryCall;
  
  CountRedoRecords(argument: _szengine_CountRedoRecordsRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_CountRedoRecordsResponse__Output>): grpc.ClientUnaryCall;
  CountRedoRecords(argument: _szengine_CountRedoRecordsRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_CountRedoRecordsResponse__Output>): grpc.ClientUnaryCall;
  CountRedoRecords(argument: _szengine_CountRedoRecordsRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_CountRedoRecordsResponse__Output>): grpc.ClientUnaryCall;
  CountRedoRecords(argument: _szengine_CountRedoRecordsRequest, callback: grpc.requestCallback<_szengine_CountRedoRecordsResponse__Output>): grpc.ClientUnaryCall;
  countRedoRecords(argument: _szengine_CountRedoRecordsRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_CountRedoRecordsResponse__Output>): grpc.ClientUnaryCall;
  countRedoRecords(argument: _szengine_CountRedoRecordsRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_CountRedoRecordsResponse__Output>): grpc.ClientUnaryCall;
  countRedoRecords(argument: _szengine_CountRedoRecordsRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_CountRedoRecordsResponse__Output>): grpc.ClientUnaryCall;
  countRedoRecords(argument: _szengine_CountRedoRecordsRequest, callback: grpc.requestCallback<_szengine_CountRedoRecordsResponse__Output>): grpc.ClientUnaryCall;
  
  DeleteRecord(argument: _szengine_DeleteRecordRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_DeleteRecordResponse__Output>): grpc.ClientUnaryCall;
  DeleteRecord(argument: _szengine_DeleteRecordRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_DeleteRecordResponse__Output>): grpc.ClientUnaryCall;
  DeleteRecord(argument: _szengine_DeleteRecordRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_DeleteRecordResponse__Output>): grpc.ClientUnaryCall;
  DeleteRecord(argument: _szengine_DeleteRecordRequest, callback: grpc.requestCallback<_szengine_DeleteRecordResponse__Output>): grpc.ClientUnaryCall;
  deleteRecord(argument: _szengine_DeleteRecordRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_DeleteRecordResponse__Output>): grpc.ClientUnaryCall;
  deleteRecord(argument: _szengine_DeleteRecordRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_DeleteRecordResponse__Output>): grpc.ClientUnaryCall;
  deleteRecord(argument: _szengine_DeleteRecordRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_DeleteRecordResponse__Output>): grpc.ClientUnaryCall;
  deleteRecord(argument: _szengine_DeleteRecordRequest, callback: grpc.requestCallback<_szengine_DeleteRecordResponse__Output>): grpc.ClientUnaryCall;
  
  ExportCsvEntityReport(argument: _szengine_ExportCsvEntityReportRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ExportCsvEntityReportResponse__Output>): grpc.ClientUnaryCall;
  ExportCsvEntityReport(argument: _szengine_ExportCsvEntityReportRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_ExportCsvEntityReportResponse__Output>): grpc.ClientUnaryCall;
  ExportCsvEntityReport(argument: _szengine_ExportCsvEntityReportRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ExportCsvEntityReportResponse__Output>): grpc.ClientUnaryCall;
  ExportCsvEntityReport(argument: _szengine_ExportCsvEntityReportRequest, callback: grpc.requestCallback<_szengine_ExportCsvEntityReportResponse__Output>): grpc.ClientUnaryCall;
  exportCsvEntityReport(argument: _szengine_ExportCsvEntityReportRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ExportCsvEntityReportResponse__Output>): grpc.ClientUnaryCall;
  exportCsvEntityReport(argument: _szengine_ExportCsvEntityReportRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_ExportCsvEntityReportResponse__Output>): grpc.ClientUnaryCall;
  exportCsvEntityReport(argument: _szengine_ExportCsvEntityReportRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ExportCsvEntityReportResponse__Output>): grpc.ClientUnaryCall;
  exportCsvEntityReport(argument: _szengine_ExportCsvEntityReportRequest, callback: grpc.requestCallback<_szengine_ExportCsvEntityReportResponse__Output>): grpc.ClientUnaryCall;
  
  ExportJsonEntityReport(argument: _szengine_ExportJsonEntityReportRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ExportJsonEntityReportResponse__Output>): grpc.ClientUnaryCall;
  ExportJsonEntityReport(argument: _szengine_ExportJsonEntityReportRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_ExportJsonEntityReportResponse__Output>): grpc.ClientUnaryCall;
  ExportJsonEntityReport(argument: _szengine_ExportJsonEntityReportRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ExportJsonEntityReportResponse__Output>): grpc.ClientUnaryCall;
  ExportJsonEntityReport(argument: _szengine_ExportJsonEntityReportRequest, callback: grpc.requestCallback<_szengine_ExportJsonEntityReportResponse__Output>): grpc.ClientUnaryCall;
  exportJsonEntityReport(argument: _szengine_ExportJsonEntityReportRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ExportJsonEntityReportResponse__Output>): grpc.ClientUnaryCall;
  exportJsonEntityReport(argument: _szengine_ExportJsonEntityReportRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_ExportJsonEntityReportResponse__Output>): grpc.ClientUnaryCall;
  exportJsonEntityReport(argument: _szengine_ExportJsonEntityReportRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ExportJsonEntityReportResponse__Output>): grpc.ClientUnaryCall;
  exportJsonEntityReport(argument: _szengine_ExportJsonEntityReportRequest, callback: grpc.requestCallback<_szengine_ExportJsonEntityReportResponse__Output>): grpc.ClientUnaryCall;
  
  FetchNext(argument: _szengine_FetchNextRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FetchNextResponse__Output>): grpc.ClientUnaryCall;
  FetchNext(argument: _szengine_FetchNextRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_FetchNextResponse__Output>): grpc.ClientUnaryCall;
  FetchNext(argument: _szengine_FetchNextRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FetchNextResponse__Output>): grpc.ClientUnaryCall;
  FetchNext(argument: _szengine_FetchNextRequest, callback: grpc.requestCallback<_szengine_FetchNextResponse__Output>): grpc.ClientUnaryCall;
  fetchNext(argument: _szengine_FetchNextRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FetchNextResponse__Output>): grpc.ClientUnaryCall;
  fetchNext(argument: _szengine_FetchNextRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_FetchNextResponse__Output>): grpc.ClientUnaryCall;
  fetchNext(argument: _szengine_FetchNextRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FetchNextResponse__Output>): grpc.ClientUnaryCall;
  fetchNext(argument: _szengine_FetchNextRequest, callback: grpc.requestCallback<_szengine_FetchNextResponse__Output>): grpc.ClientUnaryCall;
  
  FindInterestingEntitiesByEntityId(argument: _szengine_FindInterestingEntitiesByEntityIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  FindInterestingEntitiesByEntityId(argument: _szengine_FindInterestingEntitiesByEntityIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  FindInterestingEntitiesByEntityId(argument: _szengine_FindInterestingEntitiesByEntityIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  FindInterestingEntitiesByEntityId(argument: _szengine_FindInterestingEntitiesByEntityIdRequest, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  findInterestingEntitiesByEntityId(argument: _szengine_FindInterestingEntitiesByEntityIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  findInterestingEntitiesByEntityId(argument: _szengine_FindInterestingEntitiesByEntityIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  findInterestingEntitiesByEntityId(argument: _szengine_FindInterestingEntitiesByEntityIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  findInterestingEntitiesByEntityId(argument: _szengine_FindInterestingEntitiesByEntityIdRequest, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  
  FindInterestingEntitiesByRecordId(argument: _szengine_FindInterestingEntitiesByRecordIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  FindInterestingEntitiesByRecordId(argument: _szengine_FindInterestingEntitiesByRecordIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  FindInterestingEntitiesByRecordId(argument: _szengine_FindInterestingEntitiesByRecordIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  FindInterestingEntitiesByRecordId(argument: _szengine_FindInterestingEntitiesByRecordIdRequest, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  findInterestingEntitiesByRecordId(argument: _szengine_FindInterestingEntitiesByRecordIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  findInterestingEntitiesByRecordId(argument: _szengine_FindInterestingEntitiesByRecordIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  findInterestingEntitiesByRecordId(argument: _szengine_FindInterestingEntitiesByRecordIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  findInterestingEntitiesByRecordId(argument: _szengine_FindInterestingEntitiesByRecordIdRequest, callback: grpc.requestCallback<_szengine_FindInterestingEntitiesByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  
  FindNetworkByEntityId(argument: _szengine_FindNetworkByEntityIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindNetworkByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  FindNetworkByEntityId(argument: _szengine_FindNetworkByEntityIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_FindNetworkByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  FindNetworkByEntityId(argument: _szengine_FindNetworkByEntityIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindNetworkByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  FindNetworkByEntityId(argument: _szengine_FindNetworkByEntityIdRequest, callback: grpc.requestCallback<_szengine_FindNetworkByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  findNetworkByEntityId(argument: _szengine_FindNetworkByEntityIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindNetworkByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  findNetworkByEntityId(argument: _szengine_FindNetworkByEntityIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_FindNetworkByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  findNetworkByEntityId(argument: _szengine_FindNetworkByEntityIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindNetworkByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  findNetworkByEntityId(argument: _szengine_FindNetworkByEntityIdRequest, callback: grpc.requestCallback<_szengine_FindNetworkByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  
  FindNetworkByRecordId(argument: _szengine_FindNetworkByRecordIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindNetworkByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  FindNetworkByRecordId(argument: _szengine_FindNetworkByRecordIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_FindNetworkByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  FindNetworkByRecordId(argument: _szengine_FindNetworkByRecordIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindNetworkByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  FindNetworkByRecordId(argument: _szengine_FindNetworkByRecordIdRequest, callback: grpc.requestCallback<_szengine_FindNetworkByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  findNetworkByRecordId(argument: _szengine_FindNetworkByRecordIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindNetworkByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  findNetworkByRecordId(argument: _szengine_FindNetworkByRecordIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_FindNetworkByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  findNetworkByRecordId(argument: _szengine_FindNetworkByRecordIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindNetworkByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  findNetworkByRecordId(argument: _szengine_FindNetworkByRecordIdRequest, callback: grpc.requestCallback<_szengine_FindNetworkByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  
  FindPathByEntityId(argument: _szengine_FindPathByEntityIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindPathByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  FindPathByEntityId(argument: _szengine_FindPathByEntityIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_FindPathByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  FindPathByEntityId(argument: _szengine_FindPathByEntityIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindPathByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  FindPathByEntityId(argument: _szengine_FindPathByEntityIdRequest, callback: grpc.requestCallback<_szengine_FindPathByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  findPathByEntityId(argument: _szengine_FindPathByEntityIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindPathByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  findPathByEntityId(argument: _szengine_FindPathByEntityIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_FindPathByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  findPathByEntityId(argument: _szengine_FindPathByEntityIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindPathByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  findPathByEntityId(argument: _szengine_FindPathByEntityIdRequest, callback: grpc.requestCallback<_szengine_FindPathByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  
  FindPathByRecordId(argument: _szengine_FindPathByRecordIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindPathByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  FindPathByRecordId(argument: _szengine_FindPathByRecordIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_FindPathByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  FindPathByRecordId(argument: _szengine_FindPathByRecordIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindPathByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  FindPathByRecordId(argument: _szengine_FindPathByRecordIdRequest, callback: grpc.requestCallback<_szengine_FindPathByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  findPathByRecordId(argument: _szengine_FindPathByRecordIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindPathByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  findPathByRecordId(argument: _szengine_FindPathByRecordIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_FindPathByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  findPathByRecordId(argument: _szengine_FindPathByRecordIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_FindPathByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  findPathByRecordId(argument: _szengine_FindPathByRecordIdRequest, callback: grpc.requestCallback<_szengine_FindPathByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  
  GetActiveConfigId(argument: _szengine_GetActiveConfigIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetActiveConfigIdResponse__Output>): grpc.ClientUnaryCall;
  GetActiveConfigId(argument: _szengine_GetActiveConfigIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_GetActiveConfigIdResponse__Output>): grpc.ClientUnaryCall;
  GetActiveConfigId(argument: _szengine_GetActiveConfigIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetActiveConfigIdResponse__Output>): grpc.ClientUnaryCall;
  GetActiveConfigId(argument: _szengine_GetActiveConfigIdRequest, callback: grpc.requestCallback<_szengine_GetActiveConfigIdResponse__Output>): grpc.ClientUnaryCall;
  getActiveConfigId(argument: _szengine_GetActiveConfigIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetActiveConfigIdResponse__Output>): grpc.ClientUnaryCall;
  getActiveConfigId(argument: _szengine_GetActiveConfigIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_GetActiveConfigIdResponse__Output>): grpc.ClientUnaryCall;
  getActiveConfigId(argument: _szengine_GetActiveConfigIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetActiveConfigIdResponse__Output>): grpc.ClientUnaryCall;
  getActiveConfigId(argument: _szengine_GetActiveConfigIdRequest, callback: grpc.requestCallback<_szengine_GetActiveConfigIdResponse__Output>): grpc.ClientUnaryCall;
  
  GetEntityByEntityId(argument: _szengine_GetEntityByEntityIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  GetEntityByEntityId(argument: _szengine_GetEntityByEntityIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_GetEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  GetEntityByEntityId(argument: _szengine_GetEntityByEntityIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  GetEntityByEntityId(argument: _szengine_GetEntityByEntityIdRequest, callback: grpc.requestCallback<_szengine_GetEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  getEntityByEntityId(argument: _szengine_GetEntityByEntityIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  getEntityByEntityId(argument: _szengine_GetEntityByEntityIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_GetEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  getEntityByEntityId(argument: _szengine_GetEntityByEntityIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  getEntityByEntityId(argument: _szengine_GetEntityByEntityIdRequest, callback: grpc.requestCallback<_szengine_GetEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  
  GetEntityByRecordId(argument: _szengine_GetEntityByRecordIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  GetEntityByRecordId(argument: _szengine_GetEntityByRecordIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_GetEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  GetEntityByRecordId(argument: _szengine_GetEntityByRecordIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  GetEntityByRecordId(argument: _szengine_GetEntityByRecordIdRequest, callback: grpc.requestCallback<_szengine_GetEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  getEntityByRecordId(argument: _szengine_GetEntityByRecordIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  getEntityByRecordId(argument: _szengine_GetEntityByRecordIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_GetEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  getEntityByRecordId(argument: _szengine_GetEntityByRecordIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  getEntityByRecordId(argument: _szengine_GetEntityByRecordIdRequest, callback: grpc.requestCallback<_szengine_GetEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  
  GetRecord(argument: _szengine_GetRecordRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetRecordResponse__Output>): grpc.ClientUnaryCall;
  GetRecord(argument: _szengine_GetRecordRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_GetRecordResponse__Output>): grpc.ClientUnaryCall;
  GetRecord(argument: _szengine_GetRecordRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetRecordResponse__Output>): grpc.ClientUnaryCall;
  GetRecord(argument: _szengine_GetRecordRequest, callback: grpc.requestCallback<_szengine_GetRecordResponse__Output>): grpc.ClientUnaryCall;
  getRecord(argument: _szengine_GetRecordRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetRecordResponse__Output>): grpc.ClientUnaryCall;
  getRecord(argument: _szengine_GetRecordRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_GetRecordResponse__Output>): grpc.ClientUnaryCall;
  getRecord(argument: _szengine_GetRecordRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetRecordResponse__Output>): grpc.ClientUnaryCall;
  getRecord(argument: _szengine_GetRecordRequest, callback: grpc.requestCallback<_szengine_GetRecordResponse__Output>): grpc.ClientUnaryCall;
  
  GetRedoRecord(argument: _szengine_GetRedoRecordRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  GetRedoRecord(argument: _szengine_GetRedoRecordRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_GetRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  GetRedoRecord(argument: _szengine_GetRedoRecordRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  GetRedoRecord(argument: _szengine_GetRedoRecordRequest, callback: grpc.requestCallback<_szengine_GetRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  getRedoRecord(argument: _szengine_GetRedoRecordRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  getRedoRecord(argument: _szengine_GetRedoRecordRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_GetRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  getRedoRecord(argument: _szengine_GetRedoRecordRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  getRedoRecord(argument: _szengine_GetRedoRecordRequest, callback: grpc.requestCallback<_szengine_GetRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  
  GetStats(argument: _szengine_GetStatsRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  GetStats(argument: _szengine_GetStatsRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  GetStats(argument: _szengine_GetStatsRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  GetStats(argument: _szengine_GetStatsRequest, callback: grpc.requestCallback<_szengine_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  getStats(argument: _szengine_GetStatsRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  getStats(argument: _szengine_GetStatsRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  getStats(argument: _szengine_GetStatsRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  getStats(argument: _szengine_GetStatsRequest, callback: grpc.requestCallback<_szengine_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  
  GetVirtualEntityByRecordId(argument: _szengine_GetVirtualEntityByRecordIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetVirtualEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  GetVirtualEntityByRecordId(argument: _szengine_GetVirtualEntityByRecordIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_GetVirtualEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  GetVirtualEntityByRecordId(argument: _szengine_GetVirtualEntityByRecordIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetVirtualEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  GetVirtualEntityByRecordId(argument: _szengine_GetVirtualEntityByRecordIdRequest, callback: grpc.requestCallback<_szengine_GetVirtualEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  getVirtualEntityByRecordId(argument: _szengine_GetVirtualEntityByRecordIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetVirtualEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  getVirtualEntityByRecordId(argument: _szengine_GetVirtualEntityByRecordIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_GetVirtualEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  getVirtualEntityByRecordId(argument: _szengine_GetVirtualEntityByRecordIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_GetVirtualEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  getVirtualEntityByRecordId(argument: _szengine_GetVirtualEntityByRecordIdRequest, callback: grpc.requestCallback<_szengine_GetVirtualEntityByRecordIdResponse__Output>): grpc.ClientUnaryCall;
  
  HowEntityByEntityId(argument: _szengine_HowEntityByEntityIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_HowEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  HowEntityByEntityId(argument: _szengine_HowEntityByEntityIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_HowEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  HowEntityByEntityId(argument: _szengine_HowEntityByEntityIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_HowEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  HowEntityByEntityId(argument: _szengine_HowEntityByEntityIdRequest, callback: grpc.requestCallback<_szengine_HowEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  howEntityByEntityId(argument: _szengine_HowEntityByEntityIdRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_HowEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  howEntityByEntityId(argument: _szengine_HowEntityByEntityIdRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_HowEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  howEntityByEntityId(argument: _szengine_HowEntityByEntityIdRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_HowEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  howEntityByEntityId(argument: _szengine_HowEntityByEntityIdRequest, callback: grpc.requestCallback<_szengine_HowEntityByEntityIdResponse__Output>): grpc.ClientUnaryCall;
  
  PreprocessRecord(argument: _szengine_PreprocessRecordRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_PreprocessRecordResponse__Output>): grpc.ClientUnaryCall;
  PreprocessRecord(argument: _szengine_PreprocessRecordRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_PreprocessRecordResponse__Output>): grpc.ClientUnaryCall;
  PreprocessRecord(argument: _szengine_PreprocessRecordRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_PreprocessRecordResponse__Output>): grpc.ClientUnaryCall;
  PreprocessRecord(argument: _szengine_PreprocessRecordRequest, callback: grpc.requestCallback<_szengine_PreprocessRecordResponse__Output>): grpc.ClientUnaryCall;
  preprocessRecord(argument: _szengine_PreprocessRecordRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_PreprocessRecordResponse__Output>): grpc.ClientUnaryCall;
  preprocessRecord(argument: _szengine_PreprocessRecordRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_PreprocessRecordResponse__Output>): grpc.ClientUnaryCall;
  preprocessRecord(argument: _szengine_PreprocessRecordRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_PreprocessRecordResponse__Output>): grpc.ClientUnaryCall;
  preprocessRecord(argument: _szengine_PreprocessRecordRequest, callback: grpc.requestCallback<_szengine_PreprocessRecordResponse__Output>): grpc.ClientUnaryCall;
  
  PrimeEngine(argument: _szengine_PrimeEngineRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_PrimeEngineResponse__Output>): grpc.ClientUnaryCall;
  PrimeEngine(argument: _szengine_PrimeEngineRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_PrimeEngineResponse__Output>): grpc.ClientUnaryCall;
  PrimeEngine(argument: _szengine_PrimeEngineRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_PrimeEngineResponse__Output>): grpc.ClientUnaryCall;
  PrimeEngine(argument: _szengine_PrimeEngineRequest, callback: grpc.requestCallback<_szengine_PrimeEngineResponse__Output>): grpc.ClientUnaryCall;
  primeEngine(argument: _szengine_PrimeEngineRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_PrimeEngineResponse__Output>): grpc.ClientUnaryCall;
  primeEngine(argument: _szengine_PrimeEngineRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_PrimeEngineResponse__Output>): grpc.ClientUnaryCall;
  primeEngine(argument: _szengine_PrimeEngineRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_PrimeEngineResponse__Output>): grpc.ClientUnaryCall;
  primeEngine(argument: _szengine_PrimeEngineRequest, callback: grpc.requestCallback<_szengine_PrimeEngineResponse__Output>): grpc.ClientUnaryCall;
  
  ProcessRedoRecord(argument: _szengine_ProcessRedoRecordRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ProcessRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  ProcessRedoRecord(argument: _szengine_ProcessRedoRecordRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_ProcessRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  ProcessRedoRecord(argument: _szengine_ProcessRedoRecordRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ProcessRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  ProcessRedoRecord(argument: _szengine_ProcessRedoRecordRequest, callback: grpc.requestCallback<_szengine_ProcessRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  processRedoRecord(argument: _szengine_ProcessRedoRecordRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ProcessRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  processRedoRecord(argument: _szengine_ProcessRedoRecordRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_ProcessRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  processRedoRecord(argument: _szengine_ProcessRedoRecordRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ProcessRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  processRedoRecord(argument: _szengine_ProcessRedoRecordRequest, callback: grpc.requestCallback<_szengine_ProcessRedoRecordResponse__Output>): grpc.ClientUnaryCall;
  
  ReevaluateEntity(argument: _szengine_ReevaluateEntityRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ReevaluateEntityResponse__Output>): grpc.ClientUnaryCall;
  ReevaluateEntity(argument: _szengine_ReevaluateEntityRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_ReevaluateEntityResponse__Output>): grpc.ClientUnaryCall;
  ReevaluateEntity(argument: _szengine_ReevaluateEntityRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ReevaluateEntityResponse__Output>): grpc.ClientUnaryCall;
  ReevaluateEntity(argument: _szengine_ReevaluateEntityRequest, callback: grpc.requestCallback<_szengine_ReevaluateEntityResponse__Output>): grpc.ClientUnaryCall;
  reevaluateEntity(argument: _szengine_ReevaluateEntityRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ReevaluateEntityResponse__Output>): grpc.ClientUnaryCall;
  reevaluateEntity(argument: _szengine_ReevaluateEntityRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_ReevaluateEntityResponse__Output>): grpc.ClientUnaryCall;
  reevaluateEntity(argument: _szengine_ReevaluateEntityRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ReevaluateEntityResponse__Output>): grpc.ClientUnaryCall;
  reevaluateEntity(argument: _szengine_ReevaluateEntityRequest, callback: grpc.requestCallback<_szengine_ReevaluateEntityResponse__Output>): grpc.ClientUnaryCall;
  
  ReevaluateRecord(argument: _szengine_ReevaluateRecordRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ReevaluateRecordResponse__Output>): grpc.ClientUnaryCall;
  ReevaluateRecord(argument: _szengine_ReevaluateRecordRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_ReevaluateRecordResponse__Output>): grpc.ClientUnaryCall;
  ReevaluateRecord(argument: _szengine_ReevaluateRecordRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ReevaluateRecordResponse__Output>): grpc.ClientUnaryCall;
  ReevaluateRecord(argument: _szengine_ReevaluateRecordRequest, callback: grpc.requestCallback<_szengine_ReevaluateRecordResponse__Output>): grpc.ClientUnaryCall;
  reevaluateRecord(argument: _szengine_ReevaluateRecordRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ReevaluateRecordResponse__Output>): grpc.ClientUnaryCall;
  reevaluateRecord(argument: _szengine_ReevaluateRecordRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_ReevaluateRecordResponse__Output>): grpc.ClientUnaryCall;
  reevaluateRecord(argument: _szengine_ReevaluateRecordRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ReevaluateRecordResponse__Output>): grpc.ClientUnaryCall;
  reevaluateRecord(argument: _szengine_ReevaluateRecordRequest, callback: grpc.requestCallback<_szengine_ReevaluateRecordResponse__Output>): grpc.ClientUnaryCall;
  
  Reinitialize(argument: _szengine_ReinitializeRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  Reinitialize(argument: _szengine_ReinitializeRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  Reinitialize(argument: _szengine_ReinitializeRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  Reinitialize(argument: _szengine_ReinitializeRequest, callback: grpc.requestCallback<_szengine_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  reinitialize(argument: _szengine_ReinitializeRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  reinitialize(argument: _szengine_ReinitializeRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  reinitialize(argument: _szengine_ReinitializeRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  reinitialize(argument: _szengine_ReinitializeRequest, callback: grpc.requestCallback<_szengine_ReinitializeResponse__Output>): grpc.ClientUnaryCall;
  
  SearchByAttributes(argument: _szengine_SearchByAttributesRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_SearchByAttributesResponse__Output>): grpc.ClientUnaryCall;
  SearchByAttributes(argument: _szengine_SearchByAttributesRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_SearchByAttributesResponse__Output>): grpc.ClientUnaryCall;
  SearchByAttributes(argument: _szengine_SearchByAttributesRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_SearchByAttributesResponse__Output>): grpc.ClientUnaryCall;
  SearchByAttributes(argument: _szengine_SearchByAttributesRequest, callback: grpc.requestCallback<_szengine_SearchByAttributesResponse__Output>): grpc.ClientUnaryCall;
  searchByAttributes(argument: _szengine_SearchByAttributesRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_SearchByAttributesResponse__Output>): grpc.ClientUnaryCall;
  searchByAttributes(argument: _szengine_SearchByAttributesRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_SearchByAttributesResponse__Output>): grpc.ClientUnaryCall;
  searchByAttributes(argument: _szengine_SearchByAttributesRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_SearchByAttributesResponse__Output>): grpc.ClientUnaryCall;
  searchByAttributes(argument: _szengine_SearchByAttributesRequest, callback: grpc.requestCallback<_szengine_SearchByAttributesResponse__Output>): grpc.ClientUnaryCall;
  
  StreamExportCsvEntityReport(argument: _szengine_StreamExportCsvEntityReportRequest, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_szengine_StreamExportCsvEntityReportResponse__Output>;
  StreamExportCsvEntityReport(argument: _szengine_StreamExportCsvEntityReportRequest, options?: grpc.CallOptions): grpc.ClientReadableStream<_szengine_StreamExportCsvEntityReportResponse__Output>;
  streamExportCsvEntityReport(argument: _szengine_StreamExportCsvEntityReportRequest, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_szengine_StreamExportCsvEntityReportResponse__Output>;
  streamExportCsvEntityReport(argument: _szengine_StreamExportCsvEntityReportRequest, options?: grpc.CallOptions): grpc.ClientReadableStream<_szengine_StreamExportCsvEntityReportResponse__Output>;
  
  StreamExportJsonEntityReport(argument: _szengine_StreamExportJsonEntityReportRequest, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_szengine_StreamExportJsonEntityReportResponse__Output>;
  StreamExportJsonEntityReport(argument: _szengine_StreamExportJsonEntityReportRequest, options?: grpc.CallOptions): grpc.ClientReadableStream<_szengine_StreamExportJsonEntityReportResponse__Output>;
  streamExportJsonEntityReport(argument: _szengine_StreamExportJsonEntityReportRequest, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_szengine_StreamExportJsonEntityReportResponse__Output>;
  streamExportJsonEntityReport(argument: _szengine_StreamExportJsonEntityReportRequest, options?: grpc.CallOptions): grpc.ClientReadableStream<_szengine_StreamExportJsonEntityReportResponse__Output>;
  
  WhyEntities(argument: _szengine_WhyEntitiesRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_WhyEntitiesResponse__Output>): grpc.ClientUnaryCall;
  WhyEntities(argument: _szengine_WhyEntitiesRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_WhyEntitiesResponse__Output>): grpc.ClientUnaryCall;
  WhyEntities(argument: _szengine_WhyEntitiesRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_WhyEntitiesResponse__Output>): grpc.ClientUnaryCall;
  WhyEntities(argument: _szengine_WhyEntitiesRequest, callback: grpc.requestCallback<_szengine_WhyEntitiesResponse__Output>): grpc.ClientUnaryCall;
  whyEntities(argument: _szengine_WhyEntitiesRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_WhyEntitiesResponse__Output>): grpc.ClientUnaryCall;
  whyEntities(argument: _szengine_WhyEntitiesRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_WhyEntitiesResponse__Output>): grpc.ClientUnaryCall;
  whyEntities(argument: _szengine_WhyEntitiesRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_WhyEntitiesResponse__Output>): grpc.ClientUnaryCall;
  whyEntities(argument: _szengine_WhyEntitiesRequest, callback: grpc.requestCallback<_szengine_WhyEntitiesResponse__Output>): grpc.ClientUnaryCall;
  
  WhyRecordInEntity(argument: _szengine_WhyRecordInEntityRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_WhyRecordInEntityResponse__Output>): grpc.ClientUnaryCall;
  WhyRecordInEntity(argument: _szengine_WhyRecordInEntityRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_WhyRecordInEntityResponse__Output>): grpc.ClientUnaryCall;
  WhyRecordInEntity(argument: _szengine_WhyRecordInEntityRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_WhyRecordInEntityResponse__Output>): grpc.ClientUnaryCall;
  WhyRecordInEntity(argument: _szengine_WhyRecordInEntityRequest, callback: grpc.requestCallback<_szengine_WhyRecordInEntityResponse__Output>): grpc.ClientUnaryCall;
  whyRecordInEntity(argument: _szengine_WhyRecordInEntityRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_WhyRecordInEntityResponse__Output>): grpc.ClientUnaryCall;
  whyRecordInEntity(argument: _szengine_WhyRecordInEntityRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_WhyRecordInEntityResponse__Output>): grpc.ClientUnaryCall;
  whyRecordInEntity(argument: _szengine_WhyRecordInEntityRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_WhyRecordInEntityResponse__Output>): grpc.ClientUnaryCall;
  whyRecordInEntity(argument: _szengine_WhyRecordInEntityRequest, callback: grpc.requestCallback<_szengine_WhyRecordInEntityResponse__Output>): grpc.ClientUnaryCall;
  
  WhyRecords(argument: _szengine_WhyRecordsRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_WhyRecordsResponse__Output>): grpc.ClientUnaryCall;
  WhyRecords(argument: _szengine_WhyRecordsRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_WhyRecordsResponse__Output>): grpc.ClientUnaryCall;
  WhyRecords(argument: _szengine_WhyRecordsRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_WhyRecordsResponse__Output>): grpc.ClientUnaryCall;
  WhyRecords(argument: _szengine_WhyRecordsRequest, callback: grpc.requestCallback<_szengine_WhyRecordsResponse__Output>): grpc.ClientUnaryCall;
  whyRecords(argument: _szengine_WhyRecordsRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_WhyRecordsResponse__Output>): grpc.ClientUnaryCall;
  whyRecords(argument: _szengine_WhyRecordsRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_szengine_WhyRecordsResponse__Output>): grpc.ClientUnaryCall;
  whyRecords(argument: _szengine_WhyRecordsRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_szengine_WhyRecordsResponse__Output>): grpc.ClientUnaryCall;
  whyRecords(argument: _szengine_WhyRecordsRequest, callback: grpc.requestCallback<_szengine_WhyRecordsResponse__Output>): grpc.ClientUnaryCall;
  
}

export interface SzEngineHandlers extends grpc.UntypedServiceImplementation {
  AddRecord: grpc.handleUnaryCall<_szengine_AddRecordRequest__Output, _szengine_AddRecordResponse>;
  
  CloseExport: grpc.handleUnaryCall<_szengine_CloseExportRequest__Output, _szengine_CloseExportResponse>;
  
  CountRedoRecords: grpc.handleUnaryCall<_szengine_CountRedoRecordsRequest__Output, _szengine_CountRedoRecordsResponse>;
  
  DeleteRecord: grpc.handleUnaryCall<_szengine_DeleteRecordRequest__Output, _szengine_DeleteRecordResponse>;
  
  ExportCsvEntityReport: grpc.handleUnaryCall<_szengine_ExportCsvEntityReportRequest__Output, _szengine_ExportCsvEntityReportResponse>;
  
  ExportJsonEntityReport: grpc.handleUnaryCall<_szengine_ExportJsonEntityReportRequest__Output, _szengine_ExportJsonEntityReportResponse>;
  
  FetchNext: grpc.handleUnaryCall<_szengine_FetchNextRequest__Output, _szengine_FetchNextResponse>;
  
  FindInterestingEntitiesByEntityId: grpc.handleUnaryCall<_szengine_FindInterestingEntitiesByEntityIdRequest__Output, _szengine_FindInterestingEntitiesByEntityIdResponse>;
  
  FindInterestingEntitiesByRecordId: grpc.handleUnaryCall<_szengine_FindInterestingEntitiesByRecordIdRequest__Output, _szengine_FindInterestingEntitiesByRecordIdResponse>;
  
  FindNetworkByEntityId: grpc.handleUnaryCall<_szengine_FindNetworkByEntityIdRequest__Output, _szengine_FindNetworkByEntityIdResponse>;
  
  FindNetworkByRecordId: grpc.handleUnaryCall<_szengine_FindNetworkByRecordIdRequest__Output, _szengine_FindNetworkByRecordIdResponse>;
  
  FindPathByEntityId: grpc.handleUnaryCall<_szengine_FindPathByEntityIdRequest__Output, _szengine_FindPathByEntityIdResponse>;
  
  FindPathByRecordId: grpc.handleUnaryCall<_szengine_FindPathByRecordIdRequest__Output, _szengine_FindPathByRecordIdResponse>;
  
  GetActiveConfigId: grpc.handleUnaryCall<_szengine_GetActiveConfigIdRequest__Output, _szengine_GetActiveConfigIdResponse>;
  
  GetEntityByEntityId: grpc.handleUnaryCall<_szengine_GetEntityByEntityIdRequest__Output, _szengine_GetEntityByEntityIdResponse>;
  
  GetEntityByRecordId: grpc.handleUnaryCall<_szengine_GetEntityByRecordIdRequest__Output, _szengine_GetEntityByRecordIdResponse>;
  
  GetRecord: grpc.handleUnaryCall<_szengine_GetRecordRequest__Output, _szengine_GetRecordResponse>;
  
  GetRedoRecord: grpc.handleUnaryCall<_szengine_GetRedoRecordRequest__Output, _szengine_GetRedoRecordResponse>;
  
  GetStats: grpc.handleUnaryCall<_szengine_GetStatsRequest__Output, _szengine_GetStatsResponse>;
  
  GetVirtualEntityByRecordId: grpc.handleUnaryCall<_szengine_GetVirtualEntityByRecordIdRequest__Output, _szengine_GetVirtualEntityByRecordIdResponse>;
  
  HowEntityByEntityId: grpc.handleUnaryCall<_szengine_HowEntityByEntityIdRequest__Output, _szengine_HowEntityByEntityIdResponse>;
  
  PreprocessRecord: grpc.handleUnaryCall<_szengine_PreprocessRecordRequest__Output, _szengine_PreprocessRecordResponse>;
  
  PrimeEngine: grpc.handleUnaryCall<_szengine_PrimeEngineRequest__Output, _szengine_PrimeEngineResponse>;
  
  ProcessRedoRecord: grpc.handleUnaryCall<_szengine_ProcessRedoRecordRequest__Output, _szengine_ProcessRedoRecordResponse>;
  
  ReevaluateEntity: grpc.handleUnaryCall<_szengine_ReevaluateEntityRequest__Output, _szengine_ReevaluateEntityResponse>;
  
  ReevaluateRecord: grpc.handleUnaryCall<_szengine_ReevaluateRecordRequest__Output, _szengine_ReevaluateRecordResponse>;
  
  Reinitialize: grpc.handleUnaryCall<_szengine_ReinitializeRequest__Output, _szengine_ReinitializeResponse>;
  
  SearchByAttributes: grpc.handleUnaryCall<_szengine_SearchByAttributesRequest__Output, _szengine_SearchByAttributesResponse>;
  
  StreamExportCsvEntityReport: grpc.handleServerStreamingCall<_szengine_StreamExportCsvEntityReportRequest__Output, _szengine_StreamExportCsvEntityReportResponse>;
  
  StreamExportJsonEntityReport: grpc.handleServerStreamingCall<_szengine_StreamExportJsonEntityReportRequest__Output, _szengine_StreamExportJsonEntityReportResponse>;
  
  WhyEntities: grpc.handleUnaryCall<_szengine_WhyEntitiesRequest__Output, _szengine_WhyEntitiesResponse>;
  
  WhyRecordInEntity: grpc.handleUnaryCall<_szengine_WhyRecordInEntityRequest__Output, _szengine_WhyRecordInEntityResponse>;
  
  WhyRecords: grpc.handleUnaryCall<_szengine_WhyRecordsRequest__Output, _szengine_WhyRecordsResponse>;
  
}

export interface SzEngineDefinition extends grpc.ServiceDefinition {
  AddRecord: MethodDefinition<_szengine_AddRecordRequest, _szengine_AddRecordResponse, _szengine_AddRecordRequest__Output, _szengine_AddRecordResponse__Output>
  CloseExport: MethodDefinition<_szengine_CloseExportRequest, _szengine_CloseExportResponse, _szengine_CloseExportRequest__Output, _szengine_CloseExportResponse__Output>
  CountRedoRecords: MethodDefinition<_szengine_CountRedoRecordsRequest, _szengine_CountRedoRecordsResponse, _szengine_CountRedoRecordsRequest__Output, _szengine_CountRedoRecordsResponse__Output>
  DeleteRecord: MethodDefinition<_szengine_DeleteRecordRequest, _szengine_DeleteRecordResponse, _szengine_DeleteRecordRequest__Output, _szengine_DeleteRecordResponse__Output>
  ExportCsvEntityReport: MethodDefinition<_szengine_ExportCsvEntityReportRequest, _szengine_ExportCsvEntityReportResponse, _szengine_ExportCsvEntityReportRequest__Output, _szengine_ExportCsvEntityReportResponse__Output>
  ExportJsonEntityReport: MethodDefinition<_szengine_ExportJsonEntityReportRequest, _szengine_ExportJsonEntityReportResponse, _szengine_ExportJsonEntityReportRequest__Output, _szengine_ExportJsonEntityReportResponse__Output>
  FetchNext: MethodDefinition<_szengine_FetchNextRequest, _szengine_FetchNextResponse, _szengine_FetchNextRequest__Output, _szengine_FetchNextResponse__Output>
  FindInterestingEntitiesByEntityId: MethodDefinition<_szengine_FindInterestingEntitiesByEntityIdRequest, _szengine_FindInterestingEntitiesByEntityIdResponse, _szengine_FindInterestingEntitiesByEntityIdRequest__Output, _szengine_FindInterestingEntitiesByEntityIdResponse__Output>
  FindInterestingEntitiesByRecordId: MethodDefinition<_szengine_FindInterestingEntitiesByRecordIdRequest, _szengine_FindInterestingEntitiesByRecordIdResponse, _szengine_FindInterestingEntitiesByRecordIdRequest__Output, _szengine_FindInterestingEntitiesByRecordIdResponse__Output>
  FindNetworkByEntityId: MethodDefinition<_szengine_FindNetworkByEntityIdRequest, _szengine_FindNetworkByEntityIdResponse, _szengine_FindNetworkByEntityIdRequest__Output, _szengine_FindNetworkByEntityIdResponse__Output>
  FindNetworkByRecordId: MethodDefinition<_szengine_FindNetworkByRecordIdRequest, _szengine_FindNetworkByRecordIdResponse, _szengine_FindNetworkByRecordIdRequest__Output, _szengine_FindNetworkByRecordIdResponse__Output>
  FindPathByEntityId: MethodDefinition<_szengine_FindPathByEntityIdRequest, _szengine_FindPathByEntityIdResponse, _szengine_FindPathByEntityIdRequest__Output, _szengine_FindPathByEntityIdResponse__Output>
  FindPathByRecordId: MethodDefinition<_szengine_FindPathByRecordIdRequest, _szengine_FindPathByRecordIdResponse, _szengine_FindPathByRecordIdRequest__Output, _szengine_FindPathByRecordIdResponse__Output>
  GetActiveConfigId: MethodDefinition<_szengine_GetActiveConfigIdRequest, _szengine_GetActiveConfigIdResponse, _szengine_GetActiveConfigIdRequest__Output, _szengine_GetActiveConfigIdResponse__Output>
  GetEntityByEntityId: MethodDefinition<_szengine_GetEntityByEntityIdRequest, _szengine_GetEntityByEntityIdResponse, _szengine_GetEntityByEntityIdRequest__Output, _szengine_GetEntityByEntityIdResponse__Output>
  GetEntityByRecordId: MethodDefinition<_szengine_GetEntityByRecordIdRequest, _szengine_GetEntityByRecordIdResponse, _szengine_GetEntityByRecordIdRequest__Output, _szengine_GetEntityByRecordIdResponse__Output>
  GetRecord: MethodDefinition<_szengine_GetRecordRequest, _szengine_GetRecordResponse, _szengine_GetRecordRequest__Output, _szengine_GetRecordResponse__Output>
  GetRedoRecord: MethodDefinition<_szengine_GetRedoRecordRequest, _szengine_GetRedoRecordResponse, _szengine_GetRedoRecordRequest__Output, _szengine_GetRedoRecordResponse__Output>
  GetStats: MethodDefinition<_szengine_GetStatsRequest, _szengine_GetStatsResponse, _szengine_GetStatsRequest__Output, _szengine_GetStatsResponse__Output>
  GetVirtualEntityByRecordId: MethodDefinition<_szengine_GetVirtualEntityByRecordIdRequest, _szengine_GetVirtualEntityByRecordIdResponse, _szengine_GetVirtualEntityByRecordIdRequest__Output, _szengine_GetVirtualEntityByRecordIdResponse__Output>
  HowEntityByEntityId: MethodDefinition<_szengine_HowEntityByEntityIdRequest, _szengine_HowEntityByEntityIdResponse, _szengine_HowEntityByEntityIdRequest__Output, _szengine_HowEntityByEntityIdResponse__Output>
  PreprocessRecord: MethodDefinition<_szengine_PreprocessRecordRequest, _szengine_PreprocessRecordResponse, _szengine_PreprocessRecordRequest__Output, _szengine_PreprocessRecordResponse__Output>
  PrimeEngine: MethodDefinition<_szengine_PrimeEngineRequest, _szengine_PrimeEngineResponse, _szengine_PrimeEngineRequest__Output, _szengine_PrimeEngineResponse__Output>
  ProcessRedoRecord: MethodDefinition<_szengine_ProcessRedoRecordRequest, _szengine_ProcessRedoRecordResponse, _szengine_ProcessRedoRecordRequest__Output, _szengine_ProcessRedoRecordResponse__Output>
  ReevaluateEntity: MethodDefinition<_szengine_ReevaluateEntityRequest, _szengine_ReevaluateEntityResponse, _szengine_ReevaluateEntityRequest__Output, _szengine_ReevaluateEntityResponse__Output>
  ReevaluateRecord: MethodDefinition<_szengine_ReevaluateRecordRequest, _szengine_ReevaluateRecordResponse, _szengine_ReevaluateRecordRequest__Output, _szengine_ReevaluateRecordResponse__Output>
  Reinitialize: MethodDefinition<_szengine_ReinitializeRequest, _szengine_ReinitializeResponse, _szengine_ReinitializeRequest__Output, _szengine_ReinitializeResponse__Output>
  SearchByAttributes: MethodDefinition<_szengine_SearchByAttributesRequest, _szengine_SearchByAttributesResponse, _szengine_SearchByAttributesRequest__Output, _szengine_SearchByAttributesResponse__Output>
  StreamExportCsvEntityReport: MethodDefinition<_szengine_StreamExportCsvEntityReportRequest, _szengine_StreamExportCsvEntityReportResponse, _szengine_StreamExportCsvEntityReportRequest__Output, _szengine_StreamExportCsvEntityReportResponse__Output>
  StreamExportJsonEntityReport: MethodDefinition<_szengine_StreamExportJsonEntityReportRequest, _szengine_StreamExportJsonEntityReportResponse, _szengine_StreamExportJsonEntityReportRequest__Output, _szengine_StreamExportJsonEntityReportResponse__Output>
  WhyEntities: MethodDefinition<_szengine_WhyEntitiesRequest, _szengine_WhyEntitiesResponse, _szengine_WhyEntitiesRequest__Output, _szengine_WhyEntitiesResponse__Output>
  WhyRecordInEntity: MethodDefinition<_szengine_WhyRecordInEntityRequest, _szengine_WhyRecordInEntityResponse, _szengine_WhyRecordInEntityRequest__Output, _szengine_WhyRecordInEntityResponse__Output>
  WhyRecords: MethodDefinition<_szengine_WhyRecordsRequest, _szengine_WhyRecordsResponse, _szengine_WhyRecordsRequest__Output, _szengine_WhyRecordsResponse__Output>
}
