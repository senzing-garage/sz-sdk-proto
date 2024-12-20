// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface CloseExportRequest {
  'exportHandle'?: (number | string | Long);
}

export interface CloseExportRequest__Output {
  'exportHandle'?: (Long);
}
