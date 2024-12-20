// Original file: szconfig.proto

import type { Long } from '@grpc/proto-loader';

export interface GetDataSourcesRequest {
  'configHandle'?: (number | string | Long);
}

export interface GetDataSourcesRequest__Output {
  'configHandle'?: (Long);
}
