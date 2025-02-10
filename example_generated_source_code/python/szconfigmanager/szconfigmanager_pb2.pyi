from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class AddConfigRequest(_message.Message):
    __slots__ = ("configDefinition", "configComment")
    CONFIGDEFINITION_FIELD_NUMBER: _ClassVar[int]
    CONFIGCOMMENT_FIELD_NUMBER: _ClassVar[int]
    configDefinition: str
    configComment: str
    def __init__(self, configDefinition: _Optional[str] = ..., configComment: _Optional[str] = ...) -> None: ...

class AddConfigResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: int
    def __init__(self, result: _Optional[int] = ...) -> None: ...

class GetConfigRequest(_message.Message):
    __slots__ = ("configId",)
    CONFIGID_FIELD_NUMBER: _ClassVar[int]
    configId: int
    def __init__(self, configId: _Optional[int] = ...) -> None: ...

class GetConfigResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class GetConfigsRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class GetConfigsResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: str
    def __init__(self, result: _Optional[str] = ...) -> None: ...

class GetDefaultConfigIdRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class GetDefaultConfigIdResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: int
    def __init__(self, result: _Optional[int] = ...) -> None: ...

class ReplaceDefaultConfigIdRequest(_message.Message):
    __slots__ = ("currentDefaultConfigId", "newDefaultConfigId")
    CURRENTDEFAULTCONFIGID_FIELD_NUMBER: _ClassVar[int]
    NEWDEFAULTCONFIGID_FIELD_NUMBER: _ClassVar[int]
    currentDefaultConfigId: int
    newDefaultConfigId: int
    def __init__(self, currentDefaultConfigId: _Optional[int] = ..., newDefaultConfigId: _Optional[int] = ...) -> None: ...

class ReplaceDefaultConfigIdResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class SetDefaultConfigIdRequest(_message.Message):
    __slots__ = ("configId",)
    CONFIGID_FIELD_NUMBER: _ClassVar[int]
    configId: int
    def __init__(self, configId: _Optional[int] = ...) -> None: ...

class SetDefaultConfigIdResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...
