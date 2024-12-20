// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface FindPathByEntityIdRequest {
  'startEntityId'?: (number | string | Long);
  'endEntityId'?: (number | string | Long);
  'maxDegrees'?: (number | string | Long);
  'avoidEntityIds'?: (string);
  'requiredDataSources'?: (string);
  'flags'?: (number | string | Long);
}

export interface FindPathByEntityIdRequest__Output {
  'startEntityId'?: (Long);
  'endEntityId'?: (Long);
  'maxDegrees'?: (Long);
  'avoidEntityIds'?: (string);
  'requiredDataSources'?: (string);
  'flags'?: (Long);
}
