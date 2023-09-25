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
     * @param \G2diagnostic\GetAvailableMemoryRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetAvailableMemory(\G2diagnostic\GetAvailableMemoryRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetAvailableMemory',
        $argument,
        ['\G2diagnostic\GetAvailableMemoryResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\GetDBInfoRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetDBInfo(\G2diagnostic\GetDBInfoRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetDBInfo',
        $argument,
        ['\G2diagnostic\GetDBInfoResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\GetLogicalCoresRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetLogicalCores(\G2diagnostic\GetLogicalCoresRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetLogicalCores',
        $argument,
        ['\G2diagnostic\GetLogicalCoresResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\GetPhysicalCoresRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetPhysicalCores(\G2diagnostic\GetPhysicalCoresRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetPhysicalCores',
        $argument,
        ['\G2diagnostic\GetPhysicalCoresResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\GetTotalSystemMemoryRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetTotalSystemMemory(\G2diagnostic\GetTotalSystemMemoryRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetTotalSystemMemory',
        $argument,
        ['\G2diagnostic\GetTotalSystemMemoryResponse', 'decode'],
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

    /**
     * @param \G2diagnostic\StreamEntityListBySizeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ServerStreamingCall
     */
    public function StreamEntityListBySize(\G2diagnostic\StreamEntityListBySizeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_serverStreamRequest('/g2diagnostic.G2Diagnostic/StreamEntityListBySize',
        $argument,
        ['\G2diagnostic\StreamEntityListBySizeResponse', 'decode'],
        $metadata, $options);
    }

}
