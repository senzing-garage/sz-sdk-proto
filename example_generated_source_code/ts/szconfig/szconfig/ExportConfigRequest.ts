// Original file: szconfig.proto

import type { Long } from '@grpc/proto-loader';

export interface ExportConfigRequest {
  'configHandle'?: (number | string | Long);
}

export interface ExportConfigRequest__Output {
  'configHandle'?: (Long);
}
