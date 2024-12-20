// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface StreamExportCsvEntityReportRequest {
  'csvColumnList'?: (string);
  'flags'?: (number | string | Long);
}

export interface StreamExportCsvEntityReportRequest__Output {
  'csvColumnList'?: (string);
  'flags'?: (Long);
}
