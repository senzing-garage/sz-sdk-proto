// source: szengine.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {missingRequire} reports error on implicit type usages.
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!
/* eslint-disable */
// @ts-nocheck

var jspb = require('google-protobuf');
var goog = jspb;
var global = (function() {
  if (this) { return this; }
  if (typeof window !== 'undefined') { return window; }
  if (typeof global !== 'undefined') { return global; }
  if (typeof self !== 'undefined') { return self; }
  return Function('return this')();
}.call(null));

goog.exportSymbol('proto.szengine.AddRecordRequest', null, global);
goog.exportSymbol('proto.szengine.AddRecordResponse', null, global);
goog.exportSymbol('proto.szengine.CloseExportReportRequest', null, global);
goog.exportSymbol('proto.szengine.CloseExportReportResponse', null, global);
goog.exportSymbol('proto.szengine.CountRedoRecordsRequest', null, global);
goog.exportSymbol('proto.szengine.CountRedoRecordsResponse', null, global);
goog.exportSymbol('proto.szengine.DeleteRecordRequest', null, global);
goog.exportSymbol('proto.szengine.DeleteRecordResponse', null, global);
goog.exportSymbol('proto.szengine.ExportCsvEntityReportRequest', null, global);
goog.exportSymbol('proto.szengine.ExportCsvEntityReportResponse', null, global);
goog.exportSymbol('proto.szengine.ExportJsonEntityReportRequest', null, global);
goog.exportSymbol('proto.szengine.ExportJsonEntityReportResponse', null, global);
goog.exportSymbol('proto.szengine.FetchNextRequest', null, global);
goog.exportSymbol('proto.szengine.FetchNextResponse', null, global);
goog.exportSymbol('proto.szengine.FindInterestingEntitiesByEntityIdRequest', null, global);
goog.exportSymbol('proto.szengine.FindInterestingEntitiesByEntityIdResponse', null, global);
goog.exportSymbol('proto.szengine.FindInterestingEntitiesByRecordIdRequest', null, global);
goog.exportSymbol('proto.szengine.FindInterestingEntitiesByRecordIdResponse', null, global);
goog.exportSymbol('proto.szengine.FindNetworkByEntityIdRequest', null, global);
goog.exportSymbol('proto.szengine.FindNetworkByEntityIdResponse', null, global);
goog.exportSymbol('proto.szengine.FindNetworkByRecordIdRequest', null, global);
goog.exportSymbol('proto.szengine.FindNetworkByRecordIdResponse', null, global);
goog.exportSymbol('proto.szengine.FindPathByEntityIdRequest', null, global);
goog.exportSymbol('proto.szengine.FindPathByEntityIdResponse', null, global);
goog.exportSymbol('proto.szengine.FindPathByRecordIdRequest', null, global);
goog.exportSymbol('proto.szengine.FindPathByRecordIdResponse', null, global);
goog.exportSymbol('proto.szengine.GetActiveConfigIdRequest', null, global);
goog.exportSymbol('proto.szengine.GetActiveConfigIdResponse', null, global);
goog.exportSymbol('proto.szengine.GetEntityByEntityIdRequest', null, global);
goog.exportSymbol('proto.szengine.GetEntityByEntityIdResponse', null, global);
goog.exportSymbol('proto.szengine.GetEntityByRecordIdRequest', null, global);
goog.exportSymbol('proto.szengine.GetEntityByRecordIdResponse', null, global);
goog.exportSymbol('proto.szengine.GetRecordPreviewRequest', null, global);
goog.exportSymbol('proto.szengine.GetRecordPreviewResponse', null, global);
goog.exportSymbol('proto.szengine.GetRecordRequest', null, global);
goog.exportSymbol('proto.szengine.GetRecordResponse', null, global);
goog.exportSymbol('proto.szengine.GetRedoRecordRequest', null, global);
goog.exportSymbol('proto.szengine.GetRedoRecordResponse', null, global);
goog.exportSymbol('proto.szengine.GetStatsRequest', null, global);
goog.exportSymbol('proto.szengine.GetStatsResponse', null, global);
goog.exportSymbol('proto.szengine.GetVirtualEntityByRecordIdRequest', null, global);
goog.exportSymbol('proto.szengine.GetVirtualEntityByRecordIdResponse', null, global);
goog.exportSymbol('proto.szengine.HowEntityByEntityIdRequest', null, global);
goog.exportSymbol('proto.szengine.HowEntityByEntityIdResponse', null, global);
goog.exportSymbol('proto.szengine.PrimeEngineRequest', null, global);
goog.exportSymbol('proto.szengine.PrimeEngineResponse', null, global);
goog.exportSymbol('proto.szengine.ProcessRedoRecordRequest', null, global);
goog.exportSymbol('proto.szengine.ProcessRedoRecordResponse', null, global);
goog.exportSymbol('proto.szengine.ReevaluateEntityRequest', null, global);
goog.exportSymbol('proto.szengine.ReevaluateEntityResponse', null, global);
goog.exportSymbol('proto.szengine.ReevaluateRecordRequest', null, global);
goog.exportSymbol('proto.szengine.ReevaluateRecordResponse', null, global);
goog.exportSymbol('proto.szengine.ReinitializeRequest', null, global);
goog.exportSymbol('proto.szengine.ReinitializeResponse', null, global);
goog.exportSymbol('proto.szengine.SearchByAttributesRequest', null, global);
goog.exportSymbol('proto.szengine.SearchByAttributesResponse', null, global);
goog.exportSymbol('proto.szengine.StreamExportCsvEntityReportRequest', null, global);
goog.exportSymbol('proto.szengine.StreamExportCsvEntityReportResponse', null, global);
goog.exportSymbol('proto.szengine.StreamExportJsonEntityReportRequest', null, global);
goog.exportSymbol('proto.szengine.StreamExportJsonEntityReportResponse', null, global);
goog.exportSymbol('proto.szengine.WhyEntitiesRequest', null, global);
goog.exportSymbol('proto.szengine.WhyEntitiesResponse', null, global);
goog.exportSymbol('proto.szengine.WhyRecordInEntityRequest', null, global);
goog.exportSymbol('proto.szengine.WhyRecordInEntityResponse', null, global);
goog.exportSymbol('proto.szengine.WhyRecordsRequest', null, global);
goog.exportSymbol('proto.szengine.WhyRecordsResponse', null, global);
goog.exportSymbol('proto.szengine.WhySearchRequest', null, global);
goog.exportSymbol('proto.szengine.WhySearchResponse', null, global);
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.AddRecordRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.AddRecordRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.AddRecordRequest.displayName = 'proto.szengine.AddRecordRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.AddRecordResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.AddRecordResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.AddRecordResponse.displayName = 'proto.szengine.AddRecordResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.CloseExportReportRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.CloseExportReportRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.CloseExportReportRequest.displayName = 'proto.szengine.CloseExportReportRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.CloseExportReportResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.CloseExportReportResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.CloseExportReportResponse.displayName = 'proto.szengine.CloseExportReportResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.CountRedoRecordsRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.CountRedoRecordsRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.CountRedoRecordsRequest.displayName = 'proto.szengine.CountRedoRecordsRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.CountRedoRecordsResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.CountRedoRecordsResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.CountRedoRecordsResponse.displayName = 'proto.szengine.CountRedoRecordsResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.DeleteRecordRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.DeleteRecordRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.DeleteRecordRequest.displayName = 'proto.szengine.DeleteRecordRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.DeleteRecordResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.DeleteRecordResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.DeleteRecordResponse.displayName = 'proto.szengine.DeleteRecordResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.ExportCsvEntityReportRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.ExportCsvEntityReportRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.ExportCsvEntityReportRequest.displayName = 'proto.szengine.ExportCsvEntityReportRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.ExportCsvEntityReportResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.ExportCsvEntityReportResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.ExportCsvEntityReportResponse.displayName = 'proto.szengine.ExportCsvEntityReportResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.ExportJsonEntityReportRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.ExportJsonEntityReportRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.ExportJsonEntityReportRequest.displayName = 'proto.szengine.ExportJsonEntityReportRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.ExportJsonEntityReportResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.ExportJsonEntityReportResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.ExportJsonEntityReportResponse.displayName = 'proto.szengine.ExportJsonEntityReportResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.FetchNextRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.FetchNextRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.FetchNextRequest.displayName = 'proto.szengine.FetchNextRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.FetchNextResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.FetchNextResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.FetchNextResponse.displayName = 'proto.szengine.FetchNextResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.FindInterestingEntitiesByEntityIdRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.FindInterestingEntitiesByEntityIdRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.FindInterestingEntitiesByEntityIdRequest.displayName = 'proto.szengine.FindInterestingEntitiesByEntityIdRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.FindInterestingEntitiesByEntityIdResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.FindInterestingEntitiesByEntityIdResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.FindInterestingEntitiesByEntityIdResponse.displayName = 'proto.szengine.FindInterestingEntitiesByEntityIdResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.FindInterestingEntitiesByRecordIdRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.FindInterestingEntitiesByRecordIdRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.FindInterestingEntitiesByRecordIdRequest.displayName = 'proto.szengine.FindInterestingEntitiesByRecordIdRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.FindInterestingEntitiesByRecordIdResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.FindInterestingEntitiesByRecordIdResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.FindInterestingEntitiesByRecordIdResponse.displayName = 'proto.szengine.FindInterestingEntitiesByRecordIdResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.FindNetworkByEntityIdRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.FindNetworkByEntityIdRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.FindNetworkByEntityIdRequest.displayName = 'proto.szengine.FindNetworkByEntityIdRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.FindNetworkByEntityIdResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.FindNetworkByEntityIdResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.FindNetworkByEntityIdResponse.displayName = 'proto.szengine.FindNetworkByEntityIdResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.FindNetworkByRecordIdRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.FindNetworkByRecordIdRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.FindNetworkByRecordIdRequest.displayName = 'proto.szengine.FindNetworkByRecordIdRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.FindNetworkByRecordIdResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.FindNetworkByRecordIdResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.FindNetworkByRecordIdResponse.displayName = 'proto.szengine.FindNetworkByRecordIdResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.FindPathByEntityIdRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.FindPathByEntityIdRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.FindPathByEntityIdRequest.displayName = 'proto.szengine.FindPathByEntityIdRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.FindPathByEntityIdResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.FindPathByEntityIdResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.FindPathByEntityIdResponse.displayName = 'proto.szengine.FindPathByEntityIdResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.FindPathByRecordIdRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.FindPathByRecordIdRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.FindPathByRecordIdRequest.displayName = 'proto.szengine.FindPathByRecordIdRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.FindPathByRecordIdResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.FindPathByRecordIdResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.FindPathByRecordIdResponse.displayName = 'proto.szengine.FindPathByRecordIdResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetActiveConfigIdRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetActiveConfigIdRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetActiveConfigIdRequest.displayName = 'proto.szengine.GetActiveConfigIdRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetActiveConfigIdResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetActiveConfigIdResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetActiveConfigIdResponse.displayName = 'proto.szengine.GetActiveConfigIdResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetEntityByEntityIdRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetEntityByEntityIdRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetEntityByEntityIdRequest.displayName = 'proto.szengine.GetEntityByEntityIdRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetEntityByEntityIdResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetEntityByEntityIdResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetEntityByEntityIdResponse.displayName = 'proto.szengine.GetEntityByEntityIdResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetEntityByRecordIdRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetEntityByRecordIdRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetEntityByRecordIdRequest.displayName = 'proto.szengine.GetEntityByRecordIdRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetEntityByRecordIdResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetEntityByRecordIdResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetEntityByRecordIdResponse.displayName = 'proto.szengine.GetEntityByRecordIdResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetRecordPreviewRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetRecordPreviewRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetRecordPreviewRequest.displayName = 'proto.szengine.GetRecordPreviewRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetRecordPreviewResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetRecordPreviewResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetRecordPreviewResponse.displayName = 'proto.szengine.GetRecordPreviewResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetRecordRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetRecordRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetRecordRequest.displayName = 'proto.szengine.GetRecordRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetRecordResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetRecordResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetRecordResponse.displayName = 'proto.szengine.GetRecordResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetRedoRecordRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetRedoRecordRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetRedoRecordRequest.displayName = 'proto.szengine.GetRedoRecordRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetRedoRecordResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetRedoRecordResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetRedoRecordResponse.displayName = 'proto.szengine.GetRedoRecordResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetStatsRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetStatsRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetStatsRequest.displayName = 'proto.szengine.GetStatsRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetStatsResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetStatsResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetStatsResponse.displayName = 'proto.szengine.GetStatsResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetVirtualEntityByRecordIdRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetVirtualEntityByRecordIdRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetVirtualEntityByRecordIdRequest.displayName = 'proto.szengine.GetVirtualEntityByRecordIdRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.GetVirtualEntityByRecordIdResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.GetVirtualEntityByRecordIdResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.GetVirtualEntityByRecordIdResponse.displayName = 'proto.szengine.GetVirtualEntityByRecordIdResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.HowEntityByEntityIdRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.HowEntityByEntityIdRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.HowEntityByEntityIdRequest.displayName = 'proto.szengine.HowEntityByEntityIdRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.HowEntityByEntityIdResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.HowEntityByEntityIdResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.HowEntityByEntityIdResponse.displayName = 'proto.szengine.HowEntityByEntityIdResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.PrimeEngineRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.PrimeEngineRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.PrimeEngineRequest.displayName = 'proto.szengine.PrimeEngineRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.PrimeEngineResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.PrimeEngineResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.PrimeEngineResponse.displayName = 'proto.szengine.PrimeEngineResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.ProcessRedoRecordRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.ProcessRedoRecordRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.ProcessRedoRecordRequest.displayName = 'proto.szengine.ProcessRedoRecordRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.ProcessRedoRecordResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.ProcessRedoRecordResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.ProcessRedoRecordResponse.displayName = 'proto.szengine.ProcessRedoRecordResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.ReevaluateEntityRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.ReevaluateEntityRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.ReevaluateEntityRequest.displayName = 'proto.szengine.ReevaluateEntityRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.ReevaluateEntityResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.ReevaluateEntityResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.ReevaluateEntityResponse.displayName = 'proto.szengine.ReevaluateEntityResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.ReevaluateRecordRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.ReevaluateRecordRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.ReevaluateRecordRequest.displayName = 'proto.szengine.ReevaluateRecordRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.ReevaluateRecordResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.ReevaluateRecordResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.ReevaluateRecordResponse.displayName = 'proto.szengine.ReevaluateRecordResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.ReinitializeRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.ReinitializeRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.ReinitializeRequest.displayName = 'proto.szengine.ReinitializeRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.ReinitializeResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.ReinitializeResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.ReinitializeResponse.displayName = 'proto.szengine.ReinitializeResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.SearchByAttributesRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.SearchByAttributesRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.SearchByAttributesRequest.displayName = 'proto.szengine.SearchByAttributesRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.SearchByAttributesResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.SearchByAttributesResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.SearchByAttributesResponse.displayName = 'proto.szengine.SearchByAttributesResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.StreamExportCsvEntityReportRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.StreamExportCsvEntityReportRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.StreamExportCsvEntityReportRequest.displayName = 'proto.szengine.StreamExportCsvEntityReportRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.StreamExportCsvEntityReportResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.StreamExportCsvEntityReportResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.StreamExportCsvEntityReportResponse.displayName = 'proto.szengine.StreamExportCsvEntityReportResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.StreamExportJsonEntityReportRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.StreamExportJsonEntityReportRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.StreamExportJsonEntityReportRequest.displayName = 'proto.szengine.StreamExportJsonEntityReportRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.StreamExportJsonEntityReportResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.StreamExportJsonEntityReportResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.StreamExportJsonEntityReportResponse.displayName = 'proto.szengine.StreamExportJsonEntityReportResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.WhyEntitiesRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.WhyEntitiesRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.WhyEntitiesRequest.displayName = 'proto.szengine.WhyEntitiesRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.WhyEntitiesResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.WhyEntitiesResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.WhyEntitiesResponse.displayName = 'proto.szengine.WhyEntitiesResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.WhyRecordInEntityRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.WhyRecordInEntityRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.WhyRecordInEntityRequest.displayName = 'proto.szengine.WhyRecordInEntityRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.WhyRecordInEntityResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.WhyRecordInEntityResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.WhyRecordInEntityResponse.displayName = 'proto.szengine.WhyRecordInEntityResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.WhyRecordsRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.WhyRecordsRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.WhyRecordsRequest.displayName = 'proto.szengine.WhyRecordsRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.WhyRecordsResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.WhyRecordsResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.WhyRecordsResponse.displayName = 'proto.szengine.WhyRecordsResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.WhySearchRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.WhySearchRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.WhySearchRequest.displayName = 'proto.szengine.WhySearchRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.szengine.WhySearchResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.szengine.WhySearchResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.szengine.WhySearchResponse.displayName = 'proto.szengine.WhySearchResponse';
}



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.AddRecordRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.AddRecordRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.AddRecordRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.AddRecordRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    dataSourceCode: jspb.Message.getFieldWithDefault(msg, 1, ""),
    recordId: jspb.Message.getFieldWithDefault(msg, 2, ""),
    recordDefinition: jspb.Message.getFieldWithDefault(msg, 3, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 4, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.AddRecordRequest}
 */
