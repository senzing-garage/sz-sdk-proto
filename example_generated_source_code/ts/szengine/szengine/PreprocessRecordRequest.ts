// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface PreprocessRecordRequest {
  'recordDefinition'?: (string);
  'flags'?: (number | string | Long);
}

export interface PreprocessRecordRequest__Output {
  'recordDefinition'?: (string);
  'flags'?: (Long);
}
