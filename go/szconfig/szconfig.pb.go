// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.36.6
// 	protoc        v6.30.2
// source: szconfig.proto

package szconfig

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
	unsafe "unsafe"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type GetDataSourceRegistryRequest struct {
	state            protoimpl.MessageState `protogen:"open.v1"`
	ConfigDefinition string                 `protobuf:"bytes,1,opt,name=config_definition,json=configDefinition,proto3" json:"config_definition,omitempty"`
	unknownFields    protoimpl.UnknownFields
	sizeCache        protoimpl.SizeCache
}

func (x *GetDataSourceRegistryRequest) Reset() {
	*x = GetDataSourceRegistryRequest{}
	mi := &file_szconfig_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *GetDataSourceRegistryRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetDataSourceRegistryRequest) ProtoMessage() {}

func (x *GetDataSourceRegistryRequest) ProtoReflect() protoreflect.Message {
	mi := &file_szconfig_proto_msgTypes[0]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetDataSourceRegistryRequest.ProtoReflect.Descriptor instead.
func (*GetDataSourceRegistryRequest) Descriptor() ([]byte, []int) {
	return file_szconfig_proto_rawDescGZIP(), []int{0}
}

func (x *GetDataSourceRegistryRequest) GetConfigDefinition() string {
	if x != nil {
		return x.ConfigDefinition
	}
	return ""
}

type GetDataSourceRegistryResponse struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	Result        string                 `protobuf:"bytes,1,opt,name=result,proto3" json:"result,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *GetDataSourceRegistryResponse) Reset() {
	*x = GetDataSourceRegistryResponse{}
	mi := &file_szconfig_proto_msgTypes[1]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *GetDataSourceRegistryResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetDataSourceRegistryResponse) ProtoMessage() {}

func (x *GetDataSourceRegistryResponse) ProtoReflect() protoreflect.Message {
	mi := &file_szconfig_proto_msgTypes[1]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetDataSourceRegistryResponse.ProtoReflect.Descriptor instead.
func (*GetDataSourceRegistryResponse) Descriptor() ([]byte, []int) {
	return file_szconfig_proto_rawDescGZIP(), []int{1}
}

func (x *GetDataSourceRegistryResponse) GetResult() string {
	if x != nil {
		return x.Result
	}
	return ""
}

type RegisterDataSourceRequest struct {
	state            protoimpl.MessageState `protogen:"open.v1"`
	ConfigDefinition string                 `protobuf:"bytes,1,opt,name=config_definition,json=configDefinition,proto3" json:"config_definition,omitempty"`
	DataSourceCode   string                 `protobuf:"bytes,2,opt,name=data_source_code,json=dataSourceCode,proto3" json:"data_source_code,omitempty"`
	unknownFields    protoimpl.UnknownFields
	sizeCache        protoimpl.SizeCache
}

func (x *RegisterDataSourceRequest) Reset() {
	*x = RegisterDataSourceRequest{}
	mi := &file_szconfig_proto_msgTypes[2]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *RegisterDataSourceRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*RegisterDataSourceRequest) ProtoMessage() {}

func (x *RegisterDataSourceRequest) ProtoReflect() protoreflect.Message {
	mi := &file_szconfig_proto_msgTypes[2]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use RegisterDataSourceRequest.ProtoReflect.Descriptor instead.
func (*RegisterDataSourceRequest) Descriptor() ([]byte, []int) {
	return file_szconfig_proto_rawDescGZIP(), []int{2}
}

func (x *RegisterDataSourceRequest) GetConfigDefinition() string {
	if x != nil {
		return x.ConfigDefinition
	}
	return ""
}

func (x *RegisterDataSourceRequest) GetDataSourceCode() string {
	if x != nil {
		return x.DataSourceCode
	}
	return ""
}

type RegisterDataSourceResponse struct {
	state            protoimpl.MessageState `protogen:"open.v1"`
	Result           string                 `protobuf:"bytes,1,opt,name=result,proto3" json:"result,omitempty"`
	ConfigDefinition string                 `protobuf:"bytes,2,opt,name=config_definition,json=configDefinition,proto3" json:"config_definition,omitempty"`
	unknownFields    protoimpl.UnknownFields
	sizeCache        protoimpl.SizeCache
}

func (x *RegisterDataSourceResponse) Reset() {
	*x = RegisterDataSourceResponse{}
	mi := &file_szconfig_proto_msgTypes[3]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *RegisterDataSourceResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*RegisterDataSourceResponse) ProtoMessage() {}

func (x *RegisterDataSourceResponse) ProtoReflect() protoreflect.Message {
	mi := &file_szconfig_proto_msgTypes[3]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use RegisterDataSourceResponse.ProtoReflect.Descriptor instead.
func (*RegisterDataSourceResponse) Descriptor() ([]byte, []int) {
	return file_szconfig_proto_rawDescGZIP(), []int{3}
}

func (x *RegisterDataSourceResponse) GetResult() string {
	if x != nil {
		return x.Result
	}
	return ""
}

func (x *RegisterDataSourceResponse) GetConfigDefinition() string {
	if x != nil {
		return x.ConfigDefinition
	}
	return ""
}

type UnregisterDataSourceRequest struct {
	state            protoimpl.MessageState `protogen:"open.v1"`
	ConfigDefinition string                 `protobuf:"bytes,1,opt,name=config_definition,json=configDefinition,proto3" json:"config_definition,omitempty"`
	DataSourceCode   string                 `protobuf:"bytes,2,opt,name=data_source_code,json=dataSourceCode,proto3" json:"data_source_code,omitempty"`
	unknownFields    protoimpl.UnknownFields
	sizeCache        protoimpl.SizeCache
}

func (x *UnregisterDataSourceRequest) Reset() {
	*x = UnregisterDataSourceRequest{}
	mi := &file_szconfig_proto_msgTypes[4]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *UnregisterDataSourceRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UnregisterDataSourceRequest) ProtoMessage() {}

func (x *UnregisterDataSourceRequest) ProtoReflect() protoreflect.Message {
	mi := &file_szconfig_proto_msgTypes[4]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UnregisterDataSourceRequest.ProtoReflect.Descriptor instead.
func (*UnregisterDataSourceRequest) Descriptor() ([]byte, []int) {
	return file_szconfig_proto_rawDescGZIP(), []int{4}
}

func (x *UnregisterDataSourceRequest) GetConfigDefinition() string {
	if x != nil {
		return x.ConfigDefinition
	}
	return ""
}

func (x *UnregisterDataSourceRequest) GetDataSourceCode() string {
	if x != nil {
		return x.DataSourceCode
	}
	return ""
}

type UnregisterDataSourceResponse struct {
	state            protoimpl.MessageState `protogen:"open.v1"`
	Result           string                 `protobuf:"bytes,1,opt,name=result,proto3" json:"result,omitempty"`
	ConfigDefinition string                 `protobuf:"bytes,2,opt,name=config_definition,json=configDefinition,proto3" json:"config_definition,omitempty"`
	unknownFields    protoimpl.UnknownFields
	sizeCache        protoimpl.SizeCache
}

func (x *UnregisterDataSourceResponse) Reset() {
	*x = UnregisterDataSourceResponse{}
	mi := &file_szconfig_proto_msgTypes[5]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *UnregisterDataSourceResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UnregisterDataSourceResponse) ProtoMessage() {}

func (x *UnregisterDataSourceResponse) ProtoReflect() protoreflect.Message {
	mi := &file_szconfig_proto_msgTypes[5]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UnregisterDataSourceResponse.ProtoReflect.Descriptor instead.
func (*UnregisterDataSourceResponse) Descriptor() ([]byte, []int) {
	return file_szconfig_proto_rawDescGZIP(), []int{5}
}

func (x *UnregisterDataSourceResponse) GetResult() string {
	if x != nil {
		return x.Result
	}
	return ""
}

func (x *UnregisterDataSourceResponse) GetConfigDefinition() string {
	if x != nil {
		return x.ConfigDefinition
	}
	return ""
}

type VerifyConfigRequest struct {
	state            protoimpl.MessageState `protogen:"open.v1"`
	ConfigDefinition string                 `protobuf:"bytes,1,opt,name=config_definition,json=configDefinition,proto3" json:"config_definition,omitempty"`
	unknownFields    protoimpl.UnknownFields
	sizeCache        protoimpl.SizeCache
}

func (x *VerifyConfigRequest) Reset() {
	*x = VerifyConfigRequest{}
	mi := &file_szconfig_proto_msgTypes[6]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *VerifyConfigRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*VerifyConfigRequest) ProtoMessage() {}

func (x *VerifyConfigRequest) ProtoReflect() protoreflect.Message {
	mi := &file_szconfig_proto_msgTypes[6]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use VerifyConfigRequest.ProtoReflect.Descriptor instead.
func (*VerifyConfigRequest) Descriptor() ([]byte, []int) {
	return file_szconfig_proto_rawDescGZIP(), []int{6}
}

func (x *VerifyConfigRequest) GetConfigDefinition() string {
	if x != nil {
		return x.ConfigDefinition
	}
	return ""
}

type VerifyConfigResponse struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	Result        bool                   `protobuf:"varint,1,opt,name=result,proto3" json:"result,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *VerifyConfigResponse) Reset() {
	*x = VerifyConfigResponse{}
	mi := &file_szconfig_proto_msgTypes[7]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *VerifyConfigResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*VerifyConfigResponse) ProtoMessage() {}

