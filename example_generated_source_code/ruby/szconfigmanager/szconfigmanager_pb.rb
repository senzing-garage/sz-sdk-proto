# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: szconfigmanager.proto

require 'google/protobuf'

Google::Protobuf::DescriptorPool.generated_pool.build do
  add_file("szconfigmanager.proto", :syntax => :proto3) do
    add_message "szconfigmanager.GetConfigRequest" do
      optional :config_id, :int64, 1
    end
    add_message "szconfigmanager.GetConfigResponse" do
      optional :result, :string, 1
    end
    add_message "szconfigmanager.GetConfigsRequest" do
    end
    add_message "szconfigmanager.GetConfigsResponse" do
      optional :result, :string, 1
    end
    add_message "szconfigmanager.GetDefaultConfigIdRequest" do
    end
    add_message "szconfigmanager.GetDefaultConfigIdResponse" do
      optional :result, :int64, 1
    end
    add_message "szconfigmanager.GetTemplateConfigRequest" do
    end
    add_message "szconfigmanager.GetTemplateConfigResponse" do
      optional :result, :string, 1
    end
    add_message "szconfigmanager.RegisterConfigRequest" do
      optional :config_definition, :string, 1
      optional :config_comment, :string, 2
    end
    add_message "szconfigmanager.RegisterConfigResponse" do
      optional :result, :int64, 1
    end
    add_message "szconfigmanager.ReplaceDefaultConfigIdRequest" do
      optional :current_default_config_id, :int64, 1
      optional :new_default_config_id, :int64, 2
    end
    add_message "szconfigmanager.ReplaceDefaultConfigIdResponse" do
    end
    add_message "szconfigmanager.SetDefaultConfigRequest" do
      optional :config_definition, :string, 1
      optional :config_comment, :string, 2
    end
    add_message "szconfigmanager.SetDefaultConfigResponse" do
      optional :result, :int64, 1
    end
    add_message "szconfigmanager.SetDefaultConfigIdRequest" do
      optional :config_id, :int64, 1
    end
    add_message "szconfigmanager.SetDefaultConfigIdResponse" do
    end
  end
end

module Szconfigmanager
  GetConfigRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.GetConfigRequest").msgclass
  GetConfigResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.GetConfigResponse").msgclass
  GetConfigsRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.GetConfigsRequest").msgclass
  GetConfigsResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.GetConfigsResponse").msgclass
  GetDefaultConfigIdRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.GetDefaultConfigIdRequest").msgclass
  GetDefaultConfigIdResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.GetDefaultConfigIdResponse").msgclass
  GetTemplateConfigRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.GetTemplateConfigRequest").msgclass
  GetTemplateConfigResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.GetTemplateConfigResponse").msgclass
  RegisterConfigRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.RegisterConfigRequest").msgclass
  RegisterConfigResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.RegisterConfigResponse").msgclass
  ReplaceDefaultConfigIdRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.ReplaceDefaultConfigIdRequest").msgclass
  ReplaceDefaultConfigIdResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.ReplaceDefaultConfigIdResponse").msgclass
  SetDefaultConfigRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.SetDefaultConfigRequest").msgclass
  SetDefaultConfigResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.SetDefaultConfigResponse").msgclass
  SetDefaultConfigIdRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.SetDefaultConfigIdRequest").msgclass
  SetDefaultConfigIdResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szconfigmanager.SetDefaultConfigIdResponse").msgclass
end
