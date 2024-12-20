import type * as grpc from '@grpc/grpc-js';
import type { MessageTypeDefinition } from '@grpc/proto-loader';

import type { SzDiagnosticClient as _szdiagnostic_SzDiagnosticClient, SzDiagnosticDefinition as _szdiagnostic_SzDiagnosticDefinition } from './szdiagnostic/SzDiagnostic';

type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  szdiagnostic: {
    CheckDatastorePerformanceRequest: MessageTypeDefinition
    CheckDatastorePerformanceResponse: MessageTypeDefinition
    GetDatastoreInfoRequest: MessageTypeDefinition
    GetDatastoreInfoResponse: MessageTypeDefinition
    GetFeatureRequest: MessageTypeDefinition
    GetFeatureResponse: MessageTypeDefinition
    PurgeRepositoryRequest: MessageTypeDefinition
    PurgeRepositoryResponse: MessageTypeDefinition
    ReinitializeRequest: MessageTypeDefinition
    ReinitializeResponse: MessageTypeDefinition
    SzDiagnostic: SubtypeConstructor<typeof grpc.Client, _szdiagnostic_SzDiagnosticClient> & { service: _szdiagnostic_SzDiagnosticDefinition }
  }
}