proto.szengine.AddRecordRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.AddRecordRequest;
  return proto.szengine.AddRecordRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.AddRecordRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.AddRecordRequest}
 */
proto.szengine.AddRecordRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setDataSourceCode(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setRecordId(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setRecordDefinition(value);
      break;
    case 4:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.AddRecordRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.AddRecordRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.AddRecordRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.AddRecordRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDataSourceCode();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getRecordId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getRecordDefinition();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      4,
      f
    );
  }
};


/**
 * optional string data_source_code = 1;
 * @return {string}
 */
proto.szengine.AddRecordRequest.prototype.getDataSourceCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.AddRecordRequest} returns this
 */
proto.szengine.AddRecordRequest.prototype.setDataSourceCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string record_id = 2;
 * @return {string}
 */
proto.szengine.AddRecordRequest.prototype.getRecordId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.AddRecordRequest} returns this
 */
proto.szengine.AddRecordRequest.prototype.setRecordId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string record_definition = 3;
 * @return {string}
 */
proto.szengine.AddRecordRequest.prototype.getRecordDefinition = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.AddRecordRequest} returns this
 */
proto.szengine.AddRecordRequest.prototype.setRecordDefinition = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional int64 flags = 4;
 * @return {number}
 */
proto.szengine.AddRecordRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 4, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.AddRecordRequest} returns this
 */
proto.szengine.AddRecordRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 4, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.AddRecordResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.AddRecordResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.AddRecordResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.AddRecordResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.AddRecordResponse}
 */
proto.szengine.AddRecordResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.AddRecordResponse;
  return proto.szengine.AddRecordResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.AddRecordResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.AddRecordResponse}
 */
proto.szengine.AddRecordResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.AddRecordResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.AddRecordResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.AddRecordResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.AddRecordResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.AddRecordResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.AddRecordResponse} returns this
 */
proto.szengine.AddRecordResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.CloseExportReportRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.CloseExportReportRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.CloseExportReportRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.CloseExportReportRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    exportHandle: jspb.Message.getFieldWithDefault(msg, 1, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.CloseExportReportRequest}
 */
proto.szengine.CloseExportReportRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.CloseExportReportRequest;
  return proto.szengine.CloseExportReportRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.CloseExportReportRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.CloseExportReportRequest}
 */
proto.szengine.CloseExportReportRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setExportHandle(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.CloseExportReportRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.CloseExportReportRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.CloseExportReportRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.CloseExportReportRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getExportHandle();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
};


/**
 * optional int64 export_handle = 1;
 * @return {number}
 */
proto.szengine.CloseExportReportRequest.prototype.getExportHandle = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.CloseExportReportRequest} returns this
 */
proto.szengine.CloseExportReportRequest.prototype.setExportHandle = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.CloseExportReportResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.CloseExportReportResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.CloseExportReportResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.CloseExportReportResponse.toObject = function(includeInstance, msg) {
  var f, obj = {

  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.CloseExportReportResponse}
 */
proto.szengine.CloseExportReportResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.CloseExportReportResponse;
  return proto.szengine.CloseExportReportResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.CloseExportReportResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.CloseExportReportResponse}
 */
proto.szengine.CloseExportReportResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.CloseExportReportResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.CloseExportReportResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.CloseExportReportResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.CloseExportReportResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.CountRedoRecordsRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.CountRedoRecordsRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.CountRedoRecordsRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.CountRedoRecordsRequest.toObject = function(includeInstance, msg) {
  var f, obj = {

  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.CountRedoRecordsRequest}
 */
proto.szengine.CountRedoRecordsRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.CountRedoRecordsRequest;
  return proto.szengine.CountRedoRecordsRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.CountRedoRecordsRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.CountRedoRecordsRequest}
 */
proto.szengine.CountRedoRecordsRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.CountRedoRecordsRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.CountRedoRecordsRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.CountRedoRecordsRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.CountRedoRecordsRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.CountRedoRecordsResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.CountRedoRecordsResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.CountRedoRecordsResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.CountRedoRecordsResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.CountRedoRecordsResponse}
 */
proto.szengine.CountRedoRecordsResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.CountRedoRecordsResponse;
  return proto.szengine.CountRedoRecordsResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.CountRedoRecordsResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.CountRedoRecordsResponse}
 */
proto.szengine.CountRedoRecordsResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.CountRedoRecordsResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.CountRedoRecordsResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.CountRedoRecordsResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.CountRedoRecordsResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
};


/**
 * optional int64 result = 1;
 * @return {number}
 */
proto.szengine.CountRedoRecordsResponse.prototype.getResult = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.CountRedoRecordsResponse} returns this
 */
proto.szengine.CountRedoRecordsResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.DeleteRecordRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.DeleteRecordRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.DeleteRecordRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.DeleteRecordRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    dataSourceCode: jspb.Message.getFieldWithDefault(msg, 1, ""),
    recordId: jspb.Message.getFieldWithDefault(msg, 2, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 3, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.DeleteRecordRequest}
 */
proto.szengine.DeleteRecordRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.DeleteRecordRequest;
  return proto.szengine.DeleteRecordRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.DeleteRecordRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.DeleteRecordRequest}
 */
proto.szengine.DeleteRecordRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setDataSourceCode(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setRecordId(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.DeleteRecordRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.DeleteRecordRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.DeleteRecordRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.DeleteRecordRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDataSourceCode();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getRecordId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
};


/**
 * optional string data_source_code = 1;
 * @return {string}
 */
proto.szengine.DeleteRecordRequest.prototype.getDataSourceCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.DeleteRecordRequest} returns this
 */
