// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface FindNetworkByEntityIdRequest {
  'entityIds'?: (string);
  'maxDegrees'?: (number | string | Long);
  'buildOutDegrees'?: (number | string | Long);
  'buildOutMaxEntities'?: (number | string | Long);
  'flags'?: (number | string | Long);
}

export interface FindNetworkByEntityIdRequest__Output {
  'entityIds'?: (string);
  'maxDegrees'?: (Long);
  'buildOutDegrees'?: (Long);
  'buildOutMaxEntities'?: (Long);
  'flags'?: (Long);
}
