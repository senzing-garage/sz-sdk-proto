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
     * @param \Szconfig\AddDataSourceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddDataSource(\Szconfig\AddDataSourceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfig.SzConfig/AddDataSource',
        $argument,
        ['\Szconfig\AddDataSourceResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szconfig\DeleteDataSourceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteDataSource(\Szconfig\DeleteDataSourceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfig.SzConfig/DeleteDataSource',
        $argument,
        ['\Szconfig\DeleteDataSourceResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szconfig\GetDataSourcesRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetDataSources(\Szconfig\GetDataSourcesRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfig.SzConfig/GetDataSources',
        $argument,
        ['\Szconfig\GetDataSourcesResponse', 'decode'],
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
