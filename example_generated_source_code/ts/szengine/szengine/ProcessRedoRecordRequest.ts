// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface ProcessRedoRecordRequest {
  'redoRecord'?: (string);
  'flags'?: (number | string | Long);
}

export interface ProcessRedoRecordRequest__Output {
  'redoRecord'?: (string);
  'flags'?: (Long);
}
