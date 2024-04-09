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
     * @param \Szconfig\CloseConfigRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CloseConfig(\Szconfig\CloseConfigRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfig.SzConfig/CloseConfig',
        $argument,
        ['\Szconfig\CloseConfigResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szconfig\CreateConfigRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CreateConfig(\Szconfig\CreateConfigRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfig.SzConfig/CreateConfig',
        $argument,
        ['\Szconfig\CreateConfigResponse', 'decode'],
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
     * @param \Szconfig\ImportConfigRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ImportConfig(\Szconfig\ImportConfigRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfig.SzConfig/ImportConfig',
        $argument,
        ['\Szconfig\ImportConfigResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szconfig\ExportConfigRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ExportConfig(\Szconfig\ExportConfigRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szconfig.SzConfig/ExportConfig',
        $argument,
        ['\Szconfig\ExportConfigResponse', 'decode'],
        $metadata, $options);
    }

}
