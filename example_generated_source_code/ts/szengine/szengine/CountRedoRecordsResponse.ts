// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface CountRedoRecordsResponse {
  'result'?: (number | string | Long);
}

export interface CountRedoRecordsResponse__Output {
  'result'?: (Long);
}
