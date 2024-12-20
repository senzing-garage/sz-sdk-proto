// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface ExportJsonEntityReportRequest {
  'flags'?: (number | string | Long);
}

export interface ExportJsonEntityReportRequest__Output {
  'flags'?: (Long);
}
