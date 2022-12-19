<?php
// GENERATED CODE -- DO NOT EDIT!

namespace G2engine;

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
     * @param \G2engine\AddRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddRecord(\G2engine\AddRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/AddRecord',
        $argument,
        ['\G2engine\AddRecordResponseResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\AddRecordWithInfoRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddRecordWithInfo(\G2engine\AddRecordWithInfoRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/AddRecordWithInfo',
        $argument,
        ['\G2engine\AddRecordWithInfoResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\AddRecordWithInfoWithReturnedRecordIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddRecordWithInfoWithReturnedRecordID(\G2engine\AddRecordWithInfoWithReturnedRecordIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/AddRecordWithInfoWithReturnedRecordID',
        $argument,
        ['\G2engine\AddRecordWithInfoWithReturnedRecordIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\AddRecordWithReturnedRecordIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddRecordWithReturnedRecordID(\G2engine\AddRecordWithReturnedRecordIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/AddRecordWithReturnedRecordID',
        $argument,
        ['\G2engine\AddRecordWithReturnedRecordIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\CheckRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CheckRecord(\G2engine\CheckRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/CheckRecord',
        $argument,
        ['\G2engine\CheckRecordResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ClearLastExceptionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ClearLastException(\G2engine\ClearLastExceptionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ClearLastException',
        $argument,
        ['\G2engine\ClearLastExceptionResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\CloseExportRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CloseExport(\G2engine\CloseExportRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/CloseExport',
        $argument,
        ['\G2engine\CloseExportResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\CountRedoRecordsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CountRedoRecords(\G2engine\CountRedoRecordsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/CountRedoRecords',
        $argument,
        ['\G2engine\CountRedoRecordsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\DeleteRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteRecord(\G2engine\DeleteRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/DeleteRecord',
        $argument,
        ['\G2engine\DeleteRecordResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\DeleteRecordWithInfoRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteRecordWithInfo(\G2engine\DeleteRecordWithInfoRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/DeleteRecordWithInfo',
        $argument,
        ['\G2engine\DeleteRecordWithInfoResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\DestroyRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Destroy(\G2engine\DestroyRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/Destroy',
        $argument,
        ['\G2engine\DestroyResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ExportConfigRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ExportConfig(\G2engine\ExportConfigRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ExportConfig',
        $argument,
        ['\G2engine\ExportConfigResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ExportConfigAndConfigIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ExportConfigAndConfigID(\G2engine\ExportConfigAndConfigIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ExportConfigAndConfigID',
        $argument,
        ['\G2engine\ExportConfigAndConfigIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ExportCSVEntityReportRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ExportCSVEntityReport(\G2engine\ExportCSVEntityReportRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ExportCSVEntityReport',
        $argument,
        ['\G2engine\ExportCSVEntityReportResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ExportJSONEntityReportRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ExportJSONEntityReport(\G2engine\ExportJSONEntityReportRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ExportJSONEntityReport',
        $argument,
        ['\G2engine\ExportJSONEntityReportResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FetchNextRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FetchNext(\G2engine\FetchNextRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FetchNext',
        $argument,
        ['\G2engine\FetchNextResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindInterestingEntitiesByEntityIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindInterestingEntitiesByEntityID(\G2engine\FindInterestingEntitiesByEntityIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindInterestingEntitiesByEntityID',
        $argument,
        ['\G2engine\FindInterestingEntitiesByEntityIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindInterestingEntitiesByRecordIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindInterestingEntitiesByRecordID(\G2engine\FindInterestingEntitiesByRecordIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindInterestingEntitiesByRecordID',
        $argument,
        ['\G2engine\FindInterestingEntitiesByRecordIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindNetworkByEntityIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindNetworkByEntityID(\G2engine\FindNetworkByEntityIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindNetworkByEntityID',
        $argument,
        ['\G2engine\FindNetworkByEntityIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindNetworkByEntityID_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindNetworkByEntityID_V2(\G2engine\FindNetworkByEntityID_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindNetworkByEntityID_V2',
        $argument,
        ['\G2engine\FindNetworkByEntityID_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindNetworkByRecordIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindNetworkByRecordID(\G2engine\FindNetworkByRecordIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindNetworkByRecordID',
        $argument,
        ['\G2engine\FindNetworkByRecordIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindNetworkByRecordID_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindNetworkByRecordID_V2(\G2engine\FindNetworkByRecordID_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindNetworkByRecordID_V2',
        $argument,
        ['\G2engine\FindNetworkByRecordID_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindPathByEntityIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindPathByEntityID(\G2engine\FindPathByEntityIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindPathByEntityID',
        $argument,
        ['\G2engine\FindPathByEntityIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindPathByEntityID_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindPathByEntityID_V2(\G2engine\FindPathByEntityID_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindPathByEntityID_V2',
        $argument,
        ['\G2engine\FindPathByEntityID_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindPathByRecordIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindPathByRecordID(\G2engine\FindPathByRecordIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindPathByRecordID',
        $argument,
        ['\G2engine\FindPathByRecordIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindPathByRecordID_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindPathByRecordID_V2(\G2engine\FindPathByRecordID_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindPathByRecordID_V2',
        $argument,
        ['\G2engine\FindPathByRecordID_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindPathExcludingByEntityIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindPathExcludingByEntityID(\G2engine\FindPathExcludingByEntityIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindPathExcludingByEntityID',
        $argument,
        ['\G2engine\FindPathExcludingByEntityIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindPathExcludingByEntityID_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindPathExcludingByEntityID_V2(\G2engine\FindPathExcludingByEntityID_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindPathExcludingByEntityID_V2',
        $argument,
        ['\G2engine\FindPathExcludingByEntityID_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindPathExcludingByRecordIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindPathExcludingByRecordID(\G2engine\FindPathExcludingByRecordIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindPathExcludingByRecordID',
        $argument,
        ['\G2engine\FindPathExcludingByRecordIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindPathExcludingByRecordID_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindPathExcludingByRecordID_V2(\G2engine\FindPathExcludingByRecordID_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindPathExcludingByRecordID_V2',
        $argument,
        ['\G2engine\FindPathExcludingByRecordID_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindPathIncludingSourceByEntityIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindPathIncludingSourceByEntityID(\G2engine\FindPathIncludingSourceByEntityIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindPathIncludingSourceByEntityID',
        $argument,
        ['\G2engine\FindPathIncludingSourceByEntityIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindPathIncludingSourceByEntityID_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindPathIncludingSourceByEntityID_V2(\G2engine\FindPathIncludingSourceByEntityID_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindPathIncludingSourceByEntityID_V2',
        $argument,
        ['\G2engine\FindPathIncludingSourceByEntityID_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindPathIncludingSourceByRecordIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindPathIncludingSourceByRecordID(\G2engine\FindPathIncludingSourceByRecordIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindPathIncludingSourceByRecordID',
        $argument,
        ['\G2engine\FindPathIncludingSourceByRecordIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\FindPathIncludingSourceByRecordID_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindPathIncludingSourceByRecordID_V2(\G2engine\FindPathIncludingSourceByRecordID_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/FindPathIncludingSourceByRecordID_V2',
        $argument,
        ['\G2engine\FindPathIncludingSourceByRecordID_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\GetActiveConfigIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetActiveConfigID(\G2engine\GetActiveConfigIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/GetActiveConfigID',
        $argument,
        ['\G2engine\GetActiveConfigIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\GetEntityByEntityIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetEntityByEntityID(\G2engine\GetEntityByEntityIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/GetEntityByEntityID',
        $argument,
        ['\G2engine\GetEntityByEntityIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\GetEntityByEntityID_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetEntityByEntityID_V2(\G2engine\GetEntityByEntityID_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/GetEntityByEntityID_V2',
        $argument,
        ['\G2engine\GetEntityByEntityID_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\GetEntityByRecordIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetEntityByRecordID(\G2engine\GetEntityByRecordIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/GetEntityByRecordID',
        $argument,
        ['\G2engine\GetEntityByRecordIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\GetEntityByRecordID_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetEntityByRecordID_V2(\G2engine\GetEntityByRecordID_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/GetEntityByRecordID_V2',
        $argument,
        ['\G2engine\GetEntityByRecordID_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\GetLastExceptionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetLastException(\G2engine\GetLastExceptionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/GetLastException',
        $argument,
        ['\G2engine\GetLastExceptionResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\GetLastExceptionCodeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetLastExceptionCode(\G2engine\GetLastExceptionCodeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/GetLastExceptionCode',
        $argument,
        ['\G2engine\GetLastExceptionCodeResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\GetRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetRecord(\G2engine\GetRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/GetRecord',
        $argument,
        ['\G2engine\GetRecordResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\GetRecord_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetRecord_V2(\G2engine\GetRecord_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/GetRecord_V2',
        $argument,
        ['\G2engine\GetRecord_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\GetRedoRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetRedoRecord(\G2engine\GetRedoRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/GetRedoRecord',
        $argument,
        ['\G2engine\GetRedoRecordResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\GetRepositoryLastModifiedTimeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetRepositoryLastModifiedTime(\G2engine\GetRepositoryLastModifiedTimeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/GetRepositoryLastModifiedTime',
        $argument,
        ['\G2engine\GetRepositoryLastModifiedTimeResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\GetVirtualEntityByRecordIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetVirtualEntityByRecordID(\G2engine\GetVirtualEntityByRecordIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/GetVirtualEntityByRecordID',
        $argument,
        ['\G2engine\GetVirtualEntityByRecordIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\GetVirtualEntityByRecordID_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetVirtualEntityByRecordID_V2(\G2engine\GetVirtualEntityByRecordID_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/GetVirtualEntityByRecordID_V2',
        $argument,
        ['\G2engine\GetVirtualEntityByRecordID_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\HowEntityByEntityIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function HowEntityByEntityID(\G2engine\HowEntityByEntityIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/HowEntityByEntityID',
        $argument,
        ['\G2engine\HowEntityByEntityIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\HowEntityByEntityID_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function HowEntityByEntityID_V2(\G2engine\HowEntityByEntityID_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/HowEntityByEntityID_V2',
        $argument,
        ['\G2engine\HowEntityByEntityID_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\InitRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Init(\G2engine\InitRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/Init',
        $argument,
        ['\G2engine\InitResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\InitWithConfigIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function InitWithConfigID(\G2engine\InitWithConfigIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/InitWithConfigID',
        $argument,
        ['\G2engine\InitWithConfigIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\PrimeEngineRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function PrimeEngine(\G2engine\PrimeEngineRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/PrimeEngine',
        $argument,
        ['\G2engine\PrimeEngineResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ProcessRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Process(\G2engine\ProcessRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/Process',
        $argument,
        ['\G2engine\ProcessResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ProcessRedoRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ProcessRedoRecord(\G2engine\ProcessRedoRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ProcessRedoRecord',
        $argument,
        ['\G2engine\ProcessRedoRecordResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ProcessRedoRecordWithInfoRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ProcessRedoRecordWithInfo(\G2engine\ProcessRedoRecordWithInfoRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ProcessRedoRecordWithInfo',
        $argument,
        ['\G2engine\ProcessRedoRecordWithInfoResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ProcessWithInfoRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ProcessWithInfo(\G2engine\ProcessWithInfoRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ProcessWithInfo',
        $argument,
        ['\G2engine\ProcessWithInfoResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ProcessWithResponseRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ProcessWithResponse(\G2engine\ProcessWithResponseRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ProcessWithResponse',
        $argument,
        ['\G2engine\ProcessWithResponseResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ProcessWithResponseResizeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ProcessWithResponseResize(\G2engine\ProcessWithResponseResizeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ProcessWithResponseResize',
        $argument,
        ['\G2engine\ProcessWithResponseResizeResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\PurgeRepositoryRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function PurgeRepository(\G2engine\PurgeRepositoryRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/PurgeRepository',
        $argument,
        ['\G2engine\PurgeRepositoryResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ReevaluateEntityRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ReevaluateEntity(\G2engine\ReevaluateEntityRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ReevaluateEntity',
        $argument,
        ['\G2engine\ReevaluateEntityResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ReevaluateEntityWithInfoRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ReevaluateEntityWithInfo(\G2engine\ReevaluateEntityWithInfoRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ReevaluateEntityWithInfo',
        $argument,
        ['\G2engine\ReevaluateEntityWithInfoResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ReevaluateRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ReevaluateRecord(\G2engine\ReevaluateRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ReevaluateRecord',
        $argument,
        ['\G2engine\ReevaluateRecordResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ReevaluateRecordWithInfoRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ReevaluateRecordWithInfo(\G2engine\ReevaluateRecordWithInfoRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ReevaluateRecordWithInfo',
        $argument,
        ['\G2engine\ReevaluateRecordWithInfoResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ReinitRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Reinit(\G2engine\ReinitRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/Reinit',
        $argument,
        ['\G2engine\ReinitResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ReplaceRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ReplaceRecord(\G2engine\ReplaceRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ReplaceRecord',
        $argument,
        ['\G2engine\ReplaceRecordResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\ReplaceRecordWithInfoRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ReplaceRecordWithInfo(\G2engine\ReplaceRecordWithInfoRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/ReplaceRecordWithInfo',
        $argument,
        ['\G2engine\ReplaceRecordWithInfoResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\SearchByAttributesRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function SearchByAttributes(\G2engine\SearchByAttributesRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/SearchByAttributes',
        $argument,
        ['\G2engine\SearchByAttributesResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\SearchByAttributes_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function SearchByAttributes_V2(\G2engine\SearchByAttributes_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/SearchByAttributes_V2',
        $argument,
        ['\G2engine\SearchByAttributes_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\StatsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Stats(\G2engine\StatsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/Stats',
        $argument,
        ['\G2engine\StatsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\WhyEntitiesRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function WhyEntities(\G2engine\WhyEntitiesRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/WhyEntities',
        $argument,
        ['\G2engine\WhyEntitiesResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\WhyEntities_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function WhyEntities_V2(\G2engine\WhyEntities_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/WhyEntities_V2',
        $argument,
        ['\G2engine\WhyEntities_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\WhyEntityByEntityIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function WhyEntityByEntityID(\G2engine\WhyEntityByEntityIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/WhyEntityByEntityID',
        $argument,
        ['\G2engine\WhyEntityByEntityIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\WhyEntityByEntityID_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function WhyEntityByEntityID_V2(\G2engine\WhyEntityByEntityID_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/WhyEntityByEntityID_V2',
        $argument,
        ['\G2engine\WhyEntityByEntityID_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\WhyEntityByRecordIDRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function WhyEntityByRecordID(\G2engine\WhyEntityByRecordIDRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/WhyEntityByRecordID',
        $argument,
        ['\G2engine\WhyEntityByRecordIDResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\WhyEntityByRecordID_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function WhyEntityByRecordID_V2(\G2engine\WhyEntityByRecordID_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/WhyEntityByRecordID_V2',
        $argument,
        ['\G2engine\WhyEntityByRecordID_V2Response', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\WhyRecordsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function WhyRecords(\G2engine\WhyRecordsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/WhyRecords',
        $argument,
        ['\G2engine\WhyRecordsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \G2engine\WhyRecords_V2Request $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function WhyRecords_V2(\G2engine\WhyRecords_V2Request $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/g2engine.G2Diagnostic/WhyRecords_V2',
        $argument,
        ['\G2engine\WhyRecords_V2Response', 'decode'],
        $metadata, $options);
    }

}
