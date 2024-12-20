// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface WhyEntitiesRequest {
  'entityId1'?: (number | string | Long);
  'entityId2'?: (number | string | Long);
  'flags'?: (number | string | Long);
}

export interface WhyEntitiesRequest__Output {
  'entityId1'?: (Long);
  'entityId2'?: (Long);
  'flags'?: (Long);
}
