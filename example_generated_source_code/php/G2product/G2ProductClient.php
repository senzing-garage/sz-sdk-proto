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
     * @param \G2product\ClearLastExceptionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ClearLastException(\G2product\ClearLastExceptionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2product.G2Product/ClearLastException',
        $argument,
        ['\G2product\ClearLastExceptionResponse', 'decode'],
        $metadata, $options);
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
     * @param \G2product\GetLastExceptionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetLastException(\G2product\GetLastExceptionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2product.G2Product/GetLastException',
        $argument,
        ['\G2product\GetLastExceptionResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2product\GetLastExceptionCodeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetLastExceptionCode(\G2product\GetLastExceptionCodeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2product.G2Product/GetLastExceptionCode',
        $argument,
        ['\G2product\GetLastExceptionCodeResponse', 'decode'],
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
     * @param \G2product\ValidateLicenseFileRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ValidateLicenseFile(\G2product\ValidateLicenseFileRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2product.G2Product/ValidateLicenseFile',
        $argument,
        ['\G2product\ValidateLicenseFileResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2product\ValidateLicenseStringBase64Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ValidateLicenseStringBase64(\G2product\ValidateLicenseStringBase64Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2product.G2Product/ValidateLicenseStringBase64',
        $argument,
        ['\G2product\ValidateLicenseStringBase64Response', 'decode'],
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
