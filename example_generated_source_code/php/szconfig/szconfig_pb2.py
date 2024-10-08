# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: szconfig.proto
# Protobuf Python Version: 5.27.2
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    27,
    2,
    '',
    'szconfig.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x0eszconfig.proto\x12\x08szconfig\"D\n\x14\x41\x64\x64\x44\x61taSourceRequest\x12\x14\n\x0c\x63onfigHandle\x18\x01 \x01(\x03\x12\x16\n\x0e\x64\x61taSourceCode\x18\x02 \x01(\t\"\'\n\x15\x41\x64\x64\x44\x61taSourceResponse\x12\x0e\n\x06result\x18\x01 \x01(\t\"*\n\x12\x43loseConfigRequest\x12\x14\n\x0c\x63onfigHandle\x18\x01 \x01(\x03\"\x15\n\x13\x43loseConfigResponse\"\x15\n\x13\x43reateConfigRequest\"&\n\x14\x43reateConfigResponse\x12\x0e\n\x06result\x18\x01 \x01(\x03\"G\n\x17\x44\x65leteDataSourceRequest\x12\x14\n\x0c\x63onfigHandle\x18\x01 \x01(\x03\x12\x16\n\x0e\x64\x61taSourceCode\x18\x02 \x01(\t\"\x1a\n\x18\x44\x65leteDataSourceResponse\"+\n\x13\x45xportConfigRequest\x12\x14\n\x0c\x63onfigHandle\x18\x01 \x01(\x03\"&\n\x14\x45xportConfigResponse\x12\x0e\n\x06result\x18\x01 \x01(\t\"-\n\x15GetDataSourcesRequest\x12\x14\n\x0c\x63onfigHandle\x18\x01 \x01(\x03\"(\n\x16GetDataSourcesResponse\x12\x0e\n\x06result\x18\x01 \x01(\t\"/\n\x13ImportConfigRequest\x12\x18\n\x10\x63onfigDefinition\x18\x01 \x01(\t\"&\n\x14ImportConfigResponse\x12\x0e\n\x06result\x18\x01 \x01(\x03\x32\xd3\x04\n\x08SzConfig\x12R\n\rAddDataSource\x12\x1e.szconfig.AddDataSourceRequest\x1a\x1f.szconfig.AddDataSourceResponse\"\x00\x12L\n\x0b\x43loseConfig\x12\x1c.szconfig.CloseConfigRequest\x1a\x1d.szconfig.CloseConfigResponse\"\x00\x12O\n\x0c\x43reateConfig\x12\x1d.szconfig.CreateConfigRequest\x1a\x1e.szconfig.CreateConfigResponse\"\x00\x12[\n\x10\x44\x65leteDataSource\x12!.szconfig.DeleteDataSourceRequest\x1a\".szconfig.DeleteDataSourceResponse\"\x00\x12O\n\x0c\x45xportConfig\x12\x1d.szconfig.ExportConfigRequest\x1a\x1e.szconfig.ExportConfigResponse\"\x00\x12U\n\x0eGetDataSources\x12\x1f.szconfig.GetDataSourcesRequest\x1a .szconfig.GetDataSourcesResponse\"\x00\x12O\n\x0cImportConfig\x12\x1d.szconfig.ImportConfigRequest\x1a\x1e.szconfig.ImportConfigResponse\"\x00\x42g\n#com.senzing.sz.engine.grpc.SzConfigB\rSzConfigProtoZ1github.com/senzing-garage/sz-sdk-go-grpc/szconfigb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'szconfig_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n#com.senzing.sz.engine.grpc.SzConfigB\rSzConfigProtoZ1github.com/senzing-garage/sz-sdk-go-grpc/szconfig'
  _globals['_ADDDATASOURCEREQUEST']._serialized_start=28
  _globals['_ADDDATASOURCEREQUEST']._serialized_end=96
  _globals['_ADDDATASOURCERESPONSE']._serialized_start=98
  _globals['_ADDDATASOURCERESPONSE']._serialized_end=137
  _globals['_CLOSECONFIGREQUEST']._serialized_start=139
  _globals['_CLOSECONFIGREQUEST']._serialized_end=181
  _globals['_CLOSECONFIGRESPONSE']._serialized_start=183
  _globals['_CLOSECONFIGRESPONSE']._serialized_end=204
  _globals['_CREATECONFIGREQUEST']._serialized_start=206
  _globals['_CREATECONFIGREQUEST']._serialized_end=227
  _globals['_CREATECONFIGRESPONSE']._serialized_start=229
  _globals['_CREATECONFIGRESPONSE']._serialized_end=267
  _globals['_DELETEDATASOURCEREQUEST']._serialized_start=269
  _globals['_DELETEDATASOURCEREQUEST']._serialized_end=340
  _globals['_DELETEDATASOURCERESPONSE']._serialized_start=342
  _globals['_DELETEDATASOURCERESPONSE']._serialized_end=368
  _globals['_EXPORTCONFIGREQUEST']._serialized_start=370
  _globals['_EXPORTCONFIGREQUEST']._serialized_end=413
  _globals['_EXPORTCONFIGRESPONSE']._serialized_start=415
  _globals['_EXPORTCONFIGRESPONSE']._serialized_end=453
  _globals['_GETDATASOURCESREQUEST']._serialized_start=455
  _globals['_GETDATASOURCESREQUEST']._serialized_end=500
  _globals['_GETDATASOURCESRESPONSE']._serialized_start=502
  _globals['_GETDATASOURCESRESPONSE']._serialized_end=542
  _globals['_IMPORTCONFIGREQUEST']._serialized_start=544
  _globals['_IMPORTCONFIGREQUEST']._serialized_end=591
  _globals['_IMPORTCONFIGRESPONSE']._serialized_start=593
  _globals['_IMPORTCONFIGRESPONSE']._serialized_end=631
  _globals['_SZCONFIG']._serialized_start=634
  _globals['_SZCONFIG']._serialized_end=1229
# @@protoc_insertion_point(module_scope)
