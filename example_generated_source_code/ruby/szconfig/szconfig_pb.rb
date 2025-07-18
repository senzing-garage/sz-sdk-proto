# frozen_string_literal: true
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: szconfig.proto

require 'google/protobuf'


descriptor_data = "\n\x0eszconfig.proto\x12\x08szconfig\"9\n\x1cGetDataSourceRegistryRequest\x12\x19\n\x11\x63onfig_definition\x18\x01 \x01(\t\"/\n\x1dGetDataSourceRegistryResponse\x12\x0e\n\x06result\x18\x01 \x01(\t\"P\n\x19RegisterDataSourceRequest\x12\x19\n\x11\x63onfig_definition\x18\x01 \x01(\t\x12\x18\n\x10\x64\x61ta_source_code\x18\x02 \x01(\t\"G\n\x1aRegisterDataSourceResponse\x12\x0e\n\x06result\x18\x01 \x01(\t\x12\x19\n\x11\x63onfig_definition\x18\x02 \x01(\t\"R\n\x1bUnregisterDataSourceRequest\x12\x19\n\x11\x63onfig_definition\x18\x01 \x01(\t\x12\x18\n\x10\x64\x61ta_source_code\x18\x02 \x01(\t\"I\n\x1cUnregisterDataSourceResponse\x12\x0e\n\x06result\x18\x01 \x01(\t\x12\x19\n\x11\x63onfig_definition\x18\x02 \x01(\t\"0\n\x13VerifyConfigRequest\x12\x19\n\x11\x63onfig_definition\x18\x01 \x01(\t\"&\n\x14VerifyConfigResponse\x12\x0e\n\x06result\x18\x01 \x01(\x08\x32\x93\x03\n\x08SzConfig\x12j\n\x15GetDataSourceRegistry\x12&.szconfig.GetDataSourceRegistryRequest\x1a\'.szconfig.GetDataSourceRegistryResponse\"\x00\x12\x61\n\x12RegisterDataSource\x12#.szconfig.RegisterDataSourceRequest\x1a$.szconfig.RegisterDataSourceResponse\"\x00\x12g\n\x14UnregisterDataSource\x12%.szconfig.UnregisterDataSourceRequest\x1a&.szconfig.UnregisterDataSourceResponse\"\x00\x12O\n\x0cVerifyConfig\x12\x1d.szconfig.VerifyConfigRequest\x1a\x1e.szconfig.VerifyConfigResponse\"\x00\x42X\n\x14\x63om.senzing.sdk.grpcB\rSzConfigProtoZ1github.com/senzing-garage/sz-sdk-go-grpc/szconfigb\x06proto3"

pool = Google::Protobuf::DescriptorPool.generated_pool
pool.add_serialized_file(descriptor_data)

module Szconfig
  GetDataSourceRegistryRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfig.GetDataSourceRegistryRequest").msgclass
  GetDataSourceRegistryResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfig.GetDataSourceRegistryResponse").msgclass
  RegisterDataSourceRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfig.RegisterDataSourceRequest").msgclass
  RegisterDataSourceResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfig.RegisterDataSourceResponse").msgclass
  UnregisterDataSourceRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfig.UnregisterDataSourceRequest").msgclass
  UnregisterDataSourceResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfig.UnregisterDataSourceResponse").msgclass
  VerifyConfigRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfig.VerifyConfigRequest").msgclass
  VerifyConfigResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfig.VerifyConfigResponse").msgclass
end
