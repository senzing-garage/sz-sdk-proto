<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
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
            "\x0A\xF9\x0D\x0A\x15szconfigmanager.proto\x12\x0Fszconfigmanager\"%\x0A\x10GetConfigRequest\x12\x11\x0A\x09config_id\x18\x01 \x01(\x03\"#\x0A\x11GetConfigResponse\x12\x0E\x0A\x06result\x18\x01 \x01(\x09\"\x1A\x0A\x18GetConfigRegistryRequest\"+\x0A\x19GetConfigRegistryResponse\x12\x0E\x0A\x06result\x18\x01 \x01(\x09\"\x1B\x0A\x19GetDefaultConfigIdRequest\",\x0A\x1AGetDefaultConfigIdResponse\x12\x0E\x0A\x06result\x18\x01 \x01(\x03\"\x1A\x0A\x18GetTemplateConfigRequest\"+\x0A\x19GetTemplateConfigResponse\x12\x0E\x0A\x06result\x18\x01 \x01(\x09\"J\x0A\x15RegisterConfigRequest\x12\x19\x0A\x11config_definition\x18\x01 \x01(\x09\x12\x16\x0A\x0Econfig_comment\x18\x02 \x01(\x09\"(\x0A\x16RegisterConfigResponse\x12\x0E\x0A\x06result\x18\x01 \x01(\x03\"a\x0A\x1DReplaceDefaultConfigIdRequest\x12!\x0A\x19current_default_config_id\x18\x01 \x01(\x03\x12\x1D\x0A\x15new_default_config_id\x18\x02 \x01(\x03\" \x0A\x1EReplaceDefaultConfigIdResponse\"L\x0A\x17SetDefaultConfigRequest\x12\x19\x0A\x11config_definition\x18\x01 \x01(\x09\x12\x16\x0A\x0Econfig_comment\x18\x02 \x01(\x09\"*\x0A\x18SetDefaultConfigResponse\x12\x0E\x0A\x06result\x18\x01 \x01(\x03\".\x0A\x19SetDefaultConfigIdRequest\x12\x11\x0A\x09config_id\x18\x01 \x01(\x03\"\x1C\x0A\x1ASetDefaultConfigIdResponse2\xF2\x06\x0A\x0FSzConfigManager\x12T\x0A\x09GetConfig\x12!.szconfigmanager.GetConfigRequest\x1A\".szconfigmanager.GetConfigResponse\"\x00\x12l\x0A\x11GetConfigRegistry\x12).szconfigmanager.GetConfigRegistryRequest\x1A*.szconfigmanager.GetConfigRegistryResponse\"\x00\x12o\x0A\x12GetDefaultConfigId\x12*.szconfigmanager.GetDefaultConfigIdRequest\x1A+.szconfigmanager.GetDefaultConfigIdResponse\"\x00\x12l\x0A\x11GetTemplateConfig\x12).szconfigmanager.GetTemplateConfigRequest\x1A*.szconfigmanager.GetTemplateConfigResponse\"\x00\x12c\x0A\x0ERegisterConfig\x12&.szconfigmanager.RegisterConfigRequest\x1A'.szconfigmanager.RegisterConfigResponse\"\x00\x12{\x0A\x16ReplaceDefaultConfigId\x12..szconfigmanager.ReplaceDefaultConfigIdRequest\x1A/.szconfigmanager.ReplaceDefaultConfigIdResponse\"\x00\x12i\x0A\x10SetDefaultConfig\x12(.szconfigmanager.SetDefaultConfigRequest\x1A).szconfigmanager.SetDefaultConfigResponse\"\x00\x12o\x0A\x12SetDefaultConfigId\x12*.szconfigmanager.SetDefaultConfigIdRequest\x1A+.szconfigmanager.SetDefaultConfigIdResponse\"\x00Bf\x0A\x14com.senzing.sdk.grpcB\x14SzConfigManagerProtoZ8github.com/senzing-garage/sz-sdk-go-grpc/szconfigmanagerb\x06proto3"
        , true);

        static::$is_initialized = true;
    }
}

