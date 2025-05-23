// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.36.6
// 	protoc        v3.21.12
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

type AddDataSourceRequest struct {
	state            protoimpl.MessageState `protogen:"open.v1"`
	ConfigDefinition string                 `protobuf:"bytes,1,opt,name=config_definition,json=configDefinition,proto3" json:"config_definition,omitempty"`
	DataSourceCode   string                 `protobuf:"bytes,2,opt,name=data_source_code,json=dataSourceCode,proto3" json:"data_source_code,omitempty"`
	unknownFields    protoimpl.UnknownFields
	sizeCache        protoimpl.SizeCache
}

func (x *AddDataSourceRequest) Reset() {
	*x = AddDataSourceRequest{}
	mi := &file_szconfig_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *AddDataSourceRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AddDataSourceRequest) ProtoMessage() {}

func (x *AddDataSourceRequest) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use AddDataSourceRequest.ProtoReflect.Descriptor instead.
func (*AddDataSourceRequest) Descriptor() ([]byte, []int) {
	return file_szconfig_proto_rawDescGZIP(), []int{0}
}

func (x *AddDataSourceRequest) GetConfigDefinition() string {
	if x != nil {
		return x.ConfigDefinition
	}
	return ""
}

func (x *AddDataSourceRequest) GetDataSourceCode() string {
	if x != nil {
		return x.DataSourceCode
	}
	return ""
}

type AddDataSourceResponse struct {
	state            protoimpl.MessageState `protogen:"open.v1"`
	Result           string                 `protobuf:"bytes,1,opt,name=result,proto3" json:"result,omitempty"`
	ConfigDefinition string                 `protobuf:"bytes,2,opt,name=config_definition,json=configDefinition,proto3" json:"config_definition,omitempty"`
	unknownFields    protoimpl.UnknownFields
	sizeCache        protoimpl.SizeCache
}

func (x *AddDataSourceResponse) Reset() {
	*x = AddDataSourceResponse{}
	mi := &file_szconfig_proto_msgTypes[1]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *AddDataSourceResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AddDataSourceResponse) ProtoMessage() {}

func (x *AddDataSourceResponse) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use AddDataSourceResponse.ProtoReflect.Descriptor instead.
func (*AddDataSourceResponse) Descriptor() ([]byte, []int) {
	return file_szconfig_proto_rawDescGZIP(), []int{1}
}

func (x *AddDataSourceResponse) GetResult() string {
	if x != nil {
		return x.Result
	}
	return ""
}

func (x *AddDataSourceResponse) GetConfigDefinition() string {
	if x != nil {
		return x.ConfigDefinition
	}
	return ""
}

type DeleteDataSourceRequest struct {
	state            protoimpl.MessageState `protogen:"open.v1"`
	ConfigDefinition string                 `protobuf:"bytes,1,opt,name=config_definition,json=configDefinition,proto3" json:"config_definition,omitempty"`
	DataSourceCode   string                 `protobuf:"bytes,2,opt,name=data_source_code,json=dataSourceCode,proto3" json:"data_source_code,omitempty"`
	unknownFields    protoimpl.UnknownFields
	sizeCache        protoimpl.SizeCache
}

func (x *DeleteDataSourceRequest) Reset() {
	*x = DeleteDataSourceRequest{}
	mi := &file_szconfig_proto_msgTypes[2]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *DeleteDataSourceRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DeleteDataSourceRequest) ProtoMessage() {}

func (x *DeleteDataSourceRequest) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use DeleteDataSourceRequest.ProtoReflect.Descriptor instead.
func (*DeleteDataSourceRequest) Descriptor() ([]byte, []int) {
	return file_szconfig_proto_rawDescGZIP(), []int{2}
}

func (x *DeleteDataSourceRequest) GetConfigDefinition() string {
	if x != nil {
		return x.ConfigDefinition
	}
	return ""
}

func (x *DeleteDataSourceRequest) GetDataSourceCode() string {
	if x != nil {
		return x.DataSourceCode
	}
	return ""
}

type DeleteDataSourceResponse struct {
	state            protoimpl.MessageState `protogen:"open.v1"`
	Result           string                 `protobuf:"bytes,1,opt,name=result,proto3" json:"result,omitempty"`
	ConfigDefinition string                 `protobuf:"bytes,2,opt,name=config_definition,json=configDefinition,proto3" json:"config_definition,omitempty"`
	unknownFields    protoimpl.UnknownFields
	sizeCache        protoimpl.SizeCache
}

func (x *DeleteDataSourceResponse) Reset() {
	*x = DeleteDataSourceResponse{}
	mi := &file_szconfig_proto_msgTypes[3]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *DeleteDataSourceResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DeleteDataSourceResponse) ProtoMessage() {}

func (x *DeleteDataSourceResponse) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use DeleteDataSourceResponse.ProtoReflect.Descriptor instead.
func (*DeleteDataSourceResponse) Descriptor() ([]byte, []int) {
	return file_szconfig_proto_rawDescGZIP(), []int{3}
}

func (x *DeleteDataSourceResponse) GetResult() string {
	if x != nil {
		return x.Result
	}
	return ""
}

func (x *DeleteDataSourceResponse) GetConfigDefinition() string {
	if x != nil {
		return x.ConfigDefinition
	}
	return ""
}

type GetDataSourcesRequest struct {
	state            protoimpl.MessageState `protogen:"open.v1"`
	ConfigDefinition string                 `protobuf:"bytes,1,opt,name=config_definition,json=configDefinition,proto3" json:"config_definition,omitempty"`
	unknownFields    protoimpl.UnknownFields
	sizeCache        protoimpl.SizeCache
}

