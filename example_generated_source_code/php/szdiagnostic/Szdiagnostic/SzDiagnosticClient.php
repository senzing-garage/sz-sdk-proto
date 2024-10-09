<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Szdiagnostic;

/**
 */
class SzDiagnosticClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * @param \Szdiagnostic\CheckDatastorePerformanceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CheckDatastorePerformance(\Szdiagnostic\CheckDatastorePerformanceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szdiagnostic.SzDiagnostic/CheckDatastorePerformance',
        $argument,
        ['\Szdiagnostic\CheckDatastorePerformanceResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szdiagnostic\GetDatastoreInfoRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetDatastoreInfo(\Szdiagnostic\GetDatastoreInfoRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szdiagnostic.SzDiagnostic/GetDatastoreInfo',
        $argument,
        ['\Szdiagnostic\GetDatastoreInfoResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szdiagnostic\GetFeatureRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetFeature(\Szdiagnostic\GetFeatureRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szdiagnostic.SzDiagnostic/GetFeature',
        $argument,
        ['\Szdiagnostic\GetFeatureResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szdiagnostic\PurgeRepositoryRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function PurgeRepository(\Szdiagnostic\PurgeRepositoryRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szdiagnostic.SzDiagnostic/PurgeRepository',
        $argument,
        ['\Szdiagnostic\PurgeRepositoryResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szdiagnostic\ReinitializeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Reinitialize(\Szdiagnostic\ReinitializeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szdiagnostic.SzDiagnostic/Reinitialize',
        $argument,
        ['\Szdiagnostic\ReinitializeResponse', 'decode'],
        $metadata, $options);
    }

}
