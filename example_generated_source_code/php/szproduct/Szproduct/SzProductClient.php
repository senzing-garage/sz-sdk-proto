<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Szproduct;

/**
 */
class SzProductClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * @param \Szproduct\GetLicenseRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetLicense(\Szproduct\GetLicenseRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szproduct.SzProduct/GetLicense',
        $argument,
        ['\Szproduct\GetLicenseResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szproduct\GetVersionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetVersion(\Szproduct\GetVersionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szproduct.SzProduct/GetVersion',
        $argument,
        ['\Szproduct\GetVersionResponse', 'decode'],
        $metadata, $options);
    }

}
