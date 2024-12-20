// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface ExportCsvEntityReportRequest {
  'csvColumnList'?: (string);
  'flags'?: (number | string | Long);
}

export interface ExportCsvEntityReportRequest__Output {
  'csvColumnList'?: (string);
  'flags'?: (Long);
}
