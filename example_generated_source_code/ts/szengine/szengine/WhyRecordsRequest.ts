// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface WhyRecordsRequest {
  'dataSourceCode1'?: (string);
  'recordId1'?: (string);
  'dataSourceCode2'?: (string);
  'recordId2'?: (string);
  'flags'?: (number | string | Long);
}

export interface WhyRecordsRequest__Output {
  'dataSourceCode1'?: (string);
  'recordId1'?: (string);
  'dataSourceCode2'?: (string);
  'recordId2'?: (string);
  'flags'?: (Long);
}
