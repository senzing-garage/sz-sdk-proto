// Original file: szconfig.proto

import type { Long } from '@grpc/proto-loader';

export interface AddDataSourceRequest {
  'configHandle'?: (number | string | Long);
  'dataSourceCode'?: (string);
}

export interface AddDataSourceRequest__Output {
  'configHandle'?: (Long);
  'dataSourceCode'?: (string);
}
