<?php
// GENERATED CODE -- DO NOT EDIT!

namespace G2config;

/**
 */
class G2ConfigClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * @param \G2config\AddDataSourceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddDataSource(\G2config\AddDataSourceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2config.G2Config/AddDataSource',
        $argument,
        ['\G2config\AddDataSourceResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2config\ClearLastExceptionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ClearLastException(\G2config\ClearLastExceptionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2config.G2Config/ClearLastException',
        $argument,
        ['\G2config\ClearLastExceptionResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2config\CloseRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Close(\G2config\CloseRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2config.G2Config/Close',
        $argument,
        ['\G2config\CloseResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2config\CreateRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Create(\G2config\CreateRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2config.G2Config/Create',
        $argument,
        ['\G2config\CreateResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2config\DeleteDataSourceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteDataSource(\G2config\DeleteDataSourceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2config.G2Config/DeleteDataSource',
        $argument,
        ['\G2config\DeleteDataSourceResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2config\DestroyRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Destroy(\G2config\DestroyRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2config.G2Config/Destroy',
        $argument,
        ['\G2config\DestroyResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2config\GetLastExceptionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetLastException(\G2config\GetLastExceptionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2config.G2Config/GetLastException',
        $argument,
        ['\G2config\GetLastExceptionResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2config\GetLastExceptionCodeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetLastExceptionCode(\G2config\GetLastExceptionCodeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2config.G2Config/GetLastExceptionCode',
        $argument,
        ['\G2config\GetLastExceptionCodeResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2config\InitRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Init(\G2config\InitRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2config.G2Config/Init',
        $argument,
        ['\G2config\InitResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2config\ListDataSourcesRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ListDataSources(\G2config\ListDataSourcesRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2config.G2Config/ListDataSources',
        $argument,
        ['\G2config\ListDataSourcesResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2config\LoadRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Load(\G2config\LoadRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2config.G2Config/Load',
        $argument,
        ['\G2config\LoadResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2config\SaveRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Save(\G2config\SaveRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2config.G2Config/Save',
        $argument,
        ['\G2config\SaveResponse', 'decode'],
        $metadata, $options);
    }

}
