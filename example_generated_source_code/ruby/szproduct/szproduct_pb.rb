# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: szproduct.proto

require 'google/protobuf'

Google::Protobuf::DescriptorPool.generated_pool.build do
  add_file("szproduct.proto", :syntax => :proto3) do
    add_message "szproduct.GetLicenseRequest" do
    end
    add_message "szproduct.GetLicenseResponse" do
      optional :result, :string, 1
    end
    add_message "szproduct.GetVersionRequest" do
    end
    add_message "szproduct.GetVersionResponse" do
      optional :result, :string, 1
    end
  end
end

module Szproduct
  GetLicenseRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szproduct.GetLicenseRequest").msgclass
  GetLicenseResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szproduct.GetLicenseResponse").msgclass
  GetVersionRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szproduct.GetVersionRequest").msgclass
  GetVersionResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("szproduct.GetVersionResponse").msgclass
end
