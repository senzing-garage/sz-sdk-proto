// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface AddRecordRequest {
  'dataSourceCode'?: (string);
  'recordId'?: (string);
  'recordDefinition'?: (string);
  'flags'?: (number | string | Long);
}

export interface AddRecordRequest__Output {
  'dataSourceCode'?: (string);
  'recordId'?: (string);
  'recordDefinition'?: (string);
  'flags'?: (Long);
}
