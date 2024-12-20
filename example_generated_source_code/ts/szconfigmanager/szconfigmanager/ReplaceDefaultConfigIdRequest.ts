// Original file: szconfigmanager.proto

import type { Long } from '@grpc/proto-loader';

export interface ReplaceDefaultConfigIdRequest {
  'currentDefaultConfigId'?: (number | string | Long);
  'newDefaultConfigId'?: (number | string | Long);
}

export interface ReplaceDefaultConfigIdRequest__Output {
  'currentDefaultConfigId'?: (Long);
  'newDefaultConfigId'?: (Long);
}
