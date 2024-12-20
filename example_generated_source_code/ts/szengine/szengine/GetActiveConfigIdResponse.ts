// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface GetActiveConfigIdResponse {
  'result'?: (number | string | Long);
}

export interface GetActiveConfigIdResponse__Output {
  'result'?: (Long);
}
