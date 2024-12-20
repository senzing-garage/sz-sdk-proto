// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface HowEntityByEntityIdRequest {
  'entityId'?: (number | string | Long);
  'flags'?: (number | string | Long);
}

export interface HowEntityByEntityIdRequest__Output {
  'entityId'?: (Long);
  'flags'?: (Long);
}