proto.szengine.DeleteRecordRequest.prototype.setDataSourceCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string record_id = 2;
 * @return {string}
 */
proto.szengine.DeleteRecordRequest.prototype.getRecordId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.DeleteRecordRequest} returns this
 */
proto.szengine.DeleteRecordRequest.prototype.setRecordId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional int64 flags = 3;
 * @return {number}
 */
proto.szengine.DeleteRecordRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.DeleteRecordRequest} returns this
 */
proto.szengine.DeleteRecordRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.DeleteRecordResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.DeleteRecordResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.DeleteRecordResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.DeleteRecordResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.DeleteRecordResponse}
 */
proto.szengine.DeleteRecordResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.DeleteRecordResponse;
  return proto.szengine.DeleteRecordResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.DeleteRecordResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.DeleteRecordResponse}
 */
proto.szengine.DeleteRecordResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.DeleteRecordResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.DeleteRecordResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.DeleteRecordResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.DeleteRecordResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.DeleteRecordResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.DeleteRecordResponse} returns this
 */
proto.szengine.DeleteRecordResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.ExportCsvEntityReportRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.ExportCsvEntityReportRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.ExportCsvEntityReportRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ExportCsvEntityReportRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    csvColumnList: jspb.Message.getFieldWithDefault(msg, 1, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 2, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.ExportCsvEntityReportRequest}
 */
proto.szengine.ExportCsvEntityReportRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.ExportCsvEntityReportRequest;
  return proto.szengine.ExportCsvEntityReportRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.ExportCsvEntityReportRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.ExportCsvEntityReportRequest}
 */
proto.szengine.ExportCsvEntityReportRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setCsvColumnList(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.ExportCsvEntityReportRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.ExportCsvEntityReportRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.ExportCsvEntityReportRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ExportCsvEntityReportRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getCsvColumnList();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
};


/**
 * optional string csv_column_list = 1;
 * @return {string}
 */
proto.szengine.ExportCsvEntityReportRequest.prototype.getCsvColumnList = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.ExportCsvEntityReportRequest} returns this
 */
proto.szengine.ExportCsvEntityReportRequest.prototype.setCsvColumnList = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int64 flags = 2;
 * @return {number}
 */
proto.szengine.ExportCsvEntityReportRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.ExportCsvEntityReportRequest} returns this
 */
proto.szengine.ExportCsvEntityReportRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.ExportCsvEntityReportResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.ExportCsvEntityReportResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.ExportCsvEntityReportResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ExportCsvEntityReportResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.ExportCsvEntityReportResponse}
 */
proto.szengine.ExportCsvEntityReportResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.ExportCsvEntityReportResponse;
  return proto.szengine.ExportCsvEntityReportResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.ExportCsvEntityReportResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.ExportCsvEntityReportResponse}
 */
proto.szengine.ExportCsvEntityReportResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.ExportCsvEntityReportResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.ExportCsvEntityReportResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.ExportCsvEntityReportResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ExportCsvEntityReportResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
};


/**
 * optional int64 result = 1;
 * @return {number}
 */
proto.szengine.ExportCsvEntityReportResponse.prototype.getResult = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.ExportCsvEntityReportResponse} returns this
 */
proto.szengine.ExportCsvEntityReportResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.ExportJsonEntityReportRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.ExportJsonEntityReportRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.ExportJsonEntityReportRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ExportJsonEntityReportRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    flags: jspb.Message.getFieldWithDefault(msg, 1, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.ExportJsonEntityReportRequest}
 */
proto.szengine.ExportJsonEntityReportRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.ExportJsonEntityReportRequest;
  return proto.szengine.ExportJsonEntityReportRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.ExportJsonEntityReportRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.ExportJsonEntityReportRequest}
 */
proto.szengine.ExportJsonEntityReportRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.ExportJsonEntityReportRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.ExportJsonEntityReportRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.ExportJsonEntityReportRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ExportJsonEntityReportRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
};


/**
 * optional int64 flags = 1;
 * @return {number}
 */
proto.szengine.ExportJsonEntityReportRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.ExportJsonEntityReportRequest} returns this
 */
proto.szengine.ExportJsonEntityReportRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.ExportJsonEntityReportResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.ExportJsonEntityReportResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.ExportJsonEntityReportResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ExportJsonEntityReportResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.ExportJsonEntityReportResponse}
 */
proto.szengine.ExportJsonEntityReportResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.ExportJsonEntityReportResponse;
  return proto.szengine.ExportJsonEntityReportResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.ExportJsonEntityReportResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.ExportJsonEntityReportResponse}
 */
proto.szengine.ExportJsonEntityReportResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.ExportJsonEntityReportResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.ExportJsonEntityReportResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.ExportJsonEntityReportResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ExportJsonEntityReportResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
};


/**
 * optional int64 result = 1;
 * @return {number}
 */
proto.szengine.ExportJsonEntityReportResponse.prototype.getResult = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.ExportJsonEntityReportResponse} returns this
 */
proto.szengine.ExportJsonEntityReportResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.FetchNextRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.FetchNextRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.FetchNextRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FetchNextRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    exportHandle: jspb.Message.getFieldWithDefault(msg, 1, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.FetchNextRequest}
 */
proto.szengine.FetchNextRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.FetchNextRequest;
  return proto.szengine.FetchNextRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.FetchNextRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.FetchNextRequest}
 */
proto.szengine.FetchNextRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setExportHandle(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.FetchNextRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.FetchNextRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.FetchNextRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FetchNextRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getExportHandle();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
};


/**
 * optional int64 export_handle = 1;
 * @return {number}
 */
proto.szengine.FetchNextRequest.prototype.getExportHandle = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FetchNextRequest} returns this
 */
proto.szengine.FetchNextRequest.prototype.setExportHandle = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.FetchNextResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.FetchNextResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.FetchNextResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FetchNextResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.FetchNextResponse}
 */
proto.szengine.FetchNextResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.FetchNextResponse;
  return proto.szengine.FetchNextResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.FetchNextResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.FetchNextResponse}
 */
proto.szengine.FetchNextResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.FetchNextResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.FetchNextResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.FetchNextResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FetchNextResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.FetchNextResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FetchNextResponse} returns this
 */
proto.szengine.FetchNextResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.FindInterestingEntitiesByEntityIdRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.FindInterestingEntitiesByEntityIdRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.FindInterestingEntitiesByEntityIdRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindInterestingEntitiesByEntityIdRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    entityId: jspb.Message.getFieldWithDefault(msg, 1, 0),
    flags: jspb.Message.getFieldWithDefault(msg, 2, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.FindInterestingEntitiesByEntityIdRequest}
 */
proto.szengine.FindInterestingEntitiesByEntityIdRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.FindInterestingEntitiesByEntityIdRequest;
  return proto.szengine.FindInterestingEntitiesByEntityIdRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.FindInterestingEntitiesByEntityIdRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.FindInterestingEntitiesByEntityIdRequest}
 */
proto.szengine.FindInterestingEntitiesByEntityIdRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setEntityId(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.FindInterestingEntitiesByEntityIdRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.FindInterestingEntitiesByEntityIdRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.FindInterestingEntitiesByEntityIdRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindInterestingEntitiesByEntityIdRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getEntityId();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
};


/**
 * optional int64 entity_id = 1;
 * @return {number}
 */
proto.szengine.FindInterestingEntitiesByEntityIdRequest.prototype.getEntityId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindInterestingEntitiesByEntityIdRequest} returns this
 */
proto.szengine.FindInterestingEntitiesByEntityIdRequest.prototype.setEntityId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int64 flags = 2;
 * @return {number}
 */
proto.szengine.FindInterestingEntitiesByEntityIdRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindInterestingEntitiesByEntityIdRequest} returns this
 */
proto.szengine.FindInterestingEntitiesByEntityIdRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.FindInterestingEntitiesByEntityIdResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.FindInterestingEntitiesByEntityIdResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.FindInterestingEntitiesByEntityIdResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindInterestingEntitiesByEntityIdResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.FindInterestingEntitiesByEntityIdResponse}
 */
proto.szengine.FindInterestingEntitiesByEntityIdResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.FindInterestingEntitiesByEntityIdResponse;
  return proto.szengine.FindInterestingEntitiesByEntityIdResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.FindInterestingEntitiesByEntityIdResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.FindInterestingEntitiesByEntityIdResponse}
 */
proto.szengine.FindInterestingEntitiesByEntityIdResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.FindInterestingEntitiesByEntityIdResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.FindInterestingEntitiesByEntityIdResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.FindInterestingEntitiesByEntityIdResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindInterestingEntitiesByEntityIdResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.FindInterestingEntitiesByEntityIdResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindInterestingEntitiesByEntityIdResponse} returns this
 */
proto.szengine.FindInterestingEntitiesByEntityIdResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.FindInterestingEntitiesByRecordIdRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.FindInterestingEntitiesByRecordIdRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.FindInterestingEntitiesByRecordIdRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindInterestingEntitiesByRecordIdRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    dataSourceCode: jspb.Message.getFieldWithDefault(msg, 1, ""),
    recordId: jspb.Message.getFieldWithDefault(msg, 2, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 3, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.FindInterestingEntitiesByRecordIdRequest}
 */
proto.szengine.FindInterestingEntitiesByRecordIdRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.FindInterestingEntitiesByRecordIdRequest;
  return proto.szengine.FindInterestingEntitiesByRecordIdRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.FindInterestingEntitiesByRecordIdRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.FindInterestingEntitiesByRecordIdRequest}
 */
proto.szengine.FindInterestingEntitiesByRecordIdRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setDataSourceCode(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setRecordId(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.FindInterestingEntitiesByRecordIdRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.FindInterestingEntitiesByRecordIdRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.FindInterestingEntitiesByRecordIdRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindInterestingEntitiesByRecordIdRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDataSourceCode();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getRecordId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
};


/**
 * optional string data_source_code = 1;
 * @return {string}
 */
proto.szengine.FindInterestingEntitiesByRecordIdRequest.prototype.getDataSourceCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindInterestingEntitiesByRecordIdRequest} returns this
 */
proto.szengine.FindInterestingEntitiesByRecordIdRequest.prototype.setDataSourceCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string record_id = 2;
 * @return {string}
 */
proto.szengine.FindInterestingEntitiesByRecordIdRequest.prototype.getRecordId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindInterestingEntitiesByRecordIdRequest} returns this
 */
proto.szengine.FindInterestingEntitiesByRecordIdRequest.prototype.setRecordId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional int64 flags = 3;
 * @return {number}
 */
proto.szengine.FindInterestingEntitiesByRecordIdRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindInterestingEntitiesByRecordIdRequest} returns this
 */
proto.szengine.FindInterestingEntitiesByRecordIdRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.FindInterestingEntitiesByRecordIdResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.FindInterestingEntitiesByRecordIdResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.FindInterestingEntitiesByRecordIdResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindInterestingEntitiesByRecordIdResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.FindInterestingEntitiesByRecordIdResponse}
 */
proto.szengine.FindInterestingEntitiesByRecordIdResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.FindInterestingEntitiesByRecordIdResponse;
  return proto.szengine.FindInterestingEntitiesByRecordIdResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.FindInterestingEntitiesByRecordIdResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.FindInterestingEntitiesByRecordIdResponse}
 */
