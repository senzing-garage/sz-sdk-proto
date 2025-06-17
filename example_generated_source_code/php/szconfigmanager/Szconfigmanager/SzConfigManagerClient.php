<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Szconfigmanager;

/**
 */
class SzConfigManagerClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * @param \Szconfigmanager\GetConfigRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetConfig(\Szconfigmanager\GetConfigRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfigmanager.SzConfigManager/GetConfig',
        $argument,
        ['\Szconfigmanager\GetConfigResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szconfigmanager\GetConfigRegistryRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetConfigRegistry(\Szconfigmanager\GetConfigRegistryRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfigmanager.SzConfigManager/GetConfigRegistry',
        $argument,
        ['\Szconfigmanager\GetConfigRegistryResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szconfigmanager\GetDefaultConfigIdRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetDefaultConfigId(\Szconfigmanager\GetDefaultConfigIdRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfigmanager.SzConfigManager/GetDefaultConfigId',
        $argument,
        ['\Szconfigmanager\GetDefaultConfigIdResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szconfigmanager\GetTemplateConfigRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetTemplateConfig(\Szconfigmanager\GetTemplateConfigRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfigmanager.SzConfigManager/GetTemplateConfig',
        $argument,
        ['\Szconfigmanager\GetTemplateConfigResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szconfigmanager\RegisterConfigRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function RegisterConfig(\Szconfigmanager\RegisterConfigRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfigmanager.SzConfigManager/RegisterConfig',
        $argument,
        ['\Szconfigmanager\RegisterConfigResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szconfigmanager\ReplaceDefaultConfigIdRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ReplaceDefaultConfigId(\Szconfigmanager\ReplaceDefaultConfigIdRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfigmanager.SzConfigManager/ReplaceDefaultConfigId',
        $argument,
        ['\Szconfigmanager\ReplaceDefaultConfigIdResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szconfigmanager\SetDefaultConfigRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function SetDefaultConfig(\Szconfigmanager\SetDefaultConfigRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfigmanager.SzConfigManager/SetDefaultConfig',
        $argument,
        ['\Szconfigmanager\SetDefaultConfigResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szconfigmanager\SetDefaultConfigIdRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function SetDefaultConfigId(\Szconfigmanager\SetDefaultConfigIdRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfigmanager.SzConfigManager/SetDefaultConfigId',
        $argument,
        ['\Szconfigmanager\SetDefaultConfigIdResponse', 'decode'],
        $metadata, $options);
    }

}
