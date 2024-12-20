// Original file: szconfig.proto

import type { Long } from '@grpc/proto-loader';

export interface CloseConfigRequest {
  'configHandle'?: (number | string | Long);
}

export interface CloseConfigRequest__Output {
  'configHandle'?: (Long);
}
