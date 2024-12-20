// Original file: szengine.proto

import type { Long } from '@grpc/proto-loader';

export interface FindPathByRecordIdRequest {
  'startDataSourceCode'?: (string);
  'startRecordId'?: (string);
  'endDataSourceCode'?: (string);
  'endRecordId'?: (string);
  'maxDegrees'?: (number | string | Long);
  'avoidRecordKeys'?: (string);
  'requiredDataSources'?: (string);
  'flags'?: (number | string | Long);
}

export interface FindPathByRecordIdRequest__Output {
  'startDataSourceCode'?: (string);
  'startRecordId'?: (string);
  'endDataSourceCode'?: (string);
  'endRecordId'?: (string);
  'maxDegrees'?: (Long);
  'avoidRecordKeys'?: (string);
  'requiredDataSources'?: (string);
  'flags'?: (Long);
}
