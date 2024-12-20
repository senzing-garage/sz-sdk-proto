// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface GetVirtualEntityByRecordIdRequest {
  'recordKeys'?: (string);
  'flags'?: (number | string | Long);
}

export interface GetVirtualEntityByRecordIdRequest__Output {
  'recordKeys'?: (string);
  'flags'?: (Long);
}
