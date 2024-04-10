<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Szengine;

/**
 */
class SzEngineClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * @param \Szengine\AddRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddRecord(\Szengine\AddRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/AddRecord',
        $argument,
        ['\Szengine\AddRecordResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\CloseExportRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CloseExport(\Szengine\CloseExportRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/CloseExport',
        $argument,
        ['\Szengine\CloseExportResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\CountRedoRecordsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CountRedoRecords(\Szengine\CountRedoRecordsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/CountRedoRecords',
        $argument,
        ['\Szengine\CountRedoRecordsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\DeleteRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteRecord(\Szengine\DeleteRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/DeleteRecord',
        $argument,
        ['\Szengine\DeleteRecordResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\ExportCsvEntityReportRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ExportCsvEntityReport(\Szengine\ExportCsvEntityReportRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/ExportCsvEntityReport',
        $argument,
        ['\Szengine\ExportCsvEntityReportResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\ExportJsonEntityReportRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ExportJsonEntityReport(\Szengine\ExportJsonEntityReportRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/ExportJsonEntityReport',
        $argument,
        ['\Szengine\ExportJsonEntityReportResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\FetchNextRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FetchNext(\Szengine\FetchNextRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/FetchNext',
        $argument,
        ['\Szengine\FetchNextResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\FindNetworkByEntityIdRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindNetworkByEntityId(\Szengine\FindNetworkByEntityIdRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/FindNetworkByEntityId',
        $argument,
        ['\Szengine\FindNetworkByEntityIdResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\FindNetworkByRecordIdRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindNetworkByRecordId(\Szengine\FindNetworkByRecordIdRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/FindNetworkByRecordId',
        $argument,
        ['\Szengine\FindNetworkByRecordIdResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\FindPathByEntityIdRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindPathByEntityId(\Szengine\FindPathByEntityIdRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/FindPathByEntityId',
        $argument,
        ['\Szengine\FindPathByEntityIdResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\FindPathByRecordIdRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FindPathByRecordId(\Szengine\FindPathByRecordIdRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/FindPathByRecordId',
        $argument,
        ['\Szengine\FindPathByRecordIdResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\GetActiveConfigIdRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetActiveConfigId(\Szengine\GetActiveConfigIdRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/GetActiveConfigId',
        $argument,
        ['\Szengine\GetActiveConfigIdResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\GetEntityByEntityIdRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetEntityByEntityId(\Szengine\GetEntityByEntityIdRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/GetEntityByEntityId',
        $argument,
        ['\Szengine\GetEntityByEntityIdResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\GetEntityByRecordIdRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetEntityByRecordId(\Szengine\GetEntityByRecordIdRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/GetEntityByRecordId',
        $argument,
        ['\Szengine\GetEntityByRecordIdResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\GetRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetRecord(\Szengine\GetRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/GetRecord',
        $argument,
        ['\Szengine\GetRecordResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\GetRedoRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetRedoRecord(\Szengine\GetRedoRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/GetRedoRecord',
        $argument,
        ['\Szengine\GetRedoRecordResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\GetRepositoryLastModifiedTimeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetRepositoryLastModifiedTime(\Szengine\GetRepositoryLastModifiedTimeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/GetRepositoryLastModifiedTime',
        $argument,
        ['\Szengine\GetRepositoryLastModifiedTimeResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\GetStatsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetStats(\Szengine\GetStatsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/GetStats',
        $argument,
        ['\Szengine\GetStatsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\GetVirtualEntityByRecordIdRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetVirtualEntityByRecordId(\Szengine\GetVirtualEntityByRecordIdRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/GetVirtualEntityByRecordId',
        $argument,
        ['\Szengine\GetVirtualEntityByRecordIdResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\HowEntityByEntityIdRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function HowEntityByEntityId(\Szengine\HowEntityByEntityIdRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/HowEntityByEntityId',
        $argument,
        ['\Szengine\HowEntityByEntityIdResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\PrimeEngineRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function PrimeEngine(\Szengine\PrimeEngineRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/PrimeEngine',
        $argument,
        ['\Szengine\PrimeEngineResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\ProcessRedoRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ProcessRedoRecord(\Szengine\ProcessRedoRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/ProcessRedoRecord',
        $argument,
        ['\Szengine\ProcessRedoRecordResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\ReevaluateEntityRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ReevaluateEntity(\Szengine\ReevaluateEntityRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/ReevaluateEntity',
        $argument,
        ['\Szengine\ReevaluateEntityResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\ReevaluateRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ReevaluateRecord(\Szengine\ReevaluateRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/ReevaluateRecord',
        $argument,
        ['\Szengine\ReevaluateRecordResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\ReplaceRecordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ReplaceRecord(\Szengine\ReplaceRecordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/ReplaceRecord',
        $argument,
        ['\Szengine\ReplaceRecordResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\SearchByAttributesRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function SearchByAttributes(\Szengine\SearchByAttributesRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/SearchByAttributes',
        $argument,
        ['\Szengine\SearchByAttributesResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\StreamExportCsvEntityReportRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ServerStreamingCall
     */
    public function StreamExportCsvEntityReport(\Szengine\StreamExportCsvEntityReportRequest $argument,
      $metadata = [], $options = []) {
        return $this->_serverStreamRequest('/szengine.SzEngine/StreamExportCsvEntityReport',
        $argument,
        ['\Szengine\StreamExportCsvEntityReportResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\StreamExportJsonEntityReportRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ServerStreamingCall
     */
    public function StreamExportJsonEntityReport(\Szengine\StreamExportJsonEntityReportRequest $argument,
      $metadata = [], $options = []) {
        return $this->_serverStreamRequest('/szengine.SzEngine/StreamExportJsonEntityReport',
        $argument,
        ['\Szengine\StreamExportJsonEntityReportResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\WhyEntitiesRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function WhyEntities(\Szengine\WhyEntitiesRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/WhyEntities',
        $argument,
        ['\Szengine\WhyEntitiesResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\WhyRecordInEntityRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function WhyRecordInEntity(\Szengine\WhyRecordInEntityRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/WhyRecordInEntity',
        $argument,
        ['\Szengine\WhyRecordInEntityResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * @param \Szengine\WhyRecordsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function WhyRecords(\Szengine\WhyRecordsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/szengine.SzEngine/WhyRecords',
        $argument,
        ['\Szengine\WhyRecordsResponse', 'decode'],
        $metadata, $options);
    }

}
