from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class AddDataSourceRequest(_message.Message):
    __slots__ = ("configHandle", "dataSourceCode")
    CONFIGHANDLE_FIELD_NUMBER: _ClassVar[int]
    DATASOURCECODE_FIELD_NUMBER: _ClassVar[int]
    configHandle: int
    dataSourceCode: str
    def __init__(self, configHandle: _Optional[int] = ..., dataSourceCode: _Optional[str] = ...) -> None: ...

class AddDataSourceResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class CloseConfigRequest(_message.Message):
    __slots__ = ("configHandle",)
    CONFIGHANDLE_FIELD_NUMBER: _ClassVar[int]
    configHandle: int
    def __init__(self, configHandle: _Optional[int] = ...) -> None: ...

class CloseConfigResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class CreateConfigRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class CreateConfigResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: int
    def __init__(self, result: _Optional[int] = ...) -> None: ...

class DeleteDataSourceRequest(_message.Message):
    __slots__ = ("configHandle", "dataSourceCode")
    CONFIGHANDLE_FIELD_NUMBER: _ClassVar[int]
    DATASOURCECODE_FIELD_NUMBER: _ClassVar[int]
    configHandle: int
    dataSourceCode: str
    def __init__(self, configHandle: _Optional[int] = ..., dataSourceCode: _Optional[str] = ...) -> None: ...

class DeleteDataSourceResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class ExportConfigRequest(_message.Message):
    __slots__ = ("configHandle",)
    CONFIGHANDLE_FIELD_NUMBER: _ClassVar[int]
    configHandle: int
    def __init__(self, configHandle: _Optional[int] = ...) -> None: ...

class ExportConfigResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class GetDataSourcesRequest(_message.Message):
    __slots__ = ("configHandle",)
    CONFIGHANDLE_FIELD_NUMBER: _ClassVar[int]
    configHandle: int
    def __init__(self, configHandle: _Optional[int] = ...) -> None: ...

class GetDataSourcesResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class ImportConfigRequest(_message.Message):
    __slots__ = ("configDefinition",)
    CONFIGDEFINITION_FIELD_NUMBER: _ClassVar[int]
    configDefinition: str
    def __init__(self, configDefinition: _Optional[str] = ...) -> None: ...

class ImportConfigResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: int
    def __init__(self, result: _Optional[int] = ...) -> None: ...
