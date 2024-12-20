import type * as grpc from '@grpc/grpc-js';
import type { MessageTypeDefinition } from '@grpc/proto-loader';

import type { SzProductClient as _szproduct_SzProductClient, SzProductDefinition as _szproduct_SzProductDefinition } from './szproduct/SzProduct';

type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  szproduct: {
    GetLicenseRequest: MessageTypeDefinition
    GetLicenseResponse: MessageTypeDefinition
    GetVersionRequest: MessageTypeDefinition
    GetVersionResponse: MessageTypeDefinition
    SzProduct: SubtypeConstructor<typeof grpc.Client, _szproduct_SzProductClient> & { service: _szproduct_SzProductDefinition }
  }
}

