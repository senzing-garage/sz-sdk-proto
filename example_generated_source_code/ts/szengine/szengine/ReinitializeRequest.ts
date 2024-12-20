// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface ReinitializeRequest {
  'configId'?: (number | string | Long);
}

export interface ReinitializeRequest__Output {
  'configId'?: (Long);
}