proto.szengine.FindInterestingEntitiesByRecordIdResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.FindInterestingEntitiesByRecordIdResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.FindInterestingEntitiesByRecordIdResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.FindInterestingEntitiesByRecordIdResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindInterestingEntitiesByRecordIdResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.FindInterestingEntitiesByRecordIdResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindInterestingEntitiesByRecordIdResponse} returns this
 */
proto.szengine.FindInterestingEntitiesByRecordIdResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.FindNetworkByEntityIdRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.FindNetworkByEntityIdRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.FindNetworkByEntityIdRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindNetworkByEntityIdRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    entityIds: jspb.Message.getFieldWithDefault(msg, 1, ""),
    maxDegrees: jspb.Message.getFieldWithDefault(msg, 2, 0),
    buildOutDegrees: jspb.Message.getFieldWithDefault(msg, 3, 0),
    buildOutMaxEntities: jspb.Message.getFieldWithDefault(msg, 4, 0),
    flags: jspb.Message.getFieldWithDefault(msg, 5, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.FindNetworkByEntityIdRequest}
 */
proto.szengine.FindNetworkByEntityIdRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.FindNetworkByEntityIdRequest;
  return proto.szengine.FindNetworkByEntityIdRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.FindNetworkByEntityIdRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.FindNetworkByEntityIdRequest}
 */
proto.szengine.FindNetworkByEntityIdRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setEntityIds(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setMaxDegrees(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setBuildOutDegrees(value);
      break;
    case 4:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setBuildOutMaxEntities(value);
      break;
    case 5:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.FindNetworkByEntityIdRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.FindNetworkByEntityIdRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.FindNetworkByEntityIdRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindNetworkByEntityIdRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getEntityIds();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getMaxDegrees();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getBuildOutDegrees();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
  f = message.getBuildOutMaxEntities();
  if (f !== 0) {
    writer.writeInt64(
      4,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      5,
      f
    );
  }
};


/**
 * optional string entity_ids = 1;
 * @return {string}
 */
proto.szengine.FindNetworkByEntityIdRequest.prototype.getEntityIds = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindNetworkByEntityIdRequest} returns this
 */
proto.szengine.FindNetworkByEntityIdRequest.prototype.setEntityIds = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int64 max_degrees = 2;
 * @return {number}
 */
proto.szengine.FindNetworkByEntityIdRequest.prototype.getMaxDegrees = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindNetworkByEntityIdRequest} returns this
 */
proto.szengine.FindNetworkByEntityIdRequest.prototype.setMaxDegrees = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional int64 build_out_degrees = 3;
 * @return {number}
 */
proto.szengine.FindNetworkByEntityIdRequest.prototype.getBuildOutDegrees = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindNetworkByEntityIdRequest} returns this
 */
proto.szengine.FindNetworkByEntityIdRequest.prototype.setBuildOutDegrees = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};


/**
 * optional int64 build_out_max_entities = 4;
 * @return {number}
 */
proto.szengine.FindNetworkByEntityIdRequest.prototype.getBuildOutMaxEntities = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 4, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindNetworkByEntityIdRequest} returns this
 */
proto.szengine.FindNetworkByEntityIdRequest.prototype.setBuildOutMaxEntities = function(value) {
  return jspb.Message.setProto3IntField(this, 4, value);
};


/**
 * optional int64 flags = 5;
 * @return {number}
 */
proto.szengine.FindNetworkByEntityIdRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 5, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindNetworkByEntityIdRequest} returns this
 */
proto.szengine.FindNetworkByEntityIdRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 5, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.FindNetworkByEntityIdResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.FindNetworkByEntityIdResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.FindNetworkByEntityIdResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindNetworkByEntityIdResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.FindNetworkByEntityIdResponse}
 */
proto.szengine.FindNetworkByEntityIdResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.FindNetworkByEntityIdResponse;
  return proto.szengine.FindNetworkByEntityIdResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.FindNetworkByEntityIdResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.FindNetworkByEntityIdResponse}
 */
proto.szengine.FindNetworkByEntityIdResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.FindNetworkByEntityIdResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.FindNetworkByEntityIdResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.FindNetworkByEntityIdResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindNetworkByEntityIdResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.FindNetworkByEntityIdResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindNetworkByEntityIdResponse} returns this
 */
proto.szengine.FindNetworkByEntityIdResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.FindNetworkByRecordIdRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.FindNetworkByRecordIdRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.FindNetworkByRecordIdRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindNetworkByRecordIdRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    recordKeys: jspb.Message.getFieldWithDefault(msg, 1, ""),
    maxDegrees: jspb.Message.getFieldWithDefault(msg, 2, 0),
    buildOutDegrees: jspb.Message.getFieldWithDefault(msg, 3, 0),
    buildOutMaxEntities: jspb.Message.getFieldWithDefault(msg, 4, 0),
    flags: jspb.Message.getFieldWithDefault(msg, 5, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.FindNetworkByRecordIdRequest}
 */
proto.szengine.FindNetworkByRecordIdRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.FindNetworkByRecordIdRequest;
  return proto.szengine.FindNetworkByRecordIdRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.FindNetworkByRecordIdRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.FindNetworkByRecordIdRequest}
 */
proto.szengine.FindNetworkByRecordIdRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setRecordKeys(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setMaxDegrees(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setBuildOutDegrees(value);
      break;
    case 4:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setBuildOutMaxEntities(value);
      break;
    case 5:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.FindNetworkByRecordIdRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.FindNetworkByRecordIdRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.FindNetworkByRecordIdRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindNetworkByRecordIdRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getRecordKeys();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getMaxDegrees();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getBuildOutDegrees();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
  f = message.getBuildOutMaxEntities();
  if (f !== 0) {
    writer.writeInt64(
      4,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      5,
      f
    );
  }
};


/**
 * optional string record_keys = 1;
 * @return {string}
 */
proto.szengine.FindNetworkByRecordIdRequest.prototype.getRecordKeys = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindNetworkByRecordIdRequest} returns this
 */
proto.szengine.FindNetworkByRecordIdRequest.prototype.setRecordKeys = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int64 max_degrees = 2;
 * @return {number}
 */
proto.szengine.FindNetworkByRecordIdRequest.prototype.getMaxDegrees = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindNetworkByRecordIdRequest} returns this
 */
proto.szengine.FindNetworkByRecordIdRequest.prototype.setMaxDegrees = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional int64 build_out_degrees = 3;
 * @return {number}
 */
proto.szengine.FindNetworkByRecordIdRequest.prototype.getBuildOutDegrees = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindNetworkByRecordIdRequest} returns this
 */
proto.szengine.FindNetworkByRecordIdRequest.prototype.setBuildOutDegrees = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};


/**
 * optional int64 build_out_max_entities = 4;
 * @return {number}
 */
proto.szengine.FindNetworkByRecordIdRequest.prototype.getBuildOutMaxEntities = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 4, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindNetworkByRecordIdRequest} returns this
 */
proto.szengine.FindNetworkByRecordIdRequest.prototype.setBuildOutMaxEntities = function(value) {
  return jspb.Message.setProto3IntField(this, 4, value);
};


/**
 * optional int64 flags = 5;
 * @return {number}
 */
proto.szengine.FindNetworkByRecordIdRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 5, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindNetworkByRecordIdRequest} returns this
 */
proto.szengine.FindNetworkByRecordIdRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 5, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.FindNetworkByRecordIdResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.FindNetworkByRecordIdResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.FindNetworkByRecordIdResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindNetworkByRecordIdResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.FindNetworkByRecordIdResponse}
 */
proto.szengine.FindNetworkByRecordIdResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.FindNetworkByRecordIdResponse;
  return proto.szengine.FindNetworkByRecordIdResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.FindNetworkByRecordIdResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.FindNetworkByRecordIdResponse}
 */
proto.szengine.FindNetworkByRecordIdResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.FindNetworkByRecordIdResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.FindNetworkByRecordIdResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.FindNetworkByRecordIdResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindNetworkByRecordIdResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.FindNetworkByRecordIdResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindNetworkByRecordIdResponse} returns this
 */
proto.szengine.FindNetworkByRecordIdResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.FindPathByEntityIdRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.FindPathByEntityIdRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.FindPathByEntityIdRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindPathByEntityIdRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    startEntityId: jspb.Message.getFieldWithDefault(msg, 1, 0),
    endEntityId: jspb.Message.getFieldWithDefault(msg, 2, 0),
    maxDegrees: jspb.Message.getFieldWithDefault(msg, 3, 0),
    avoidEntityIds: jspb.Message.getFieldWithDefault(msg, 4, ""),
    requiredDataSources: jspb.Message.getFieldWithDefault(msg, 5, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 6, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.FindPathByEntityIdRequest}
 */
proto.szengine.FindPathByEntityIdRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.FindPathByEntityIdRequest;
  return proto.szengine.FindPathByEntityIdRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.FindPathByEntityIdRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.FindPathByEntityIdRequest}
 */
proto.szengine.FindPathByEntityIdRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setStartEntityId(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setEndEntityId(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setMaxDegrees(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setAvoidEntityIds(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setRequiredDataSources(value);
      break;
    case 6:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.FindPathByEntityIdRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.FindPathByEntityIdRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.FindPathByEntityIdRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindPathByEntityIdRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStartEntityId();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getEndEntityId();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getMaxDegrees();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
  f = message.getAvoidEntityIds();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getRequiredDataSources();
  if (f.length > 0) {
    writer.writeString(
      5,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      6,
      f
    );
  }
};


/**
 * optional int64 start_entity_id = 1;
 * @return {number}
 */
proto.szengine.FindPathByEntityIdRequest.prototype.getStartEntityId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindPathByEntityIdRequest} returns this
 */
proto.szengine.FindPathByEntityIdRequest.prototype.setStartEntityId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int64 end_entity_id = 2;
 * @return {number}
 */
proto.szengine.FindPathByEntityIdRequest.prototype.getEndEntityId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindPathByEntityIdRequest} returns this
 */
proto.szengine.FindPathByEntityIdRequest.prototype.setEndEntityId = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional int64 max_degrees = 3;
 * @return {number}
 */
proto.szengine.FindPathByEntityIdRequest.prototype.getMaxDegrees = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindPathByEntityIdRequest} returns this
 */
proto.szengine.FindPathByEntityIdRequest.prototype.setMaxDegrees = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};


/**
 * optional string avoid_entity_ids = 4;
 * @return {string}
 */
proto.szengine.FindPathByEntityIdRequest.prototype.getAvoidEntityIds = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindPathByEntityIdRequest} returns this
 */
proto.szengine.FindPathByEntityIdRequest.prototype.setAvoidEntityIds = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional string required_data_sources = 5;
 * @return {string}
 */
proto.szengine.FindPathByEntityIdRequest.prototype.getRequiredDataSources = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindPathByEntityIdRequest} returns this
 */
proto.szengine.FindPathByEntityIdRequest.prototype.setRequiredDataSources = function(value) {
  return jspb.Message.setProto3StringField(this, 5, value);
};


/**
 * optional int64 flags = 6;
 * @return {number}
 */
proto.szengine.FindPathByEntityIdRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 6, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindPathByEntityIdRequest} returns this
 */
proto.szengine.FindPathByEntityIdRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 6, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.FindPathByEntityIdResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.FindPathByEntityIdResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.FindPathByEntityIdResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindPathByEntityIdResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.FindPathByEntityIdResponse}
 */
