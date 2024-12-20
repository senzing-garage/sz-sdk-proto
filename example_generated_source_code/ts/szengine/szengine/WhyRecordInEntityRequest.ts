// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface WhyRecordInEntityRequest {
  'dataSourceCode'?: (string);
  'recordId'?: (string);
  'flags'?: (number | string | Long);
}

export interface WhyRecordInEntityRequest__Output {
  'dataSourceCode'?: (string);
  'recordId'?: (string);
  'flags'?: (Long);
}
