<?php
// GENERATED CODE -- DO NOT EDIT!

namespace G2configmgr;

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
     * @param \G2configmgr\AddConfigRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddConfig(\G2configmgr\AddConfigRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2configmgr.G2Diagnostic/AddConfig',
        $argument,
        ['\G2configmgr\AddConfigResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2configmgr\ClearLastExceptionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ClearLastException(\G2configmgr\ClearLastExceptionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2configmgr.G2Diagnostic/ClearLastException',
        $argument,
        ['\G2configmgr\ClearLastExceptionResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2configmgr\DestroyRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Destroy(\G2configmgr\DestroyRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2configmgr.G2Diagnostic/Destroy',
        $argument,
        ['\G2configmgr\DestroyResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2configmgr\GetConfigRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetConfig(\G2configmgr\GetConfigRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2configmgr.G2Diagnostic/GetConfig',
        $argument,
        ['\G2configmgr\GetConfigResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2configmgr\GetConfigListRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetConfigList(\G2configmgr\GetConfigListRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2configmgr.G2Diagnostic/GetConfigList',
        $argument,
        ['\G2configmgr\GetConfigListResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2configmgr\GetDefaultConfigIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetDefaultConfigID(\G2configmgr\GetDefaultConfigIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2configmgr.G2Diagnostic/GetDefaultConfigID',
        $argument,
        ['\G2configmgr\GetDefaultConfigIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2configmgr\GetLastExceptionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetLastException(\G2configmgr\GetLastExceptionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2configmgr.G2Diagnostic/GetLastException',
        $argument,
        ['\G2configmgr\GetLastExceptionResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2configmgr\GetLastExceptionCodeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetLastExceptionCode(\G2configmgr\GetLastExceptionCodeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2configmgr.G2Diagnostic/GetLastExceptionCode',
        $argument,
        ['\G2configmgr\GetLastExceptionCodeResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2configmgr\InitRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Init(\G2configmgr\InitRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2configmgr.G2Diagnostic/Init',
        $argument,
        ['\G2configmgr\InitResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2configmgr\ReplaceDefaultConfigIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ReplaceDefaultConfigID(\G2configmgr\ReplaceDefaultConfigIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2configmgr.G2Diagnostic/ReplaceDefaultConfigID',
        $argument,
        ['\G2configmgr\ReplaceDefaultConfigIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2configmgr\SetDefaultConfigIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function SetDefaultConfigID(\G2configmgr\SetDefaultConfigIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2configmgr.G2Diagnostic/SetDefaultConfigID',
        $argument,
        ['\G2configmgr\SetDefaultConfigIDResponse', 'decode'],
        $metadata, $options);
    }

}
