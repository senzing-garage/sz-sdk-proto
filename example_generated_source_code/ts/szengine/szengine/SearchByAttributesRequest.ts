// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface SearchByAttributesRequest {
  'attributes'?: (string);
  'searchProfile'?: (string);
  'flags'?: (number | string | Long);
}

export interface SearchByAttributesRequest__Output {
  'attributes'?: (string);
  'searchProfile'?: (string);
  'flags'?: (Long);
}
