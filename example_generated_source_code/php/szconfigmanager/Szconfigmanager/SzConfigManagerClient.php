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
     * @param \Szconfigmanager\AddConfigRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddConfig(\Szconfigmanager\AddConfigRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfigmanager.SzConfigManager/AddConfig',
        $argument,
        ['\Szconfigmanager\AddConfigResponse', 'decode'],
        $metadata, $options);
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
     * @param \Szconfigmanager\GetConfigListRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetConfigList(\Szconfigmanager\GetConfigListRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfigmanager.SzConfigManager/GetConfigList',
        $argument,
        ['\Szconfigmanager\GetConfigListResponse', 'decode'],
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