proto.szengine.FindPathByEntityIdResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.FindPathByEntityIdResponse;
  return proto.szengine.FindPathByEntityIdResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.FindPathByEntityIdResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.FindPathByEntityIdResponse}
 */
proto.szengine.FindPathByEntityIdResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.FindPathByEntityIdResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.FindPathByEntityIdResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.FindPathByEntityIdResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindPathByEntityIdResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.FindPathByEntityIdResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindPathByEntityIdResponse} returns this
 */
proto.szengine.FindPathByEntityIdResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.FindPathByRecordIdRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.FindPathByRecordIdRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.FindPathByRecordIdRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindPathByRecordIdRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    startDataSourceCode: jspb.Message.getFieldWithDefault(msg, 1, ""),
    startRecordId: jspb.Message.getFieldWithDefault(msg, 2, ""),
    endDataSourceCode: jspb.Message.getFieldWithDefault(msg, 3, ""),
    endRecordId: jspb.Message.getFieldWithDefault(msg, 4, ""),
    maxDegrees: jspb.Message.getFieldWithDefault(msg, 5, 0),
    avoidRecordKeys: jspb.Message.getFieldWithDefault(msg, 6, ""),
    requiredDataSources: jspb.Message.getFieldWithDefault(msg, 7, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 8, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.FindPathByRecordIdRequest}
 */
proto.szengine.FindPathByRecordIdRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.FindPathByRecordIdRequest;
  return proto.szengine.FindPathByRecordIdRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.FindPathByRecordIdRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.FindPathByRecordIdRequest}
 */
proto.szengine.FindPathByRecordIdRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setStartDataSourceCode(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setStartRecordId(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setEndDataSourceCode(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setEndRecordId(value);
      break;
    case 5:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setMaxDegrees(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setAvoidRecordKeys(value);
      break;
    case 7:
      var value = /** @type {string} */ (reader.readString());
      msg.setRequiredDataSources(value);
      break;
    case 8:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.FindPathByRecordIdRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.FindPathByRecordIdRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.FindPathByRecordIdRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindPathByRecordIdRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStartDataSourceCode();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getStartRecordId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getEndDataSourceCode();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getEndRecordId();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getMaxDegrees();
  if (f !== 0) {
    writer.writeInt64(
      5,
      f
    );
  }
  f = message.getAvoidRecordKeys();
  if (f.length > 0) {
    writer.writeString(
      6,
      f
    );
  }
  f = message.getRequiredDataSources();
  if (f.length > 0) {
    writer.writeString(
      7,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      8,
      f
    );
  }
};


/**
 * optional string start_data_source_code = 1;
 * @return {string}
 */
proto.szengine.FindPathByRecordIdRequest.prototype.getStartDataSourceCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindPathByRecordIdRequest} returns this
 */
proto.szengine.FindPathByRecordIdRequest.prototype.setStartDataSourceCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string start_record_id = 2;
 * @return {string}
 */
proto.szengine.FindPathByRecordIdRequest.prototype.getStartRecordId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindPathByRecordIdRequest} returns this
 */
proto.szengine.FindPathByRecordIdRequest.prototype.setStartRecordId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string end_data_source_code = 3;
 * @return {string}
 */
proto.szengine.FindPathByRecordIdRequest.prototype.getEndDataSourceCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindPathByRecordIdRequest} returns this
 */
proto.szengine.FindPathByRecordIdRequest.prototype.setEndDataSourceCode = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string end_record_id = 4;
 * @return {string}
 */
proto.szengine.FindPathByRecordIdRequest.prototype.getEndRecordId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindPathByRecordIdRequest} returns this
 */
proto.szengine.FindPathByRecordIdRequest.prototype.setEndRecordId = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional int64 max_degrees = 5;
 * @return {number}
 */
proto.szengine.FindPathByRecordIdRequest.prototype.getMaxDegrees = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 5, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindPathByRecordIdRequest} returns this
 */
proto.szengine.FindPathByRecordIdRequest.prototype.setMaxDegrees = function(value) {
  return jspb.Message.setProto3IntField(this, 5, value);
};


/**
 * optional string avoid_record_keys = 6;
 * @return {string}
 */
proto.szengine.FindPathByRecordIdRequest.prototype.getAvoidRecordKeys = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindPathByRecordIdRequest} returns this
 */
proto.szengine.FindPathByRecordIdRequest.prototype.setAvoidRecordKeys = function(value) {
  return jspb.Message.setProto3StringField(this, 6, value);
};


/**
 * optional string required_data_sources = 7;
 * @return {string}
 */
proto.szengine.FindPathByRecordIdRequest.prototype.getRequiredDataSources = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 7, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindPathByRecordIdRequest} returns this
 */
proto.szengine.FindPathByRecordIdRequest.prototype.setRequiredDataSources = function(value) {
  return jspb.Message.setProto3StringField(this, 7, value);
};


/**
 * optional int64 flags = 8;
 * @return {number}
 */
proto.szengine.FindPathByRecordIdRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 8, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.FindPathByRecordIdRequest} returns this
 */
proto.szengine.FindPathByRecordIdRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 8, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.FindPathByRecordIdResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.FindPathByRecordIdResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.FindPathByRecordIdResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindPathByRecordIdResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.FindPathByRecordIdResponse}
 */
proto.szengine.FindPathByRecordIdResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.FindPathByRecordIdResponse;
  return proto.szengine.FindPathByRecordIdResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.FindPathByRecordIdResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.FindPathByRecordIdResponse}
 */
proto.szengine.FindPathByRecordIdResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.FindPathByRecordIdResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.FindPathByRecordIdResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.FindPathByRecordIdResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.FindPathByRecordIdResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.FindPathByRecordIdResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.FindPathByRecordIdResponse} returns this
 */
proto.szengine.FindPathByRecordIdResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetActiveConfigIdRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetActiveConfigIdRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetActiveConfigIdRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetActiveConfigIdRequest.toObject = function(includeInstance, msg) {
  var f, obj = {

  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetActiveConfigIdRequest}
 */
proto.szengine.GetActiveConfigIdRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetActiveConfigIdRequest;
  return proto.szengine.GetActiveConfigIdRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetActiveConfigIdRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetActiveConfigIdRequest}
 */
proto.szengine.GetActiveConfigIdRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetActiveConfigIdRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetActiveConfigIdRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetActiveConfigIdRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetActiveConfigIdRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetActiveConfigIdResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetActiveConfigIdResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetActiveConfigIdResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetActiveConfigIdResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetActiveConfigIdResponse}
 */
proto.szengine.GetActiveConfigIdResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetActiveConfigIdResponse;
  return proto.szengine.GetActiveConfigIdResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetActiveConfigIdResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetActiveConfigIdResponse}
 */
proto.szengine.GetActiveConfigIdResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetActiveConfigIdResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetActiveConfigIdResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetActiveConfigIdResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetActiveConfigIdResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
};


/**
 * optional int64 result = 1;
 * @return {number}
 */
proto.szengine.GetActiveConfigIdResponse.prototype.getResult = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.GetActiveConfigIdResponse} returns this
 */
proto.szengine.GetActiveConfigIdResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetEntityByEntityIdRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetEntityByEntityIdRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetEntityByEntityIdRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetEntityByEntityIdRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    entityId: jspb.Message.getFieldWithDefault(msg, 1, 0),
    flags: jspb.Message.getFieldWithDefault(msg, 2, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetEntityByEntityIdRequest}
 */
proto.szengine.GetEntityByEntityIdRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetEntityByEntityIdRequest;
  return proto.szengine.GetEntityByEntityIdRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetEntityByEntityIdRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetEntityByEntityIdRequest}
 */
proto.szengine.GetEntityByEntityIdRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setEntityId(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetEntityByEntityIdRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetEntityByEntityIdRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetEntityByEntityIdRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetEntityByEntityIdRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getEntityId();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
};


/**
 * optional int64 entity_id = 1;
 * @return {number}
 */
proto.szengine.GetEntityByEntityIdRequest.prototype.getEntityId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.GetEntityByEntityIdRequest} returns this
 */
proto.szengine.GetEntityByEntityIdRequest.prototype.setEntityId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int64 flags = 2;
 * @return {number}
 */
proto.szengine.GetEntityByEntityIdRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.GetEntityByEntityIdRequest} returns this
 */
proto.szengine.GetEntityByEntityIdRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetEntityByEntityIdResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetEntityByEntityIdResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetEntityByEntityIdResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetEntityByEntityIdResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetEntityByEntityIdResponse}
 */
proto.szengine.GetEntityByEntityIdResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetEntityByEntityIdResponse;
  return proto.szengine.GetEntityByEntityIdResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetEntityByEntityIdResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetEntityByEntityIdResponse}
 */
proto.szengine.GetEntityByEntityIdResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetEntityByEntityIdResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetEntityByEntityIdResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetEntityByEntityIdResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetEntityByEntityIdResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.GetEntityByEntityIdResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.GetEntityByEntityIdResponse} returns this
 */
proto.szengine.GetEntityByEntityIdResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetEntityByRecordIdRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetEntityByRecordIdRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetEntityByRecordIdRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetEntityByRecordIdRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    dataSourceCode: jspb.Message.getFieldWithDefault(msg, 1, ""),
    recordId: jspb.Message.getFieldWithDefault(msg, 2, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 3, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetEntityByRecordIdRequest}
 */
proto.szengine.GetEntityByRecordIdRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetEntityByRecordIdRequest;
  return proto.szengine.GetEntityByRecordIdRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetEntityByRecordIdRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetEntityByRecordIdRequest}
 */
proto.szengine.GetEntityByRecordIdRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setDataSourceCode(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setRecordId(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetEntityByRecordIdRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetEntityByRecordIdRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetEntityByRecordIdRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetEntityByRecordIdRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDataSourceCode();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getRecordId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
};


/**
 * optional string data_source_code = 1;
 * @return {string}
 */
proto.szengine.GetEntityByRecordIdRequest.prototype.getDataSourceCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.GetEntityByRecordIdRequest} returns this
 */
proto.szengine.GetEntityByRecordIdRequest.prototype.setDataSourceCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string record_id = 2;
 * @return {string}
 */
proto.szengine.GetEntityByRecordIdRequest.prototype.getRecordId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.GetEntityByRecordIdRequest} returns this
 */
proto.szengine.GetEntityByRecordIdRequest.prototype.setRecordId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional int64 flags = 3;
 * @return {number}
 */
proto.szengine.GetEntityByRecordIdRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.GetEntityByRecordIdRequest} returns this
 */
proto.szengine.GetEntityByRecordIdRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetEntityByRecordIdResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetEntityByRecordIdResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetEntityByRecordIdResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetEntityByRecordIdResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetEntityByRecordIdResponse}
 */
proto.szengine.GetEntityByRecordIdResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetEntityByRecordIdResponse;
  return proto.szengine.GetEntityByRecordIdResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetEntityByRecordIdResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetEntityByRecordIdResponse}
 */
proto.szengine.GetEntityByRecordIdResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetEntityByRecordIdResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetEntityByRecordIdResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetEntityByRecordIdResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetEntityByRecordIdResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.GetEntityByRecordIdResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.GetEntityByRecordIdResponse} returns this
 */
