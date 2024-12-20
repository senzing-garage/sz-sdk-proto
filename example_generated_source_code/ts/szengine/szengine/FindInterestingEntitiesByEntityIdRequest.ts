// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface FindInterestingEntitiesByEntityIdRequest {
  'entityId'?: (number | string | Long);
  'flags'?: (number | string | Long);
}

export interface FindInterestingEntitiesByEntityIdRequest__Output {
  'entityId'?: (Long);
  'flags'?: (Long);
}
