// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface GetRecordRequest {
  'dataSourceCode'?: (string);
  'recordId'?: (string);
  'flags'?: (number | string | Long);
}

export interface GetRecordRequest__Output {
  'dataSourceCode'?: (string);
  'recordId'?: (string);
  'flags'?: (Long);
}