proto.szengine.GetEntityByRecordIdResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetRecordPreviewRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetRecordPreviewRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetRecordPreviewRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetRecordPreviewRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    recordDefinition: jspb.Message.getFieldWithDefault(msg, 1, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 2, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetRecordPreviewRequest}
 */
proto.szengine.GetRecordPreviewRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetRecordPreviewRequest;
  return proto.szengine.GetRecordPreviewRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetRecordPreviewRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetRecordPreviewRequest}
 */
proto.szengine.GetRecordPreviewRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setRecordDefinition(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetRecordPreviewRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetRecordPreviewRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetRecordPreviewRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetRecordPreviewRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getRecordDefinition();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
};


/**
 * optional string record_definition = 1;
 * @return {string}
 */
proto.szengine.GetRecordPreviewRequest.prototype.getRecordDefinition = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.GetRecordPreviewRequest} returns this
 */
proto.szengine.GetRecordPreviewRequest.prototype.setRecordDefinition = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int64 flags = 2;
 * @return {number}
 */
proto.szengine.GetRecordPreviewRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.GetRecordPreviewRequest} returns this
 */
proto.szengine.GetRecordPreviewRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetRecordPreviewResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetRecordPreviewResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetRecordPreviewResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetRecordPreviewResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetRecordPreviewResponse}
 */
proto.szengine.GetRecordPreviewResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetRecordPreviewResponse;
  return proto.szengine.GetRecordPreviewResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetRecordPreviewResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetRecordPreviewResponse}
 */
proto.szengine.GetRecordPreviewResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetRecordPreviewResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetRecordPreviewResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetRecordPreviewResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetRecordPreviewResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.GetRecordPreviewResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.GetRecordPreviewResponse} returns this
 */
proto.szengine.GetRecordPreviewResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetRecordRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetRecordRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetRecordRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetRecordRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    dataSourceCode: jspb.Message.getFieldWithDefault(msg, 1, ""),
    recordId: jspb.Message.getFieldWithDefault(msg, 2, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 3, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetRecordRequest}
 */
proto.szengine.GetRecordRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetRecordRequest;
  return proto.szengine.GetRecordRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetRecordRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetRecordRequest}
 */
proto.szengine.GetRecordRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setDataSourceCode(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setRecordId(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetRecordRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetRecordRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetRecordRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetRecordRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDataSourceCode();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getRecordId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
};


/**
 * optional string data_source_code = 1;
 * @return {string}
 */
proto.szengine.GetRecordRequest.prototype.getDataSourceCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.GetRecordRequest} returns this
 */
proto.szengine.GetRecordRequest.prototype.setDataSourceCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string record_id = 2;
 * @return {string}
 */
proto.szengine.GetRecordRequest.prototype.getRecordId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.GetRecordRequest} returns this
 */
proto.szengine.GetRecordRequest.prototype.setRecordId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional int64 flags = 3;
 * @return {number}
 */
proto.szengine.GetRecordRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.GetRecordRequest} returns this
 */
proto.szengine.GetRecordRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetRecordResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetRecordResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetRecordResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetRecordResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetRecordResponse}
 */
proto.szengine.GetRecordResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetRecordResponse;
  return proto.szengine.GetRecordResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetRecordResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetRecordResponse}
 */
proto.szengine.GetRecordResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetRecordResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetRecordResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetRecordResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetRecordResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.GetRecordResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.GetRecordResponse} returns this
 */
proto.szengine.GetRecordResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetRedoRecordRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetRedoRecordRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetRedoRecordRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetRedoRecordRequest.toObject = function(includeInstance, msg) {
  var f, obj = {

  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetRedoRecordRequest}
 */
proto.szengine.GetRedoRecordRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetRedoRecordRequest;
  return proto.szengine.GetRedoRecordRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetRedoRecordRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetRedoRecordRequest}
 */
proto.szengine.GetRedoRecordRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetRedoRecordRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetRedoRecordRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetRedoRecordRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetRedoRecordRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetRedoRecordResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetRedoRecordResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetRedoRecordResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetRedoRecordResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetRedoRecordResponse}
 */
proto.szengine.GetRedoRecordResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetRedoRecordResponse;
  return proto.szengine.GetRedoRecordResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetRedoRecordResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetRedoRecordResponse}
 */
proto.szengine.GetRedoRecordResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetRedoRecordResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetRedoRecordResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetRedoRecordResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetRedoRecordResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.GetRedoRecordResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.GetRedoRecordResponse} returns this
 */
proto.szengine.GetRedoRecordResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetStatsRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetStatsRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetStatsRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetStatsRequest.toObject = function(includeInstance, msg) {
  var f, obj = {

  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetStatsRequest}
 */
proto.szengine.GetStatsRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetStatsRequest;
  return proto.szengine.GetStatsRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetStatsRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetStatsRequest}
 */
proto.szengine.GetStatsRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetStatsRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetStatsRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetStatsRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetStatsRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetStatsResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetStatsResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetStatsResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetStatsResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetStatsResponse}
 */
proto.szengine.GetStatsResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetStatsResponse;
  return proto.szengine.GetStatsResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetStatsResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetStatsResponse}
 */
proto.szengine.GetStatsResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetStatsResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetStatsResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetStatsResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetStatsResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.GetStatsResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.GetStatsResponse} returns this
 */
proto.szengine.GetStatsResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetVirtualEntityByRecordIdRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetVirtualEntityByRecordIdRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetVirtualEntityByRecordIdRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetVirtualEntityByRecordIdRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    recordKeys: jspb.Message.getFieldWithDefault(msg, 1, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 2, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetVirtualEntityByRecordIdRequest}
 */
proto.szengine.GetVirtualEntityByRecordIdRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetVirtualEntityByRecordIdRequest;
  return proto.szengine.GetVirtualEntityByRecordIdRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetVirtualEntityByRecordIdRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetVirtualEntityByRecordIdRequest}
 */
proto.szengine.GetVirtualEntityByRecordIdRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setRecordKeys(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetVirtualEntityByRecordIdRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetVirtualEntityByRecordIdRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetVirtualEntityByRecordIdRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetVirtualEntityByRecordIdRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getRecordKeys();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
};


/**
 * optional string record_keys = 1;
 * @return {string}
 */
proto.szengine.GetVirtualEntityByRecordIdRequest.prototype.getRecordKeys = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.GetVirtualEntityByRecordIdRequest} returns this
 */
proto.szengine.GetVirtualEntityByRecordIdRequest.prototype.setRecordKeys = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int64 flags = 2;
 * @return {number}
 */
proto.szengine.GetVirtualEntityByRecordIdRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.GetVirtualEntityByRecordIdRequest} returns this
 */
proto.szengine.GetVirtualEntityByRecordIdRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.GetVirtualEntityByRecordIdResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.GetVirtualEntityByRecordIdResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.GetVirtualEntityByRecordIdResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetVirtualEntityByRecordIdResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.GetVirtualEntityByRecordIdResponse}
 */
proto.szengine.GetVirtualEntityByRecordIdResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.GetVirtualEntityByRecordIdResponse;
  return proto.szengine.GetVirtualEntityByRecordIdResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.GetVirtualEntityByRecordIdResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.GetVirtualEntityByRecordIdResponse}
 */
proto.szengine.GetVirtualEntityByRecordIdResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.GetVirtualEntityByRecordIdResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.GetVirtualEntityByRecordIdResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.GetVirtualEntityByRecordIdResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.GetVirtualEntityByRecordIdResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.GetVirtualEntityByRecordIdResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.GetVirtualEntityByRecordIdResponse} returns this
 */
proto.szengine.GetVirtualEntityByRecordIdResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.HowEntityByEntityIdRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.HowEntityByEntityIdRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.HowEntityByEntityIdRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.HowEntityByEntityIdRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    entityId: jspb.Message.getFieldWithDefault(msg, 1, 0),
    flags: jspb.Message.getFieldWithDefault(msg, 2, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.HowEntityByEntityIdRequest}
 */
proto.szengine.HowEntityByEntityIdRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.HowEntityByEntityIdRequest;
  return proto.szengine.HowEntityByEntityIdRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.HowEntityByEntityIdRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.HowEntityByEntityIdRequest}
 */
proto.szengine.HowEntityByEntityIdRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setEntityId(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.HowEntityByEntityIdRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.HowEntityByEntityIdRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.HowEntityByEntityIdRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.HowEntityByEntityIdRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getEntityId();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
};


/**
 * optional int64 entity_id = 1;
 * @return {number}
 */
proto.szengine.HowEntityByEntityIdRequest.prototype.getEntityId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.HowEntityByEntityIdRequest} returns this
 */
proto.szengine.HowEntityByEntityIdRequest.prototype.setEntityId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int64 flags = 2;
 * @return {number}
 */
proto.szengine.HowEntityByEntityIdRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.HowEntityByEntityIdRequest} returns this
 */
proto.szengine.HowEntityByEntityIdRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.HowEntityByEntityIdResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.HowEntityByEntityIdResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.HowEntityByEntityIdResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.HowEntityByEntityIdResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.HowEntityByEntityIdResponse}
 */
proto.szengine.HowEntityByEntityIdResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.HowEntityByEntityIdResponse;
  return proto.szengine.HowEntityByEntityIdResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.HowEntityByEntityIdResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.HowEntityByEntityIdResponse}
 */
proto.szengine.HowEntityByEntityIdResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.HowEntityByEntityIdResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.HowEntityByEntityIdResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.HowEntityByEntityIdResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.HowEntityByEntityIdResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.HowEntityByEntityIdResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.HowEntityByEntityIdResponse} returns this
 */
proto.szengine.HowEntityByEntityIdResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.PrimeEngineRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.PrimeEngineRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.PrimeEngineRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.PrimeEngineRequest.toObject = function(includeInstance, msg) {
  var f, obj = {

  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.PrimeEngineRequest}
 */
proto.szengine.PrimeEngineRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.PrimeEngineRequest;
  return proto.szengine.PrimeEngineRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.PrimeEngineRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.PrimeEngineRequest}
 */
proto.szengine.PrimeEngineRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.PrimeEngineRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.PrimeEngineRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.PrimeEngineRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.PrimeEngineRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.PrimeEngineResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.PrimeEngineResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.PrimeEngineResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.PrimeEngineResponse.toObject = function(includeInstance, msg) {
  var f, obj = {

  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.PrimeEngineResponse}
 */
proto.szengine.PrimeEngineResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.PrimeEngineResponse;
  return proto.szengine.PrimeEngineResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.PrimeEngineResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.PrimeEngineResponse}
 */
proto.szengine.PrimeEngineResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.PrimeEngineResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.PrimeEngineResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.PrimeEngineResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.PrimeEngineResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.ProcessRedoRecordRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.ProcessRedoRecordRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.ProcessRedoRecordRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ProcessRedoRecordRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    redoRecord: jspb.Message.getFieldWithDefault(msg, 1, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 2, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.ProcessRedoRecordRequest}
 */
proto.szengine.ProcessRedoRecordRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.ProcessRedoRecordRequest;
  return proto.szengine.ProcessRedoRecordRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.ProcessRedoRecordRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.ProcessRedoRecordRequest}
 */
proto.szengine.ProcessRedoRecordRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setRedoRecord(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.ProcessRedoRecordRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.ProcessRedoRecordRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.ProcessRedoRecordRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ProcessRedoRecordRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getRedoRecord();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
};


