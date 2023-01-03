<?php
// GENERATED CODE -- DO NOT EDIT!

namespace G2configmgr;

/**
 */
class G2ConfigMgrClient extends \Grpc\BaseStub {

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
        return $this->_simpleRequest('/g2configmgr.G2ConfigMgr/AddConfig',
        $argument,
        ['\G2configmgr\AddConfigResponse', 'decode'],
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
        return $this->_simpleRequest('/g2configmgr.G2ConfigMgr/Destroy',
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
        return $this->_simpleRequest('/g2configmgr.G2ConfigMgr/GetConfig',
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
        return $this->_simpleRequest('/g2configmgr.G2ConfigMgr/GetConfigList',
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
        return $this->_simpleRequest('/g2configmgr.G2ConfigMgr/GetDefaultConfigID',
        $argument,
        ['\G2configmgr\GetDefaultConfigIDResponse', 'decode'],
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
        return $this->_simpleRequest('/g2configmgr.G2ConfigMgr/Init',
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
        return $this->_simpleRequest('/g2configmgr.G2ConfigMgr/ReplaceDefaultConfigID',
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
        return $this->_simpleRequest('/g2configmgr.G2ConfigMgr/SetDefaultConfigID',
        $argument,
        ['\G2configmgr\SetDefaultConfigIDResponse', 'decode'],
        $metadata, $options);
    }

}
