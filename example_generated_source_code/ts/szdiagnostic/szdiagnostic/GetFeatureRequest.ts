// Original file: szdiagnostic.proto

import type { Long } from '@grpc/proto-loader';

export interface GetFeatureRequest {
  'featureId'?: (number | string | Long);
}

export interface GetFeatureRequest__Output {
  'featureId'?: (Long);
}
