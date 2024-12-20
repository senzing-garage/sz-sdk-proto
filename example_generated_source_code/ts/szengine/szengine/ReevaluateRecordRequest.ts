// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface ReevaluateRecordRequest {
  'dataSourceCode'?: (string);
  'recordId'?: (string);
  'flags'?: (number | string | Long);
}

export interface ReevaluateRecordRequest__Output {
  'dataSourceCode'?: (string);
  'recordId'?: (string);
  'flags'?: (Long);
}
