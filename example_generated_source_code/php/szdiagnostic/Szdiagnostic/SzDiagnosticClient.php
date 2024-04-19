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
     * @param \Szdiagnostic\CheckDatabasePerformanceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CheckDatabasePerformance(\Szdiagnostic\CheckDatabasePerformanceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szdiagnostic.SzDiagnostic/CheckDatabasePerformance',
        $argument,
        ['\Szdiagnostic\CheckDatabasePerformanceResponse', 'decode'],
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

}
