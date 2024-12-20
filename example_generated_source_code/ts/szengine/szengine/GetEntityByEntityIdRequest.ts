// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface GetEntityByEntityIdRequest {
  'entityId'?: (number | string | Long);
  'flags'?: (number | string | Long);
}

export interface GetEntityByEntityIdRequest__Output {
  'entityId'?: (Long);
  'flags'?: (Long);
}