/**
 * optional string redo_record = 1;
 * @return {string}
 */
proto.szengine.ProcessRedoRecordRequest.prototype.getRedoRecord = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.ProcessRedoRecordRequest} returns this
 */
proto.szengine.ProcessRedoRecordRequest.prototype.setRedoRecord = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int64 flags = 2;
 * @return {number}
 */
proto.szengine.ProcessRedoRecordRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.ProcessRedoRecordRequest} returns this
 */
proto.szengine.ProcessRedoRecordRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.ProcessRedoRecordResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.ProcessRedoRecordResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.ProcessRedoRecordResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ProcessRedoRecordResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.ProcessRedoRecordResponse}
 */
proto.szengine.ProcessRedoRecordResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.ProcessRedoRecordResponse;
  return proto.szengine.ProcessRedoRecordResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.ProcessRedoRecordResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.ProcessRedoRecordResponse}
 */
proto.szengine.ProcessRedoRecordResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.ProcessRedoRecordResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.ProcessRedoRecordResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.ProcessRedoRecordResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ProcessRedoRecordResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.ProcessRedoRecordResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.ProcessRedoRecordResponse} returns this
 */
proto.szengine.ProcessRedoRecordResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.ReevaluateEntityRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.ReevaluateEntityRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.ReevaluateEntityRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ReevaluateEntityRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    entityId: jspb.Message.getFieldWithDefault(msg, 1, 0),
    flags: jspb.Message.getFieldWithDefault(msg, 2, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.ReevaluateEntityRequest}
 */
proto.szengine.ReevaluateEntityRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.ReevaluateEntityRequest;
  return proto.szengine.ReevaluateEntityRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.ReevaluateEntityRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.ReevaluateEntityRequest}
 */
proto.szengine.ReevaluateEntityRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setEntityId(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.ReevaluateEntityRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.ReevaluateEntityRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.ReevaluateEntityRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ReevaluateEntityRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getEntityId();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
};


/**
 * optional int64 entity_id = 1;
 * @return {number}
 */
proto.szengine.ReevaluateEntityRequest.prototype.getEntityId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.ReevaluateEntityRequest} returns this
 */
proto.szengine.ReevaluateEntityRequest.prototype.setEntityId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int64 flags = 2;
 * @return {number}
 */
proto.szengine.ReevaluateEntityRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.ReevaluateEntityRequest} returns this
 */
proto.szengine.ReevaluateEntityRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.ReevaluateEntityResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.ReevaluateEntityResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.ReevaluateEntityResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ReevaluateEntityResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.ReevaluateEntityResponse}
 */
proto.szengine.ReevaluateEntityResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.ReevaluateEntityResponse;
  return proto.szengine.ReevaluateEntityResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.ReevaluateEntityResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.ReevaluateEntityResponse}
 */
proto.szengine.ReevaluateEntityResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.ReevaluateEntityResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.ReevaluateEntityResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.ReevaluateEntityResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ReevaluateEntityResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.ReevaluateEntityResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.ReevaluateEntityResponse} returns this
 */
proto.szengine.ReevaluateEntityResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.ReevaluateRecordRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.ReevaluateRecordRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.ReevaluateRecordRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ReevaluateRecordRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    dataSourceCode: jspb.Message.getFieldWithDefault(msg, 1, ""),
    recordId: jspb.Message.getFieldWithDefault(msg, 2, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 3, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.ReevaluateRecordRequest}
 */
proto.szengine.ReevaluateRecordRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.ReevaluateRecordRequest;
  return proto.szengine.ReevaluateRecordRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.ReevaluateRecordRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.ReevaluateRecordRequest}
 */
proto.szengine.ReevaluateRecordRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setDataSourceCode(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setRecordId(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.ReevaluateRecordRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.ReevaluateRecordRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.ReevaluateRecordRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ReevaluateRecordRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDataSourceCode();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getRecordId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
};


/**
 * optional string data_source_code = 1;
 * @return {string}
 */
proto.szengine.ReevaluateRecordRequest.prototype.getDataSourceCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.ReevaluateRecordRequest} returns this
 */
proto.szengine.ReevaluateRecordRequest.prototype.setDataSourceCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string record_id = 2;
 * @return {string}
 */
proto.szengine.ReevaluateRecordRequest.prototype.getRecordId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.ReevaluateRecordRequest} returns this
 */
proto.szengine.ReevaluateRecordRequest.prototype.setRecordId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional int64 flags = 3;
 * @return {number}
 */
proto.szengine.ReevaluateRecordRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.ReevaluateRecordRequest} returns this
 */
proto.szengine.ReevaluateRecordRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.ReevaluateRecordResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.ReevaluateRecordResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.ReevaluateRecordResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ReevaluateRecordResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.ReevaluateRecordResponse}
 */
proto.szengine.ReevaluateRecordResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.ReevaluateRecordResponse;
  return proto.szengine.ReevaluateRecordResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.ReevaluateRecordResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.ReevaluateRecordResponse}
 */
proto.szengine.ReevaluateRecordResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.ReevaluateRecordResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.ReevaluateRecordResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.ReevaluateRecordResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ReevaluateRecordResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.ReevaluateRecordResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.ReevaluateRecordResponse} returns this
 */
proto.szengine.ReevaluateRecordResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.ReinitializeRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.ReinitializeRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.ReinitializeRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ReinitializeRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    configId: jspb.Message.getFieldWithDefault(msg, 1, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.ReinitializeRequest}
 */
proto.szengine.ReinitializeRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.ReinitializeRequest;
  return proto.szengine.ReinitializeRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.ReinitializeRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.ReinitializeRequest}
 */
proto.szengine.ReinitializeRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setConfigId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.ReinitializeRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.ReinitializeRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.ReinitializeRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ReinitializeRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getConfigId();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
};


/**
 * optional int64 config_id = 1;
 * @return {number}
 */
proto.szengine.ReinitializeRequest.prototype.getConfigId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.ReinitializeRequest} returns this
 */
proto.szengine.ReinitializeRequest.prototype.setConfigId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.ReinitializeResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.ReinitializeResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.ReinitializeResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ReinitializeResponse.toObject = function(includeInstance, msg) {
  var f, obj = {

  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.ReinitializeResponse}
 */
proto.szengine.ReinitializeResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.ReinitializeResponse;
  return proto.szengine.ReinitializeResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.ReinitializeResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.ReinitializeResponse}
 */
proto.szengine.ReinitializeResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.ReinitializeResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.ReinitializeResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.ReinitializeResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.ReinitializeResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.SearchByAttributesRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.SearchByAttributesRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.SearchByAttributesRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.SearchByAttributesRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    attributes: jspb.Message.getFieldWithDefault(msg, 1, ""),
    searchProfile: jspb.Message.getFieldWithDefault(msg, 2, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 3, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.SearchByAttributesRequest}
 */
proto.szengine.SearchByAttributesRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.SearchByAttributesRequest;
  return proto.szengine.SearchByAttributesRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.SearchByAttributesRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.SearchByAttributesRequest}
 */
proto.szengine.SearchByAttributesRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setAttributes(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setSearchProfile(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.SearchByAttributesRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.SearchByAttributesRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.SearchByAttributesRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.SearchByAttributesRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getAttributes();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getSearchProfile();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
};


/**
 * optional string attributes = 1;
 * @return {string}
 */
proto.szengine.SearchByAttributesRequest.prototype.getAttributes = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.SearchByAttributesRequest} returns this
 */
proto.szengine.SearchByAttributesRequest.prototype.setAttributes = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string search_profile = 2;
 * @return {string}
 */
proto.szengine.SearchByAttributesRequest.prototype.getSearchProfile = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.SearchByAttributesRequest} returns this
 */
proto.szengine.SearchByAttributesRequest.prototype.setSearchProfile = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional int64 flags = 3;
 * @return {number}
 */
proto.szengine.SearchByAttributesRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.SearchByAttributesRequest} returns this
 */
proto.szengine.SearchByAttributesRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.SearchByAttributesResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.SearchByAttributesResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.SearchByAttributesResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.SearchByAttributesResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.SearchByAttributesResponse}
 */
proto.szengine.SearchByAttributesResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.SearchByAttributesResponse;
  return proto.szengine.SearchByAttributesResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.SearchByAttributesResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.SearchByAttributesResponse}
 */
proto.szengine.SearchByAttributesResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.SearchByAttributesResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.SearchByAttributesResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.SearchByAttributesResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.SearchByAttributesResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.SearchByAttributesResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.SearchByAttributesResponse} returns this
 */
proto.szengine.SearchByAttributesResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.StreamExportCsvEntityReportRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.StreamExportCsvEntityReportRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.StreamExportCsvEntityReportRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.StreamExportCsvEntityReportRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    csvColumnList: jspb.Message.getFieldWithDefault(msg, 1, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 2, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.StreamExportCsvEntityReportRequest}
 */
proto.szengine.StreamExportCsvEntityReportRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.StreamExportCsvEntityReportRequest;
  return proto.szengine.StreamExportCsvEntityReportRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.StreamExportCsvEntityReportRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.StreamExportCsvEntityReportRequest}
 */
proto.szengine.StreamExportCsvEntityReportRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setCsvColumnList(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.StreamExportCsvEntityReportRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.StreamExportCsvEntityReportRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.StreamExportCsvEntityReportRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.StreamExportCsvEntityReportRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getCsvColumnList();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
};


/**
 * optional string csv_column_list = 1;
 * @return {string}
 */
proto.szengine.StreamExportCsvEntityReportRequest.prototype.getCsvColumnList = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.StreamExportCsvEntityReportRequest} returns this
 */
proto.szengine.StreamExportCsvEntityReportRequest.prototype.setCsvColumnList = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int64 flags = 2;
 * @return {number}
 */
proto.szengine.StreamExportCsvEntityReportRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.StreamExportCsvEntityReportRequest} returns this
 */
proto.szengine.StreamExportCsvEntityReportRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.StreamExportCsvEntityReportResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.StreamExportCsvEntityReportResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.StreamExportCsvEntityReportResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.StreamExportCsvEntityReportResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.StreamExportCsvEntityReportResponse}
 */
proto.szengine.StreamExportCsvEntityReportResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.StreamExportCsvEntityReportResponse;
  return proto.szengine.StreamExportCsvEntityReportResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.StreamExportCsvEntityReportResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.StreamExportCsvEntityReportResponse}
 */
proto.szengine.StreamExportCsvEntityReportResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.StreamExportCsvEntityReportResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.StreamExportCsvEntityReportResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.StreamExportCsvEntityReportResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.StreamExportCsvEntityReportResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.StreamExportCsvEntityReportResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.StreamExportCsvEntityReportResponse} returns this
 */
proto.szengine.StreamExportCsvEntityReportResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.StreamExportJsonEntityReportRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.StreamExportJsonEntityReportRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.StreamExportJsonEntityReportRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.StreamExportJsonEntityReportRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    flags: jspb.Message.getFieldWithDefault(msg, 1, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.StreamExportJsonEntityReportRequest}
 */
proto.szengine.StreamExportJsonEntityReportRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.StreamExportJsonEntityReportRequest;
  return proto.szengine.StreamExportJsonEntityReportRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.StreamExportJsonEntityReportRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.StreamExportJsonEntityReportRequest}
 */
proto.szengine.StreamExportJsonEntityReportRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.StreamExportJsonEntityReportRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.StreamExportJsonEntityReportRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.StreamExportJsonEntityReportRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.StreamExportJsonEntityReportRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
};


/**
 * optional int64 flags = 1;
 * @return {number}
 */
proto.szengine.StreamExportJsonEntityReportRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.StreamExportJsonEntityReportRequest} returns this
 */
proto.szengine.StreamExportJsonEntityReportRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.StreamExportJsonEntityReportResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.StreamExportJsonEntityReportResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.StreamExportJsonEntityReportResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.StreamExportJsonEntityReportResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.StreamExportJsonEntityReportResponse}
 */
proto.szengine.StreamExportJsonEntityReportResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.StreamExportJsonEntityReportResponse;
  return proto.szengine.StreamExportJsonEntityReportResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.StreamExportJsonEntityReportResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.StreamExportJsonEntityReportResponse}
 */
proto.szengine.StreamExportJsonEntityReportResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.StreamExportJsonEntityReportResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.StreamExportJsonEntityReportResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.StreamExportJsonEntityReportResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.StreamExportJsonEntityReportResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.StreamExportJsonEntityReportResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.StreamExportJsonEntityReportResponse} returns this
 */
proto.szengine.StreamExportJsonEntityReportResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.WhyEntitiesRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.WhyEntitiesRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.WhyEntitiesRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhyEntitiesRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    entityId1: jspb.Message.getFieldWithDefault(msg, 1, 0),
    entityId2: jspb.Message.getFieldWithDefault(msg, 2, 0),
    flags: jspb.Message.getFieldWithDefault(msg, 3, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.WhyEntitiesRequest}
 */
proto.szengine.WhyEntitiesRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.WhyEntitiesRequest;
  return proto.szengine.WhyEntitiesRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.WhyEntitiesRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.WhyEntitiesRequest}
 */
proto.szengine.WhyEntitiesRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setEntityId1(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setEntityId2(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.WhyEntitiesRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.WhyEntitiesRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.WhyEntitiesRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhyEntitiesRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getEntityId1();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getEntityId2();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
};


/**
 * optional int64 entity_id_1 = 1;
 * @return {number}
 */
proto.szengine.WhyEntitiesRequest.prototype.getEntityId1 = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.WhyEntitiesRequest} returns this
 */
proto.szengine.WhyEntitiesRequest.prototype.setEntityId1 = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int64 entity_id_2 = 2;
 * @return {number}
 */
proto.szengine.WhyEntitiesRequest.prototype.getEntityId2 = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.WhyEntitiesRequest} returns this
 */
proto.szengine.WhyEntitiesRequest.prototype.setEntityId2 = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional int64 flags = 3;
 * @return {number}
 */
proto.szengine.WhyEntitiesRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.WhyEntitiesRequest} returns this
 */
proto.szengine.WhyEntitiesRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.WhyEntitiesResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.WhyEntitiesResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.WhyEntitiesResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhyEntitiesResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.WhyEntitiesResponse}
 */
proto.szengine.WhyEntitiesResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.WhyEntitiesResponse;
  return proto.szengine.WhyEntitiesResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.WhyEntitiesResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.WhyEntitiesResponse}
 */
proto.szengine.WhyEntitiesResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.WhyEntitiesResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.WhyEntitiesResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.WhyEntitiesResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhyEntitiesResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.WhyEntitiesResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.WhyEntitiesResponse} returns this
 */
proto.szengine.WhyEntitiesResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.WhyRecordInEntityRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.WhyRecordInEntityRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.WhyRecordInEntityRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhyRecordInEntityRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    dataSourceCode: jspb.Message.getFieldWithDefault(msg, 1, ""),
    recordId: jspb.Message.getFieldWithDefault(msg, 2, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 3, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.WhyRecordInEntityRequest}
 */
proto.szengine.WhyRecordInEntityRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.WhyRecordInEntityRequest;
  return proto.szengine.WhyRecordInEntityRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.WhyRecordInEntityRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.WhyRecordInEntityRequest}
 */
proto.szengine.WhyRecordInEntityRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setDataSourceCode(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setRecordId(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.WhyRecordInEntityRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.WhyRecordInEntityRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.WhyRecordInEntityRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhyRecordInEntityRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDataSourceCode();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getRecordId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
};


/**
 * optional string data_source_code = 1;
 * @return {string}
 */
proto.szengine.WhyRecordInEntityRequest.prototype.getDataSourceCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.WhyRecordInEntityRequest} returns this
 */
proto.szengine.WhyRecordInEntityRequest.prototype.setDataSourceCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string record_id = 2;
 * @return {string}
 */
proto.szengine.WhyRecordInEntityRequest.prototype.getRecordId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.WhyRecordInEntityRequest} returns this
 */
proto.szengine.WhyRecordInEntityRequest.prototype.setRecordId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional int64 flags = 3;
 * @return {number}
 */
proto.szengine.WhyRecordInEntityRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.WhyRecordInEntityRequest} returns this
 */
proto.szengine.WhyRecordInEntityRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.WhyRecordInEntityResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.WhyRecordInEntityResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.WhyRecordInEntityResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhyRecordInEntityResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.WhyRecordInEntityResponse}
 */
proto.szengine.WhyRecordInEntityResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.WhyRecordInEntityResponse;
  return proto.szengine.WhyRecordInEntityResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.WhyRecordInEntityResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.WhyRecordInEntityResponse}
 */
proto.szengine.WhyRecordInEntityResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.WhyRecordInEntityResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.WhyRecordInEntityResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.WhyRecordInEntityResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhyRecordInEntityResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.WhyRecordInEntityResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.WhyRecordInEntityResponse} returns this
 */
proto.szengine.WhyRecordInEntityResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.WhyRecordsRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.WhyRecordsRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.WhyRecordsRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhyRecordsRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    dataSourceCode1: jspb.Message.getFieldWithDefault(msg, 1, ""),
    recordId1: jspb.Message.getFieldWithDefault(msg, 2, ""),
    dataSourceCode2: jspb.Message.getFieldWithDefault(msg, 3, ""),
    recordId2: jspb.Message.getFieldWithDefault(msg, 4, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 5, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.WhyRecordsRequest}
 */
proto.szengine.WhyRecordsRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.WhyRecordsRequest;
  return proto.szengine.WhyRecordsRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.WhyRecordsRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.WhyRecordsRequest}
 */
proto.szengine.WhyRecordsRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setDataSourceCode1(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setRecordId1(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setDataSourceCode2(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setRecordId2(value);
      break;
    case 5:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.WhyRecordsRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.WhyRecordsRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.WhyRecordsRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhyRecordsRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDataSourceCode1();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getRecordId1();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getDataSourceCode2();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getRecordId2();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      5,
      f
    );
  }
};


/**
 * optional string data_source_code_1 = 1;
 * @return {string}
 */
proto.szengine.WhyRecordsRequest.prototype.getDataSourceCode1 = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.WhyRecordsRequest} returns this
 */
proto.szengine.WhyRecordsRequest.prototype.setDataSourceCode1 = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string record_id_1 = 2;
 * @return {string}
 */
proto.szengine.WhyRecordsRequest.prototype.getRecordId1 = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.WhyRecordsRequest} returns this
 */
proto.szengine.WhyRecordsRequest.prototype.setRecordId1 = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string data_source_code_2 = 3;
 * @return {string}
 */
proto.szengine.WhyRecordsRequest.prototype.getDataSourceCode2 = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.WhyRecordsRequest} returns this
 */
proto.szengine.WhyRecordsRequest.prototype.setDataSourceCode2 = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string record_id_2 = 4;
 * @return {string}
 */
proto.szengine.WhyRecordsRequest.prototype.getRecordId2 = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.WhyRecordsRequest} returns this
 */
proto.szengine.WhyRecordsRequest.prototype.setRecordId2 = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional int64 flags = 5;
 * @return {number}
 */
proto.szengine.WhyRecordsRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 5, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.WhyRecordsRequest} returns this
 */
proto.szengine.WhyRecordsRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 5, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.WhyRecordsResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.WhyRecordsResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.WhyRecordsResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhyRecordsResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.WhyRecordsResponse}
 */
proto.szengine.WhyRecordsResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.WhyRecordsResponse;
  return proto.szengine.WhyRecordsResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.WhyRecordsResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.WhyRecordsResponse}
 */
proto.szengine.WhyRecordsResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.WhyRecordsResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.WhyRecordsResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.WhyRecordsResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhyRecordsResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.WhyRecordsResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.WhyRecordsResponse} returns this
 */
proto.szengine.WhyRecordsResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.WhySearchRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.WhySearchRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.WhySearchRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhySearchRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    attributes: jspb.Message.getFieldWithDefault(msg, 1, ""),
    entityId: jspb.Message.getFieldWithDefault(msg, 2, 0),
    searchProfile: jspb.Message.getFieldWithDefault(msg, 3, ""),
    flags: jspb.Message.getFieldWithDefault(msg, 4, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.WhySearchRequest}
 */
proto.szengine.WhySearchRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.WhySearchRequest;
  return proto.szengine.WhySearchRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.WhySearchRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.WhySearchRequest}
 */
proto.szengine.WhySearchRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setAttributes(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setEntityId(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setSearchProfile(value);
      break;
    case 4:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFlags(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.WhySearchRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.WhySearchRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.WhySearchRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhySearchRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getAttributes();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getEntityId();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getSearchProfile();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getFlags();
  if (f !== 0) {
    writer.writeInt64(
      4,
      f
    );
  }
};


/**
 * optional string attributes = 1;
 * @return {string}
 */
proto.szengine.WhySearchRequest.prototype.getAttributes = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.WhySearchRequest} returns this
 */
proto.szengine.WhySearchRequest.prototype.setAttributes = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int64 entity_id = 2;
 * @return {number}
 */
proto.szengine.WhySearchRequest.prototype.getEntityId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.WhySearchRequest} returns this
 */
proto.szengine.WhySearchRequest.prototype.setEntityId = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional string search_profile = 3;
 * @return {string}
 */
proto.szengine.WhySearchRequest.prototype.getSearchProfile = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.WhySearchRequest} returns this
 */
proto.szengine.WhySearchRequest.prototype.setSearchProfile = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional int64 flags = 4;
 * @return {number}
 */
proto.szengine.WhySearchRequest.prototype.getFlags = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 4, 0));
};


/**
 * @param {number} value
 * @return {!proto.szengine.WhySearchRequest} returns this
 */
proto.szengine.WhySearchRequest.prototype.setFlags = function(value) {
  return jspb.Message.setProto3IntField(this, 4, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.szengine.WhySearchResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.szengine.WhySearchResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.szengine.WhySearchResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhySearchResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.szengine.WhySearchResponse}
 */
proto.szengine.WhySearchResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.szengine.WhySearchResponse;
  return proto.szengine.WhySearchResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.szengine.WhySearchResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.szengine.WhySearchResponse}
 */
proto.szengine.WhySearchResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.szengine.WhySearchResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.szengine.WhySearchResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.szengine.WhySearchResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.szengine.WhySearchResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string result = 1;
 * @return {string}
 */
proto.szengine.WhySearchResponse.prototype.getResult = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.szengine.WhySearchResponse} returns this
 */
proto.szengine.WhySearchResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


goog.object.extend(exports, proto.szengine);
