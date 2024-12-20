// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface FindInterestingEntitiesByRecordIdRequest {
  'dataSourceCode'?: (string);
  'recordId'?: (string);
  'flags'?: (number | string | Long);
}

export interface FindInterestingEntitiesByRecordIdRequest__Output {
  'dataSourceCode'?: (string);
  'recordId'?: (string);
  'flags'?: (Long);
}
