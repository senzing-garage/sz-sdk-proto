import type * as grpc from '@grpc/grpc-js';
import type { MessageTypeDefinition } from '@grpc/proto-loader';

import type { SzEngineClient as _szengine_SzEngineClient, SzEngineDefinition as _szengine_SzEngineDefinition } from './szengine/SzEngine';

type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  szengine: {
    AddRecordRequest: MessageTypeDefinition
    AddRecordResponse: MessageTypeDefinition
    CloseExportRequest: MessageTypeDefinition
    CloseExportResponse: MessageTypeDefinition
    CountRedoRecordsRequest: MessageTypeDefinition
    CountRedoRecordsResponse: MessageTypeDefinition
    DeleteRecordRequest: MessageTypeDefinition
    DeleteRecordResponse: MessageTypeDefinition
    ExportCsvEntityReportRequest: MessageTypeDefinition
    ExportCsvEntityReportResponse: MessageTypeDefinition
    ExportJsonEntityReportRequest: MessageTypeDefinition
    ExportJsonEntityReportResponse: MessageTypeDefinition
    FetchNextRequest: MessageTypeDefinition
    FetchNextResponse: MessageTypeDefinition
    FindInterestingEntitiesByEntityIdRequest: MessageTypeDefinition
    FindInterestingEntitiesByEntityIdResponse: MessageTypeDefinition
    FindInterestingEntitiesByRecordIdRequest: MessageTypeDefinition
    FindInterestingEntitiesByRecordIdResponse: MessageTypeDefinition
    FindNetworkByEntityIdRequest: MessageTypeDefinition
    FindNetworkByEntityIdResponse: MessageTypeDefinition
    FindNetworkByRecordIdRequest: MessageTypeDefinition
    FindNetworkByRecordIdResponse: MessageTypeDefinition
    FindPathByEntityIdRequest: MessageTypeDefinition
    FindPathByEntityIdResponse: MessageTypeDefinition
    FindPathByRecordIdRequest: MessageTypeDefinition
    FindPathByRecordIdResponse: MessageTypeDefinition
    GetActiveConfigIdRequest: MessageTypeDefinition
    GetActiveConfigIdResponse: MessageTypeDefinition
    GetEntityByEntityIdRequest: MessageTypeDefinition
    GetEntityByEntityIdResponse: MessageTypeDefinition
    GetEntityByRecordIdRequest: MessageTypeDefinition
    GetEntityByRecordIdResponse: MessageTypeDefinition
    GetRecordRequest: MessageTypeDefinition
    GetRecordResponse: MessageTypeDefinition
    GetRedoRecordRequest: MessageTypeDefinition
    GetRedoRecordResponse: MessageTypeDefinition
    GetStatsRequest: MessageTypeDefinition
    GetStatsResponse: MessageTypeDefinition
    GetVirtualEntityByRecordIdRequest: MessageTypeDefinition
    GetVirtualEntityByRecordIdResponse: MessageTypeDefinition
    HowEntityByEntityIdRequest: MessageTypeDefinition
    HowEntityByEntityIdResponse: MessageTypeDefinition
    PreprocessRecordRequest: MessageTypeDefinition
    PreprocessRecordResponse: MessageTypeDefinition
    PrimeEngineRequest: MessageTypeDefinition
    PrimeEngineResponse: MessageTypeDefinition
    ProcessRedoRecordRequest: MessageTypeDefinition
    ProcessRedoRecordResponse: MessageTypeDefinition
    ReevaluateEntityRequest: MessageTypeDefinition
    ReevaluateEntityResponse: MessageTypeDefinition
    ReevaluateRecordRequest: MessageTypeDefinition
    ReevaluateRecordResponse: MessageTypeDefinition
    ReinitializeRequest: MessageTypeDefinition
    ReinitializeResponse: MessageTypeDefinition
    SearchByAttributesRequest: MessageTypeDefinition
    SearchByAttributesResponse: MessageTypeDefinition
    StreamExportCsvEntityReportRequest: MessageTypeDefinition
    StreamExportCsvEntityReportResponse: MessageTypeDefinition
    StreamExportJsonEntityReportRequest: MessageTypeDefinition
    StreamExportJsonEntityReportResponse: MessageTypeDefinition
    SzEngine: SubtypeConstructor<typeof grpc.Client, _szengine_SzEngineClient> & { service: _szengine_SzEngineDefinition }
    WhyEntitiesRequest: MessageTypeDefinition
    WhyEntitiesResponse: MessageTypeDefinition
    WhyRecordInEntityRequest: MessageTypeDefinition
    WhyRecordInEntityResponse: MessageTypeDefinition
    WhyRecordsRequest: MessageTypeDefinition
    WhyRecordsResponse: MessageTypeDefinition
  }
}

