from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class CheckDatastorePerformanceRequest(_message.Message):
    __slots__ = ("secondsToRun",)
    SECONDSTORUN_FIELD_NUMBER: _ClassVar[int]
    secondsToRun: int
    def __init__(self, secondsToRun: _Optional[int] = ...) -> None: ...

class CheckDatastorePerformanceResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class GetDatastoreInfoRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class GetDatastoreInfoResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class GetFeatureRequest(_message.Message):
    __slots__ = ("featureId",)
    FEATUREID_FIELD_NUMBER: _ClassVar[int]
    featureId: int
    def __init__(self, featureId: _Optional[int] = ...) -> None: ...

class GetFeatureResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class PurgeRepositoryRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class PurgeRepositoryResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class ReinitializeRequest(_message.Message):
    __slots__ = ("configId",)
    CONFIGID_FIELD_NUMBER: _ClassVar[int]
    configId: int
    def __init__(self, configId: _Optional[int] = ...) -> None: ...

class ReinitializeResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...
