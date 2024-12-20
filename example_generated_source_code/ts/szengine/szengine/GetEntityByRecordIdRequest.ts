// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface GetEntityByRecordIdRequest {
  'dataSourceCode'?: (string);
  'recordId'?: (string);
  'flags'?: (number | string | Long);
}

export interface GetEntityByRecordIdRequest__Output {
  'dataSourceCode'?: (string);
  'recordId'?: (string);
  'flags'?: (Long);
}
