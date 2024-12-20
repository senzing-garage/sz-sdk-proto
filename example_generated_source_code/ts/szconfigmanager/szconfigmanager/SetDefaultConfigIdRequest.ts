// Original file: szconfigmanager.proto

import type { Long } from '@grpc/proto-loader';

export interface SetDefaultConfigIdRequest {
  'configId'?: (number | string | Long);
}

export interface SetDefaultConfigIdRequest__Output {
  'configId'?: (Long);
}
