# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: szdiagnostic.proto
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
    'szdiagnostic.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x12szdiagnostic.proto\x12\x0cszdiagnostic\"8\n CheckDatastorePerformanceRequest\x12\x14\n\x0csecondsToRun\x18\x01 \x01(\x05\"3\n!CheckDatastorePerformanceResponse\x12\x0e\n\x06result\x18\x01 \x01(\t\"\x19\n\x17GetDatastoreInfoRequest\"*\n\x18GetDatastoreInfoResponse\x12\x0e\n\x06result\x18\x01 \x01(\t\"&\n\x11GetFeatureRequest\x12\x11\n\tfeatureId\x18\x01 \x01(\x03\"$\n\x12GetFeatureResponse\x12\x0e\n\x06result\x18\x01 \x01(\t\"\x18\n\x16PurgeRepositoryRequest\"\x19\n\x17PurgeRepositoryResponse\"\'\n\x13ReinitializeRequest\x12\x10\n\x08\x63onfigId\x18\x01 \x01(\x03\"\x16\n\x14ReinitializeResponse2\x81\x04\n\x0cSzDiagnostic\x12~\n\x19\x43heckDatastorePerformance\x12..szdiagnostic.CheckDatastorePerformanceRequest\x1a/.szdiagnostic.CheckDatastorePerformanceResponse\"\x00\x12\x63\n\x10GetDatastoreInfo\x12%.szdiagnostic.GetDatastoreInfoRequest\x1a&.szdiagnostic.GetDatastoreInfoResponse\"\x00\x12Q\n\nGetFeature\x12\x1f.szdiagnostic.GetFeatureRequest\x1a .szdiagnostic.GetFeatureResponse\"\x00\x12`\n\x0fPurgeRepository\x12$.szdiagnostic.PurgeRepositoryRequest\x1a%.szdiagnostic.PurgeRepositoryResponse\"\x00\x12W\n\x0cReinitialize\x12!.szdiagnostic.ReinitializeRequest\x1a\".szdiagnostic.ReinitializeResponse\"\x00\x42s\n\'com.senzing.sz.engine.grpc.SzDiagnosticB\x11SzDiagnosticProtoZ5github.com/senzing-garage/sz-sdk-go-grpc/szdiagnosticb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'szdiagnostic_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\'com.senzing.sz.engine.grpc.SzDiagnosticB\021SzDiagnosticProtoZ5github.com/senzing-garage/sz-sdk-go-grpc/szdiagnostic'
  _globals['_CHECKDATASTOREPERFORMANCEREQUEST']._serialized_start=36
  _globals['_CHECKDATASTOREPERFORMANCEREQUEST']._serialized_end=92
  _globals['_CHECKDATASTOREPERFORMANCERESPONSE']._serialized_start=94
  _globals['_CHECKDATASTOREPERFORMANCERESPONSE']._serialized_end=145
  _globals['_GETDATASTOREINFOREQUEST']._serialized_start=147
  _globals['_GETDATASTOREINFOREQUEST']._serialized_end=172
  _globals['_GETDATASTOREINFORESPONSE']._serialized_start=174
  _globals['_GETDATASTOREINFORESPONSE']._serialized_end=216
  _globals['_GETFEATUREREQUEST']._serialized_start=218
  _globals['_GETFEATUREREQUEST']._serialized_end=256
  _globals['_GETFEATURERESPONSE']._serialized_start=258
  _globals['_GETFEATURERESPONSE']._serialized_end=294
  _globals['_PURGEREPOSITORYREQUEST']._serialized_start=296
  _globals['_PURGEREPOSITORYREQUEST']._serialized_end=320
  _globals['_PURGEREPOSITORYRESPONSE']._serialized_start=322
  _globals['_PURGEREPOSITORYRESPONSE']._serialized_end=347
  _globals['_REINITIALIZEREQUEST']._serialized_start=349
  _globals['_REINITIALIZEREQUEST']._serialized_end=388
  _globals['_REINITIALIZERESPONSE']._serialized_start=390
  _globals['_REINITIALIZERESPONSE']._serialized_end=412
  _globals['_SZDIAGNOSTIC']._serialized_start=415
  _globals['_SZDIAGNOSTIC']._serialized_end=928
# @@protoc_insertion_point(module_scope)