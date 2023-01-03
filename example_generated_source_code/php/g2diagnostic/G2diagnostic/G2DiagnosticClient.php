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
     * @param \G2diagnostic\CloseEntityListBySizeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CloseEntityListBySize(\G2diagnostic\CloseEntityListBySizeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/CloseEntityListBySize',
        $argument,
        ['\G2diagnostic\CloseEntityListBySizeResponse', 'decode'],
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
     * @param \G2diagnostic\FetchNextEntityBySizeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FetchNextEntityBySize(\G2diagnostic\FetchNextEntityBySizeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/FetchNextEntityBySize',
        $argument,
        ['\G2diagnostic\FetchNextEntityBySizeResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\FindEntitiesByFeatureIDsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindEntitiesByFeatureIDs(\G2diagnostic\FindEntitiesByFeatureIDsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/FindEntitiesByFeatureIDs',
        $argument,
        ['\G2diagnostic\FindEntitiesByFeatureIDsResponse', 'decode'],
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
     * @param \G2diagnostic\GetDataSourceCountsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetDataSourceCounts(\G2diagnostic\GetDataSourceCountsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetDataSourceCounts',
        $argument,
        ['\G2diagnostic\GetDataSourceCountsResponse', 'decode'],
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
     * @param \G2diagnostic\GetEntityDetailsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetEntityDetails(\G2diagnostic\GetEntityDetailsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetEntityDetails',
        $argument,
        ['\G2diagnostic\GetEntityDetailsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\GetEntityListBySizeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetEntityListBySize(\G2diagnostic\GetEntityListBySizeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetEntityListBySize',
        $argument,
        ['\G2diagnostic\GetEntityListBySizeResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\GetEntityResumeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetEntityResume(\G2diagnostic\GetEntityResumeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetEntityResume',
        $argument,
        ['\G2diagnostic\GetEntityResumeResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\GetEntitySizeBreakdownRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetEntitySizeBreakdown(\G2diagnostic\GetEntitySizeBreakdownRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetEntitySizeBreakdown',
        $argument,
        ['\G2diagnostic\GetEntitySizeBreakdownResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\GetFeatureRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetFeature(\G2diagnostic\GetFeatureRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetFeature',
        $argument,
        ['\G2diagnostic\GetFeatureResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\GetGenericFeaturesRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetGenericFeatures(\G2diagnostic\GetGenericFeaturesRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetGenericFeatures',
        $argument,
        ['\G2diagnostic\GetGenericFeaturesResponse', 'decode'],
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
     * @param \G2diagnostic\GetMappingStatisticsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetMappingStatistics(\G2diagnostic\GetMappingStatisticsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetMappingStatistics',
        $argument,
        ['\G2diagnostic\GetMappingStatisticsResponse', 'decode'],
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
     * @param \G2diagnostic\GetRelationshipDetailsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetRelationshipDetails(\G2diagnostic\GetRelationshipDetailsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetRelationshipDetails',
        $argument,
        ['\G2diagnostic\GetRelationshipDetailsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2diagnostic\GetResolutionStatisticsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetResolutionStatistics(\G2diagnostic\GetResolutionStatisticsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2diagnostic.G2Diagnostic/GetResolutionStatistics',
        $argument,
        ['\G2diagnostic\GetResolutionStatisticsResponse', 'decode'],
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

}