func (x *GetDataSourcesRequest) Reset() {
	*x = GetDataSourcesRequest{}
	mi := &file_szconfig_proto_msgTypes[4]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *GetDataSourcesRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetDataSourcesRequest) ProtoMessage() {}

func (x *GetDataSourcesRequest) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use GetDataSourcesRequest.ProtoReflect.Descriptor instead.
func (*GetDataSourcesRequest) Descriptor() ([]byte, []int) {
	return file_szconfig_proto_rawDescGZIP(), []int{4}
}

func (x *GetDataSourcesRequest) GetConfigDefinition() string {
	if x != nil {
		return x.ConfigDefinition
	}
	return ""
}

type GetDataSourcesResponse struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	Result        string                 `protobuf:"bytes,1,opt,name=result,proto3" json:"result,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *GetDataSourcesResponse) Reset() {
	*x = GetDataSourcesResponse{}
	mi := &file_szconfig_proto_msgTypes[5]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *GetDataSourcesResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetDataSourcesResponse) ProtoMessage() {}

func (x *GetDataSourcesResponse) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use GetDataSourcesResponse.ProtoReflect.Descriptor instead.
func (*GetDataSourcesResponse) Descriptor() ([]byte, []int) {
	return file_szconfig_proto_rawDescGZIP(), []int{5}
}

func (x *GetDataSourcesResponse) GetResult() string {
	if x != nil {
		return x.Result
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
	"\x0eszconfig.proto\x12\bszconfig\"m\n" +
	"\x14AddDataSourceRequest\x12+\n" +
	"\x11config_definition\x18\x01 \x01(\tR\x10configDefinition\x12(\n" +
	"\x10data_source_code\x18\x02 \x01(\tR\x0edataSourceCode\"\\\n" +
	"\x15AddDataSourceResponse\x12\x16\n" +
	"\x06result\x18\x01 \x01(\tR\x06result\x12+\n" +
	"\x11config_definition\x18\x02 \x01(\tR\x10configDefinition\"p\n" +
	"\x17DeleteDataSourceRequest\x12+\n" +
	"\x11config_definition\x18\x01 \x01(\tR\x10configDefinition\x12(\n" +
	"\x10data_source_code\x18\x02 \x01(\tR\x0edataSourceCode\"_\n" +
	"\x18DeleteDataSourceResponse\x12\x16\n" +
	"\x06result\x18\x01 \x01(\tR\x06result\x12+\n" +
	"\x11config_definition\x18\x02 \x01(\tR\x10configDefinition\"D\n" +
	"\x15GetDataSourcesRequest\x12+\n" +
	"\x11config_definition\x18\x01 \x01(\tR\x10configDefinition\"0\n" +
	"\x16GetDataSourcesResponse\x12\x16\n" +
	"\x06result\x18\x01 \x01(\tR\x06result\"B\n" +
	"\x13VerifyConfigRequest\x12+\n" +
	"\x11config_definition\x18\x01 \x01(\tR\x10configDefinition\".\n" +
	"\x14VerifyConfigResponse\x12\x16\n" +
	"\x06result\x18\x01 \x01(\bR\x06result2\xe3\x02\n" +
	"\bSzConfig\x12R\n" +
	"\rAddDataSource\x12\x1e.szconfig.AddDataSourceRequest\x1a\x1f.szconfig.AddDataSourceResponse\"\x00\x12[\n" +
	"\x10DeleteDataSource\x12!.szconfig.DeleteDataSourceRequest\x1a\".szconfig.DeleteDataSourceResponse\"\x00\x12U\n" +
	"\x0eGetDataSources\x12\x1f.szconfig.GetDataSourcesRequest\x1a .szconfig.GetDataSourcesResponse\"\x00\x12O\n" +
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
	(*AddDataSourceRequest)(nil),     // 0: szconfig.AddDataSourceRequest
	(*AddDataSourceResponse)(nil),    // 1: szconfig.AddDataSourceResponse
	(*DeleteDataSourceRequest)(nil),  // 2: szconfig.DeleteDataSourceRequest
	(*DeleteDataSourceResponse)(nil), // 3: szconfig.DeleteDataSourceResponse
	(*GetDataSourcesRequest)(nil),    // 4: szconfig.GetDataSourcesRequest
	(*GetDataSourcesResponse)(nil),   // 5: szconfig.GetDataSourcesResponse
	(*VerifyConfigRequest)(nil),      // 6: szconfig.VerifyConfigRequest
	(*VerifyConfigResponse)(nil),     // 7: szconfig.VerifyConfigResponse
}
var file_szconfig_proto_depIdxs = []int32{
	0, // 0: szconfig.SzConfig.AddDataSource:input_type -> szconfig.AddDataSourceRequest
	2, // 1: szconfig.SzConfig.DeleteDataSource:input_type -> szconfig.DeleteDataSourceRequest
	4, // 2: szconfig.SzConfig.GetDataSources:input_type -> szconfig.GetDataSourcesRequest
	6, // 3: szconfig.SzConfig.VerifyConfig:input_type -> szconfig.VerifyConfigRequest
	1, // 4: szconfig.SzConfig.AddDataSource:output_type -> szconfig.AddDataSourceResponse
	3, // 5: szconfig.SzConfig.DeleteDataSource:output_type -> szconfig.DeleteDataSourceResponse
	5, // 6: szconfig.SzConfig.GetDataSources:output_type -> szconfig.GetDataSourcesResponse
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
