<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: szconfig.proto

namespace GPBMetadata;

class Szconfig
{
    public static $is_initialized = false;

    public static function initOnce() {
        $pool = \Google\Protobuf\Internal\DescriptorPool::getGeneratedPool();

        if (static::$is_initialized == true) {
          return;
        }
        $pool->internalAddGeneratedFile(
            '
�

szconfig.protoszconfig"G
AddDataSourceRequest
config_handle (
data_source_code (	"\'
AddDataSourceResponse
result (	"+
CloseConfigRequest
config_handle ("
CloseConfigResponse"
CreateConfigRequest"&
CreateConfigResponse
result ("J
DeleteDataSourceRequest
config_handle (
data_source_code (	"
DeleteDataSourceResponse",
ExportConfigRequest
config_handle ("&
ExportConfigResponse
result (	".
GetDataSourcesRequest
config_handle ("(
GetDataSourcesResponse
result (	"0
ImportConfigRequest
config_definition (	"&
ImportConfigResponse
result (2�
SzConfigR
AddDataSource.szconfig.AddDataSourceRequest.szconfig.AddDataSourceResponse" L
CloseConfig.szconfig.CloseConfigRequest.szconfig.CloseConfigResponse" O
CreateConfig.szconfig.CreateConfigRequest.szconfig.CreateConfigResponse" [
DeleteDataSource!.szconfig.DeleteDataSourceRequest".szconfig.DeleteDataSourceResponse" O
ExportConfig.szconfig.ExportConfigRequest.szconfig.ExportConfigResponse" U
GetDataSources.szconfig.GetDataSourcesRequest .szconfig.GetDataSourcesResponse" O
ImportConfig.szconfig.ImportConfigRequest.szconfig.ImportConfigResponse" BX
com.senzing.sdk.grpcBSzConfigProtoZ1github.com/senzing-garage/sz-sdk-go-grpc/szconfigbproto3'
        , true);

        static::$is_initialized = true;
    }
}

