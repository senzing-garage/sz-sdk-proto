// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface DeleteRecordRequest {
  'dataSourceCode'?: (string);
  'recordId'?: (string);
  'flags'?: (number | string | Long);
}

export interface DeleteRecordRequest__Output {
  'dataSourceCode'?: (string);
  'recordId'?: (string);
  'flags'?: (Long);
}
