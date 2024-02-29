<?php
// GENERATED CODE -- DO NOT EDIT!

namespace G2diagnostic;

/**
 */
class G2DiagnosticClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * @param \G2diagnostic\CheckDBPerfRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CheckDBPerf(\G2diagnostic\CheckDBPerfRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/CheckDBPerf',
        $argument,
        ['\G2diagnostic\CheckDBPerfResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\DestroyRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Destroy(\G2diagnostic\DestroyRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/Destroy',
        $argument,
        ['\G2diagnostic\DestroyResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\InitRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Init(\G2diagnostic\InitRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/Init',
        $argument,
        ['\G2diagnostic\InitResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\InitWithConfigIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function InitWithConfigID(\G2diagnostic\InitWithConfigIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/InitWithConfigID',
        $argument,
        ['\G2diagnostic\InitWithConfigIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\PurgeRepositoryRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function PurgeRepository(\G2diagnostic\PurgeRepositoryRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/PurgeRepository',
        $argument,
        ['\G2diagnostic\PurgeRepositoryResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\ReinitRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Reinit(\G2diagnostic\ReinitRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/Reinit',
        $argument,
        ['\G2diagnostic\ReinitResponse', 'decode'],
        $metadata, $options);
    }

}
