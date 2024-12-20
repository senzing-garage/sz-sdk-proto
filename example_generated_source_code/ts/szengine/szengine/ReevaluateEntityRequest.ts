// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface ReevaluateEntityRequest {
  'entityId'?: (number | string | Long);
  'flags'?: (number | string | Long);
}

export interface ReevaluateEntityRequest__Output {
  'entityId'?: (Long);
  'flags'?: (Long);
}
