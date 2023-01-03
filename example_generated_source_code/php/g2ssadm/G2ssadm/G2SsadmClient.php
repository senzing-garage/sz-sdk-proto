<?php
// GENERATED CODE -- DO NOT EDIT!

namespace G2ssadm;

/**
 */
class G2SsadmClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * @param \G2ssadm\CreateSaltInStoreRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CreateSaltInStore(\G2ssadm\CreateSaltInStoreRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2ssadm.G2Ssadm/CreateSaltInStore',
        $argument,
        ['\G2ssadm\CreateSaltInStoreResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2ssadm\DestroyRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Destroy(\G2ssadm\DestroyRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2ssadm.G2Ssadm/Destroy',
        $argument,
        ['\G2ssadm\DestroyResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2ssadm\InitRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Init(\G2ssadm\InitRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2ssadm.G2Ssadm/Init',
        $argument,
        ['\G2ssadm\InitResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2ssadm\InitializeNewTokenRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function InitializeNewToken(\G2ssadm\InitializeNewTokenRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2ssadm.G2Ssadm/InitializeNewToken',
        $argument,
        ['\G2ssadm\InitializeNewTokenResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2ssadm\ListRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function List(\G2ssadm\ListRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2ssadm.G2Ssadm/List',
        $argument,
        ['\G2ssadm\ListResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2ssadm\PutRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Put(\G2ssadm\PutRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2ssadm.G2Ssadm/Put',
        $argument,
        ['\G2ssadm\PutResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2ssadm\ReinitializeTokenRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ReinitializeToken(\G2ssadm\ReinitializeTokenRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2ssadm.G2Ssadm/ReinitializeToken',
        $argument,
        ['\G2ssadm\ReinitializeTokenResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2ssadm\SetupStoreRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function SetupStore(\G2ssadm\SetupStoreRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2ssadm.G2Ssadm/SetupStore',
        $argument,
        ['\G2ssadm\SetupStoreResponse', 'decode'],
        $metadata, $options);
    }

}
