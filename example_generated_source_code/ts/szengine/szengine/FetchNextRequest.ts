// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface FetchNextRequest {
  'exportHandle'?: (number | string | Long);
}

export interface FetchNextRequest__Output {
  'exportHandle'?: (Long);
}
