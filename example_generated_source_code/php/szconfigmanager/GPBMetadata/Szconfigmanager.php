<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: szconfigmanager.proto

namespace GPBMetadata;

class Szconfigmanager
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
szconfigmanager.protoszconfigmanager"%
GetConfigRequest
	config_id ("#
GetConfigResponse
result (	"
GetConfigsRequest"$
GetConfigsResponse
result (	"
GetDefaultConfigIdRequest",
GetDefaultConfigIdResponse
result ("
GetTemplateConfigRequest"+
GetTemplateConfigResponse
result (	"J
RegisterConfigRequest
config_definition (	
config_comment (	"(
RegisterConfigResponse
result ("a
ReplaceDefaultConfigIdRequest!
current_default_config_id (
new_default_config_id (" 
ReplaceDefaultConfigIdResponse"L
SetDefaultConfigRequest
config_definition (	
config_comment (	"*
SetDefaultConfigResponse
result (".
SetDefaultConfigIdRequest
	config_id ("
SetDefaultConfigIdResponse2�
SzConfigManagerT
	GetConfig!.szconfigmanager.GetConfigRequest".szconfigmanager.GetConfigResponse" W

GetConfigs".szconfigmanager.GetConfigsRequest#.szconfigmanager.GetConfigsResponse" o
GetDefaultConfigId*.szconfigmanager.GetDefaultConfigIdRequest+.szconfigmanager.GetDefaultConfigIdResponse" l
GetTemplateConfig).szconfigmanager.GetTemplateConfigRequest*.szconfigmanager.GetTemplateConfigResponse" c
RegisterConfig&.szconfigmanager.RegisterConfigRequest\'.szconfigmanager.RegisterConfigResponse" {
ReplaceDefaultConfigId..szconfigmanager.ReplaceDefaultConfigIdRequest/.szconfigmanager.ReplaceDefaultConfigIdResponse" i
SetDefaultConfig(.szconfigmanager.SetDefaultConfigRequest).szconfigmanager.SetDefaultConfigResponse" o
SetDefaultConfigId*.szconfigmanager.SetDefaultConfigIdRequest+.szconfigmanager.SetDefaultConfigIdResponse" Bf
com.senzing.sdk.grpcBSzConfigManagerProtoZ8github.com/senzing-garage/sz-sdk-go-grpc/szconfigmanagerbproto3'
        , true);

        static::$is_initialized = true;
    }
}

