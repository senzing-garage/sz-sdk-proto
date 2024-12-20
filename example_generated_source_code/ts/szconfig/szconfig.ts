import type * as grpc from '@grpc/grpc-js';
import type { MessageTypeDefinition } from '@grpc/proto-loader';

import type { SzConfigClient as _szconfig_SzConfigClient, SzConfigDefinition as _szconfig_SzConfigDefinition } from './szconfig/SzConfig';

type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  szconfig: {
    AddDataSourceRequest: MessageTypeDefinition
    AddDataSourceResponse: MessageTypeDefinition
    CloseConfigRequest: MessageTypeDefinition
    CloseConfigResponse: MessageTypeDefinition
    CreateConfigRequest: MessageTypeDefinition
    CreateConfigResponse: MessageTypeDefinition
    DeleteDataSourceRequest: MessageTypeDefinition
    DeleteDataSourceResponse: MessageTypeDefinition
    ExportConfigRequest: MessageTypeDefinition
    ExportConfigResponse: MessageTypeDefinition
    GetDataSourcesRequest: MessageTypeDefinition
    GetDataSourcesResponse: MessageTypeDefinition
    ImportConfigRequest: MessageTypeDefinition
    ImportConfigResponse: MessageTypeDefinition
    SzConfig: SubtypeConstructor<typeof grpc.Client, _szconfig_SzConfigClient> & { service: _szconfig_SzConfigDefinition }
  }
}

