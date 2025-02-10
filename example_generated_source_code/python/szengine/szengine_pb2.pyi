from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class AddRecordRequest(_message.Message):
    __slots__ = ("dataSourceCode", "recordId", "recordDefinition", "flags")
    DATASOURCECODE_FIELD_NUMBER: _ClassVar[int]
    RECORDID_FIELD_NUMBER: _ClassVar[int]
    RECORDDEFINITION_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    dataSourceCode: str
    recordId: str
    recordDefinition: str
    flags: int
    def __init__(self, dataSourceCode: _Optional[str] = ..., recordId: _Optional[str] = ..., recordDefinition: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class AddRecordResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class CloseExportRequest(_message.Message):
    __slots__ = ("exportHandle",)
    EXPORTHANDLE_FIELD_NUMBER: _ClassVar[int]
    exportHandle: int
    def __init__(self, exportHandle: _Optional[int] = ...) -> None: ...

class CloseExportResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class CountRedoRecordsRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class CountRedoRecordsResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: int
    def __init__(self, result: _Optional[int] = ...) -> None: ...

class DeleteRecordRequest(_message.Message):
    __slots__ = ("dataSourceCode", "recordId", "flags")
    DATASOURCECODE_FIELD_NUMBER: _ClassVar[int]
    RECORDID_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    dataSourceCode: str
    recordId: str
    flags: int
    def __init__(self, dataSourceCode: _Optional[str] = ..., recordId: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class DeleteRecordResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class ExportCsvEntityReportRequest(_message.Message):
    __slots__ = ("csvColumnList", "flags")
    CSVCOLUMNLIST_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    csvColumnList: str
    flags: int
    def __init__(self, csvColumnList: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class ExportCsvEntityReportResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: int
    def __init__(self, result: _Optional[int] = ...) -> None: ...

class ExportJsonEntityReportRequest(_message.Message):
    __slots__ = ("flags",)
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    flags: int
    def __init__(self, flags: _Optional[int] = ...) -> None: ...

class ExportJsonEntityReportResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: int
    def __init__(self, result: _Optional[int] = ...) -> None: ...

class FetchNextRequest(_message.Message):
    __slots__ = ("exportHandle",)
    EXPORTHANDLE_FIELD_NUMBER: _ClassVar[int]
    exportHandle: int
    def __init__(self, exportHandle: _Optional[int] = ...) -> None: ...

class FetchNextResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class FindInterestingEntitiesByEntityIdRequest(_message.Message):
    __slots__ = ("entityId", "flags")
    ENTITYID_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    entityId: int
    flags: int
    def __init__(self, entityId: _Optional[int] = ..., flags: _Optional[int] = ...) -> None: ...

class FindInterestingEntitiesByEntityIdResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class FindInterestingEntitiesByRecordIdRequest(_message.Message):
    __slots__ = ("dataSourceCode", "recordId", "flags")
    DATASOURCECODE_FIELD_NUMBER: _ClassVar[int]
    RECORDID_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    dataSourceCode: str
    recordId: str
    flags: int
    def __init__(self, dataSourceCode: _Optional[str] = ..., recordId: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class FindInterestingEntitiesByRecordIdResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class FindNetworkByEntityIdRequest(_message.Message):
    __slots__ = ("entityIds", "maxDegrees", "buildOutDegrees", "buildOutMaxEntities", "flags")
    ENTITYIDS_FIELD_NUMBER: _ClassVar[int]
    MAXDEGREES_FIELD_NUMBER: _ClassVar[int]
    BUILDOUTDEGREES_FIELD_NUMBER: _ClassVar[int]
    BUILDOUTMAXENTITIES_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    entityIds: str
    maxDegrees: int
    buildOutDegrees: int
    buildOutMaxEntities: int
    flags: int
    def __init__(self, entityIds: _Optional[str] = ..., maxDegrees: _Optional[int] = ..., buildOutDegrees: _Optional[int] = ..., buildOutMaxEntities: _Optional[int] = ..., flags: _Optional[int] = ...) -> None: ...

class FindNetworkByEntityIdResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class FindNetworkByRecordIdRequest(_message.Message):
    __slots__ = ("recordKeys", "maxDegrees", "buildOutDegrees", "buildOutMaxEntities", "flags")
    RECORDKEYS_FIELD_NUMBER: _ClassVar[int]
    MAXDEGREES_FIELD_NUMBER: _ClassVar[int]
    BUILDOUTDEGREES_FIELD_NUMBER: _ClassVar[int]
    BUILDOUTMAXENTITIES_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    recordKeys: str
    maxDegrees: int
    buildOutDegrees: int
    buildOutMaxEntities: int
    flags: int
    def __init__(self, recordKeys: _Optional[str] = ..., maxDegrees: _Optional[int] = ..., buildOutDegrees: _Optional[int] = ..., buildOutMaxEntities: _Optional[int] = ..., flags: _Optional[int] = ...) -> None: ...

class FindNetworkByRecordIdResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class FindPathByEntityIdRequest(_message.Message):
    __slots__ = ("startEntityId", "endEntityId", "maxDegrees", "avoidEntityIds", "requiredDataSources", "flags")
    STARTENTITYID_FIELD_NUMBER: _ClassVar[int]
    ENDENTITYID_FIELD_NUMBER: _ClassVar[int]
    MAXDEGREES_FIELD_NUMBER: _ClassVar[int]
    AVOIDENTITYIDS_FIELD_NUMBER: _ClassVar[int]
    REQUIREDDATASOURCES_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    startEntityId: int
    endEntityId: int
    maxDegrees: int
    avoidEntityIds: str
    requiredDataSources: str
    flags: int
    def __init__(self, startEntityId: _Optional[int] = ..., endEntityId: _Optional[int] = ..., maxDegrees: _Optional[int] = ..., avoidEntityIds: _Optional[str] = ..., requiredDataSources: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class FindPathByEntityIdResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class FindPathByRecordIdRequest(_message.Message):
    __slots__ = ("startDataSourceCode", "startRecordId", "endDataSourceCode", "endRecordId", "maxDegrees", "avoidRecordKeys", "requiredDataSources", "flags")
    STARTDATASOURCECODE_FIELD_NUMBER: _ClassVar[int]
    STARTRECORDID_FIELD_NUMBER: _ClassVar[int]
    ENDDATASOURCECODE_FIELD_NUMBER: _ClassVar[int]
    ENDRECORDID_FIELD_NUMBER: _ClassVar[int]
    MAXDEGREES_FIELD_NUMBER: _ClassVar[int]
    AVOIDRECORDKEYS_FIELD_NUMBER: _ClassVar[int]
    REQUIREDDATASOURCES_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    startDataSourceCode: str
    startRecordId: str
    endDataSourceCode: str
    endRecordId: str
    maxDegrees: int
    avoidRecordKeys: str
    requiredDataSources: str
    flags: int
    def __init__(self, startDataSourceCode: _Optional[str] = ..., startRecordId: _Optional[str] = ..., endDataSourceCode: _Optional[str] = ..., endRecordId: _Optional[str] = ..., maxDegrees: _Optional[int] = ..., avoidRecordKeys: _Optional[str] = ..., requiredDataSources: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class FindPathByRecordIdResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class GetActiveConfigIdRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class GetActiveConfigIdResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: int
    def __init__(self, result: _Optional[int] = ...) -> None: ...

class GetEntityByEntityIdRequest(_message.Message):
    __slots__ = ("entityId", "flags")
    ENTITYID_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    entityId: int
    flags: int
    def __init__(self, entityId: _Optional[int] = ..., flags: _Optional[int] = ...) -> None: ...

class GetEntityByEntityIdResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class GetEntityByRecordIdRequest(_message.Message):
    __slots__ = ("dataSourceCode", "recordId", "flags")
    DATASOURCECODE_FIELD_NUMBER: _ClassVar[int]
    RECORDID_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    dataSourceCode: str
    recordId: str
    flags: int
    def __init__(self, dataSourceCode: _Optional[str] = ..., recordId: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class GetEntityByRecordIdResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class GetRecordRequest(_message.Message):
    __slots__ = ("dataSourceCode", "recordId", "flags")
    DATASOURCECODE_FIELD_NUMBER: _ClassVar[int]
    RECORDID_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    dataSourceCode: str
    recordId: str
    flags: int
    def __init__(self, dataSourceCode: _Optional[str] = ..., recordId: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class GetRecordResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class GetRedoRecordRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class GetRedoRecordResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class GetStatsRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class GetStatsResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class GetVirtualEntityByRecordIdRequest(_message.Message):
    __slots__ = ("recordKeys", "flags")
    RECORDKEYS_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    recordKeys: str
    flags: int
    def __init__(self, recordKeys: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class GetVirtualEntityByRecordIdResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class HowEntityByEntityIdRequest(_message.Message):
    __slots__ = ("entityId", "flags")
    ENTITYID_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    entityId: int
    flags: int
    def __init__(self, entityId: _Optional[int] = ..., flags: _Optional[int] = ...) -> None: ...

class HowEntityByEntityIdResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class PreprocessRecordRequest(_message.Message):
    __slots__ = ("recordDefinition", "flags")
    RECORDDEFINITION_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    recordDefinition: str
    flags: int
    def __init__(self, recordDefinition: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class PreprocessRecordResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class PrimeEngineRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class PrimeEngineResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class ProcessRedoRecordRequest(_message.Message):
    __slots__ = ("redoRecord", "flags")
    REDORECORD_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    redoRecord: str
    flags: int
    def __init__(self, redoRecord: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class ProcessRedoRecordResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class ReevaluateEntityRequest(_message.Message):
    __slots__ = ("entityId", "flags")
    ENTITYID_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    entityId: int
    flags: int
    def __init__(self, entityId: _Optional[int] = ..., flags: _Optional[int] = ...) -> None: ...

class ReevaluateEntityResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class ReevaluateRecordRequest(_message.Message):
    __slots__ = ("dataSourceCode", "recordId", "flags")
    DATASOURCECODE_FIELD_NUMBER: _ClassVar[int]
    RECORDID_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    dataSourceCode: str
    recordId: str
    flags: int
    def __init__(self, dataSourceCode: _Optional[str] = ..., recordId: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class ReevaluateRecordResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class ReinitializeRequest(_message.Message):
    __slots__ = ("configId",)
    CONFIGID_FIELD_NUMBER: _ClassVar[int]
    configId: int
    def __init__(self, configId: _Optional[int] = ...) -> None: ...

class ReinitializeResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class SearchByAttributesRequest(_message.Message):
    __slots__ = ("attributes", "searchProfile", "flags")
    ATTRIBUTES_FIELD_NUMBER: _ClassVar[int]
    SEARCHPROFILE_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    attributes: str
    searchProfile: str
    flags: int
    def __init__(self, attributes: _Optional[str] = ..., searchProfile: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class SearchByAttributesResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class StreamExportCsvEntityReportRequest(_message.Message):
    __slots__ = ("csvColumnList", "flags")
    CSVCOLUMNLIST_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    csvColumnList: str
    flags: int
    def __init__(self, csvColumnList: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class StreamExportCsvEntityReportResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class StreamExportJsonEntityReportRequest(_message.Message):
    __slots__ = ("flags",)
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    flags: int
    def __init__(self, flags: _Optional[int] = ...) -> None: ...

class StreamExportJsonEntityReportResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class WhyEntitiesRequest(_message.Message):
    __slots__ = ("entityId1", "entityId2", "flags")
    ENTITYID1_FIELD_NUMBER: _ClassVar[int]
    ENTITYID2_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    entityId1: int
    entityId2: int
    flags: int
    def __init__(self, entityId1: _Optional[int] = ..., entityId2: _Optional[int] = ..., flags: _Optional[int] = ...) -> None: ...

class WhyEntitiesResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class WhyRecordInEntityRequest(_message.Message):
    __slots__ = ("dataSourceCode", "recordId", "flags")
    DATASOURCECODE_FIELD_NUMBER: _ClassVar[int]
    RECORDID_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    dataSourceCode: str
    recordId: str
    flags: int
    def __init__(self, dataSourceCode: _Optional[str] = ..., recordId: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class WhyRecordInEntityResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class WhyRecordsRequest(_message.Message):
    __slots__ = ("dataSourceCode1", "recordId1", "dataSourceCode2", "recordId2", "flags")
    DATASOURCECODE1_FIELD_NUMBER: _ClassVar[int]
    RECORDID1_FIELD_NUMBER: _ClassVar[int]
    DATASOURCECODE2_FIELD_NUMBER: _ClassVar[int]
    RECORDID2_FIELD_NUMBER: _ClassVar[int]
    FLAGS_FIELD_NUMBER: _ClassVar[int]
    dataSourceCode1: str
    recordId1: str
    dataSourceCode2: str
    recordId2: str
    flags: int
    def __init__(self, dataSourceCode1: _Optional[str] = ..., recordId1: _Optional[str] = ..., dataSourceCode2: _Optional[str] = ..., recordId2: _Optional[str] = ..., flags: _Optional[int] = ...) -> None: ...

class WhyRecordsResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...
