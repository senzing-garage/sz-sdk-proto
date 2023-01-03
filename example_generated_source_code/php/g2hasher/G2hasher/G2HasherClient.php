<?php
// GENERATED CODE -- DO NOT EDIT!

namespace G2hasher;

/**
 */
class G2HasherClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * @param \G2hasher\DestroyRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Destroy(\G2hasher\DestroyRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2hasher.G2Hasher/Destroy',
        $argument,
        ['\G2hasher\DestroyResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2hasher\ExportTokenLibraryRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ExportTokenLibrary(\G2hasher\ExportTokenLibraryRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2hasher.G2Hasher/ExportTokenLibrary',
        $argument,
        ['\G2hasher\ExportTokenLibraryResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2hasher\InitRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Init(\G2hasher\InitRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2hasher.G2Hasher/Init',
        $argument,
        ['\G2hasher\InitResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2hasher\InitWithConfigRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function InitWithConfig(\G2hasher\InitWithConfigRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2hasher.G2Hasher/InitWithConfig',
        $argument,
        ['\G2hasher\InitWithConfigResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2hasher\ProcessRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Process(\G2hasher\ProcessRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2hasher.G2Hasher/Process',
        $argument,
        ['\G2hasher\ProcessResponse', 'decode'],
        $metadata, $options);
    }

}