func (x *VerifyConfigResponse) ProtoReflect() protoreflect.Message {
	mi := &file_szconfig_proto_msgTypes[7]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use VerifyConfigResponse.ProtoReflect.Descriptor instead.
func (*VerifyConfigResponse) Descriptor() ([]byte, []int) {
	return file_szconfig_proto_rawDescGZIP(), []int{7}
}

func (x *VerifyConfigResponse) GetResult() bool {
	if x != nil {
		return x.Result
	}
	return false
}

var File_szconfig_proto protoreflect.FileDescriptor

const file_szconfig_proto_rawDesc = "" +
	"\n" +
	"\x0eszconfig.proto\x12\bszconfig\"K\n" +
	"\x1cGetDataSourceRegistryRequest\x12+\n" +
	"\x11config_definition\x18\x01 \x01(\tR\x10configDefinition\"7\n" +
	"\x1dGetDataSourceRegistryResponse\x12\x16\n" +
	"\x06result\x18\x01 \x01(\tR\x06result\"r\n" +
	"\x19RegisterDataSourceRequest\x12+\n" +
	"\x11config_definition\x18\x01 \x01(\tR\x10configDefinition\x12(\n" +
	"\x10data_source_code\x18\x02 \x01(\tR\x0edataSourceCode\"a\n" +
	"\x1aRegisterDataSourceResponse\x12\x16\n" +
	"\x06result\x18\x01 \x01(\tR\x06result\x12+\n" +
	"\x11config_definition\x18\x02 \x01(\tR\x10configDefinition\"t\n" +
	"\x1bUnregisterDataSourceRequest\x12+\n" +
	"\x11config_definition\x18\x01 \x01(\tR\x10configDefinition\x12(\n" +
	"\x10data_source_code\x18\x02 \x01(\tR\x0edataSourceCode\"c\n" +
	"\x1cUnregisterDataSourceResponse\x12\x16\n" +
	"\x06result\x18\x01 \x01(\tR\x06result\x12+\n" +
	"\x11config_definition\x18\x02 \x01(\tR\x10configDefinition\"B\n" +
	"\x13VerifyConfigRequest\x12+\n" +
	"\x11config_definition\x18\x01 \x01(\tR\x10configDefinition\".\n" +
	"\x14VerifyConfigResponse\x12\x16\n" +
	"\x06result\x18\x01 \x01(\bR\x06result2\x93\x03\n" +
	"\bSzConfig\x12j\n" +
	"\x15GetDataSourceRegistry\x12&.szconfig.GetDataSourceRegistryRequest\x1a'.szconfig.GetDataSourceRegistryResponse\"\x00\x12a\n" +
	"\x12RegisterDataSource\x12#.szconfig.RegisterDataSourceRequest\x1a$.szconfig.RegisterDataSourceResponse\"\x00\x12g\n" +
	"\x14UnregisterDataSource\x12%.szconfig.UnregisterDataSourceRequest\x1a&.szconfig.UnregisterDataSourceResponse\"\x00\x12O\n" +
	"\fVerifyConfig\x12\x1d.szconfig.VerifyConfigRequest\x1a\x1e.szconfig.VerifyConfigResponse\"\x00BX\n" +
	"\x14com.senzing.sdk.grpcB\rSzConfigProtoZ1github.com/senzing-garage/sz-sdk-go-grpc/szconfigb\x06proto3"

var (
	file_szconfig_proto_rawDescOnce sync.Once
	file_szconfig_proto_rawDescData []byte
)

func file_szconfig_proto_rawDescGZIP() []byte {
	file_szconfig_proto_rawDescOnce.Do(func() {
		file_szconfig_proto_rawDescData = protoimpl.X.CompressGZIP(unsafe.Slice(unsafe.StringData(file_szconfig_proto_rawDesc), len(file_szconfig_proto_rawDesc)))
	})
	return file_szconfig_proto_rawDescData
}

var file_szconfig_proto_msgTypes = make([]protoimpl.MessageInfo, 8)
var file_szconfig_proto_goTypes = []any{
	(*GetDataSourceRegistryRequest)(nil),  // 0: szconfig.GetDataSourceRegistryRequest
	(*GetDataSourceRegistryResponse)(nil), // 1: szconfig.GetDataSourceRegistryResponse
	(*RegisterDataSourceRequest)(nil),     // 2: szconfig.RegisterDataSourceRequest
	(*RegisterDataSourceResponse)(nil),    // 3: szconfig.RegisterDataSourceResponse
	(*UnregisterDataSourceRequest)(nil),   // 4: szconfig.UnregisterDataSourceRequest
	(*UnregisterDataSourceResponse)(nil),  // 5: szconfig.UnregisterDataSourceResponse
	(*VerifyConfigRequest)(nil),           // 6: szconfig.VerifyConfigRequest
	(*VerifyConfigResponse)(nil),          // 7: szconfig.VerifyConfigResponse
}
var file_szconfig_proto_depIdxs = []int32{
	0, // 0: szconfig.SzConfig.GetDataSourceRegistry:input_type -> szconfig.GetDataSourceRegistryRequest
	2, // 1: szconfig.SzConfig.RegisterDataSource:input_type -> szconfig.RegisterDataSourceRequest
	4, // 2: szconfig.SzConfig.UnregisterDataSource:input_type -> szconfig.UnregisterDataSourceRequest
	6, // 3: szconfig.SzConfig.VerifyConfig:input_type -> szconfig.VerifyConfigRequest
	1, // 4: szconfig.SzConfig.GetDataSourceRegistry:output_type -> szconfig.GetDataSourceRegistryResponse
	3, // 5: szconfig.SzConfig.RegisterDataSource:output_type -> szconfig.RegisterDataSourceResponse
	5, // 6: szconfig.SzConfig.UnregisterDataSource:output_type -> szconfig.UnregisterDataSourceResponse
	7, // 7: szconfig.SzConfig.VerifyConfig:output_type -> szconfig.VerifyConfigResponse
	4, // [4:8] is the sub-list for method output_type
	0, // [0:4] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_szconfig_proto_init() }
func file_szconfig_proto_init() {
	if File_szconfig_proto != nil {
		return
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: unsafe.Slice(unsafe.StringData(file_szconfig_proto_rawDesc), len(file_szconfig_proto_rawDesc)),
			NumEnums:      0,
			NumMessages:   8,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_szconfig_proto_goTypes,
		DependencyIndexes: file_szconfig_proto_depIdxs,
		MessageInfos:      file_szconfig_proto_msgTypes,
	}.Build()
	File_szconfig_proto = out.File
	file_szconfig_proto_goTypes = nil
	file_szconfig_proto_depIdxs = nil
}
