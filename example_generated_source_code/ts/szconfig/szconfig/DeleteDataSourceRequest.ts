// Original file: szconfig.proto

import type { Long } from '@grpc/proto-loader';

export interface DeleteDataSourceRequest {
  'configHandle'?: (number | string | Long);
  'dataSourceCode'?: (string);
}

export interface DeleteDataSourceRequest__Output {
  'configHandle'?: (Long);
  'dataSourceCode'?: (string);
}
