<?php
// GENERATED CODE -- DO NOT EDIT!

namespace G2product;

/**
 */
class G2ProductClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * @param \G2product\DestroyRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Destroy(\G2product\DestroyRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2product.G2Product/Destroy',
        $argument,
        ['\G2product\DestroyResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2product\InitRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Init(\G2product\InitRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2product.G2Product/Init',
        $argument,
        ['\G2product\InitResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2product\LicenseRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function License(\G2product\LicenseRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2product.G2Product/License',
        $argument,
        ['\G2product\LicenseResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2product\VersionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Version(\G2product\VersionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2product.G2Product/Version',
        $argument,
        ['\G2product\VersionResponse', 'decode'],
        $metadata, $options);
    }

}
