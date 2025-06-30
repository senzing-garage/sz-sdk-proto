<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Szconfig;

/**
 */
class SzConfigClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * @param \Szconfig\RegisterDataSourceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function RegisterDataSource(\Szconfig\RegisterDataSourceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfig.SzConfig/RegisterDataSource',
        $argument,
        ['\Szconfig\RegisterDataSourceResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szconfig\UnregisterDataSourceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UnregisterDataSource(\Szconfig\UnregisterDataSourceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfig.SzConfig/UnregisterDataSource',
        $argument,
        ['\Szconfig\UnregisterDataSourceResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szconfig\GetDataSourceRegistryRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetDataSourceRegistry(\Szconfig\GetDataSourceRegistryRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfig.SzConfig/GetDataSourceRegistry',
        $argument,
        ['\Szconfig\GetDataSourceRegistryResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szconfig\VerifyConfigRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function VerifyConfig(\Szconfig\VerifyConfigRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfig.SzConfig/VerifyConfig',
        $argument,
        ['\Szconfig\VerifyConfigResponse', 'decode'],
        $metadata, $options);
    }

}
