import type * as grpc from '@grpc/grpc-js';
import type { MessageTypeDefinition } from '@grpc/proto-loader';

import type { SzConfigManagerClient as _szconfigmanager_SzConfigManagerClient, SzConfigManagerDefinition as _szconfigmanager_SzConfigManagerDefinition } from './szconfigmanager/SzConfigManager';

type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  szconfigmanager: {
    AddConfigRequest: MessageTypeDefinition
    AddConfigResponse: MessageTypeDefinition
    GetConfigRequest: MessageTypeDefinition
    GetConfigResponse: MessageTypeDefinition
    GetConfigsRequest: MessageTypeDefinition
    GetConfigsResponse: MessageTypeDefinition
    GetDefaultConfigIdRequest: MessageTypeDefinition
    GetDefaultConfigIdResponse: MessageTypeDefinition
    ReplaceDefaultConfigIdRequest: MessageTypeDefinition
    ReplaceDefaultConfigIdResponse: MessageTypeDefinition
    SetDefaultConfigIdRequest: MessageTypeDefinition
    SetDefaultConfigIdResponse: MessageTypeDefinition
    SzConfigManager: SubtypeConstructor<typeof grpc.Client, _szconfigmanager_SzConfigManagerClient> & { service: _szconfigmanager_SzConfigManagerDefinition }
  }
}

