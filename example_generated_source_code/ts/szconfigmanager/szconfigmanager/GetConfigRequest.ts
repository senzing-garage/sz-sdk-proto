// Original file: szconfigmanager.proto

import type { Long } from '@grpc/proto-loader';

export interface GetConfigRequest {
  'configId'?: (number | string | Long);
}

export interface GetConfigRequest__Output {
  'configId'?: (Long);
}
