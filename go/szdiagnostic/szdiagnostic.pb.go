// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.36.6
// 	protoc        v6.30.2
// source: szdiagnostic.proto

package szdiagnostic

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

type CheckRepositoryPerformanceRequest struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	SecondsToRun  int32                  `protobuf:"varint,1,opt,name=seconds_to_run,json=secondsToRun,proto3" json:"seconds_to_run,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *CheckRepositoryPerformanceRequest) Reset() {
	*x = CheckRepositoryPerformanceRequest{}
	mi := &file_szdiagnostic_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *CheckRepositoryPerformanceRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CheckRepositoryPerformanceRequest) ProtoMessage() {}

func (x *CheckRepositoryPerformanceRequest) ProtoReflect() protoreflect.Message {
	mi := &file_szdiagnostic_proto_msgTypes[0]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CheckRepositoryPerformanceRequest.ProtoReflect.Descriptor instead.
func (*CheckRepositoryPerformanceRequest) Descriptor() ([]byte, []int) {
	return file_szdiagnostic_proto_rawDescGZIP(), []int{0}
}

func (x *CheckRepositoryPerformanceRequest) GetSecondsToRun() int32 {
	if x != nil {
		return x.SecondsToRun
	}
	return 0
}

type CheckRepositoryPerformanceResponse struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	Result        string                 `protobuf:"bytes,1,opt,name=result,proto3" json:"result,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *CheckRepositoryPerformanceResponse) Reset() {
	*x = CheckRepositoryPerformanceResponse{}
	mi := &file_szdiagnostic_proto_msgTypes[1]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *CheckRepositoryPerformanceResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CheckRepositoryPerformanceResponse) ProtoMessage() {}

func (x *CheckRepositoryPerformanceResponse) ProtoReflect() protoreflect.Message {
	mi := &file_szdiagnostic_proto_msgTypes[1]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CheckRepositoryPerformanceResponse.ProtoReflect.Descriptor instead.
func (*CheckRepositoryPerformanceResponse) Descriptor() ([]byte, []int) {
	return file_szdiagnostic_proto_rawDescGZIP(), []int{1}
}

func (x *CheckRepositoryPerformanceResponse) GetResult() string {
	if x != nil {
		return x.Result
	}
	return ""
}

type GetRepositoryInfoRequest struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *GetRepositoryInfoRequest) Reset() {
	*x = GetRepositoryInfoRequest{}
	mi := &file_szdiagnostic_proto_msgTypes[2]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *GetRepositoryInfoRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetRepositoryInfoRequest) ProtoMessage() {}

func (x *GetRepositoryInfoRequest) ProtoReflect() protoreflect.Message {
	mi := &file_szdiagnostic_proto_msgTypes[2]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetRepositoryInfoRequest.ProtoReflect.Descriptor instead.
func (*GetRepositoryInfoRequest) Descriptor() ([]byte, []int) {
	return file_szdiagnostic_proto_rawDescGZIP(), []int{2}
}

type GetRepositoryInfoResponse struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	Result        string                 `protobuf:"bytes,1,opt,name=result,proto3" json:"result,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *GetRepositoryInfoResponse) Reset() {
	*x = GetRepositoryInfoResponse{}
	mi := &file_szdiagnostic_proto_msgTypes[3]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *GetRepositoryInfoResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetRepositoryInfoResponse) ProtoMessage() {}

func (x *GetRepositoryInfoResponse) ProtoReflect() protoreflect.Message {
	mi := &file_szdiagnostic_proto_msgTypes[3]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetRepositoryInfoResponse.ProtoReflect.Descriptor instead.
func (*GetRepositoryInfoResponse) Descriptor() ([]byte, []int) {
	return file_szdiagnostic_proto_rawDescGZIP(), []int{3}
}

func (x *GetRepositoryInfoResponse) GetResult() string {
	if x != nil {
		return x.Result
	}
	return ""
}

type GetFeatureRequest struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	FeatureId     int64                  `protobuf:"varint,1,opt,name=feature_id,json=featureId,proto3" json:"feature_id,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *GetFeatureRequest) Reset() {
	*x = GetFeatureRequest{}
	mi := &file_szdiagnostic_proto_msgTypes[4]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *GetFeatureRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetFeatureRequest) ProtoMessage() {}

func (x *GetFeatureRequest) ProtoReflect() protoreflect.Message {
	mi := &file_szdiagnostic_proto_msgTypes[4]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetFeatureRequest.ProtoReflect.Descriptor instead.
func (*GetFeatureRequest) Descriptor() ([]byte, []int) {
	return file_szdiagnostic_proto_rawDescGZIP(), []int{4}
}

func (x *GetFeatureRequest) GetFeatureId() int64 {
	if x != nil {
		return x.FeatureId
	}
	return 0
}

type GetFeatureResponse struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	Result        string                 `protobuf:"bytes,1,opt,name=result,proto3" json:"result,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *GetFeatureResponse) Reset() {
	*x = GetFeatureResponse{}
	mi := &file_szdiagnostic_proto_msgTypes[5]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *GetFeatureResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetFeatureResponse) ProtoMessage() {}

func (x *GetFeatureResponse) ProtoReflect() protoreflect.Message {
	mi := &file_szdiagnostic_proto_msgTypes[5]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetFeatureResponse.ProtoReflect.Descriptor instead.
func (*GetFeatureResponse) Descriptor() ([]byte, []int) {
	return file_szdiagnostic_proto_rawDescGZIP(), []int{5}
}

func (x *GetFeatureResponse) GetResult() string {
	if x != nil {
		return x.Result
	}
	return ""
}

type PurgeRepositoryRequest struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *PurgeRepositoryRequest) Reset() {
	*x = PurgeRepositoryRequest{}
	mi := &file_szdiagnostic_proto_msgTypes[6]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *PurgeRepositoryRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*PurgeRepositoryRequest) ProtoMessage() {}

func (x *PurgeRepositoryRequest) ProtoReflect() protoreflect.Message {
	mi := &file_szdiagnostic_proto_msgTypes[6]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use PurgeRepositoryRequest.ProtoReflect.Descriptor instead.
func (*PurgeRepositoryRequest) Descriptor() ([]byte, []int) {
	return file_szdiagnostic_proto_rawDescGZIP(), []int{6}
}

type PurgeRepositoryResponse struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *PurgeRepositoryResponse) Reset() {
	*x = PurgeRepositoryResponse{}
	mi := &file_szdiagnostic_proto_msgTypes[7]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *PurgeRepositoryResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*PurgeRepositoryResponse) ProtoMessage() {}

func (x *PurgeRepositoryResponse) ProtoReflect() protoreflect.Message {
	mi := &file_szdiagnostic_proto_msgTypes[7]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use PurgeRepositoryResponse.ProtoReflect.Descriptor instead.
func (*PurgeRepositoryResponse) Descriptor() ([]byte, []int) {
	return file_szdiagnostic_proto_rawDescGZIP(), []int{7}
}

type ReinitializeRequest struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	ConfigId      int64                  `protobuf:"varint,1,opt,name=config_id,json=configId,proto3" json:"config_id,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *ReinitializeRequest) Reset() {
	*x = ReinitializeRequest{}
	mi := &file_szdiagnostic_proto_msgTypes[8]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *ReinitializeRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ReinitializeRequest) ProtoMessage() {}

func (x *ReinitializeRequest) ProtoReflect() protoreflect.Message {
	mi := &file_szdiagnostic_proto_msgTypes[8]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ReinitializeRequest.ProtoReflect.Descriptor instead.
func (*ReinitializeRequest) Descriptor() ([]byte, []int) {
	return file_szdiagnostic_proto_rawDescGZIP(), []int{8}
}

func (x *ReinitializeRequest) GetConfigId() int64 {
	if x != nil {
		return x.ConfigId
	}
	return 0
}

type ReinitializeResponse struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *ReinitializeResponse) Reset() {
	*x = ReinitializeResponse{}
	mi := &file_szdiagnostic_proto_msgTypes[9]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *ReinitializeResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ReinitializeResponse) ProtoMessage() {}

func (x *ReinitializeResponse) ProtoReflect() protoreflect.Message {
	mi := &file_szdiagnostic_proto_msgTypes[9]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ReinitializeResponse.ProtoReflect.Descriptor instead.
func (*ReinitializeResponse) Descriptor() ([]byte, []int) {
	return file_szdiagnostic_proto_rawDescGZIP(), []int{9}
}

var File_szdiagnostic_proto protoreflect.FileDescriptor

const file_szdiagnostic_proto_rawDesc = "" +
	"\n" +
	"\x12szdiagnostic.proto\x12\fszdiagnostic\"I\n" +
	"!CheckRepositoryPerformanceRequest\x12$\n" +
	"\x0eseconds_to_run\x18\x01 \x01(\x05R\fsecondsToRun\"<\n" +
	"\"CheckRepositoryPerformanceResponse\x12\x16\n" +
	"\x06result\x18\x01 \x01(\tR\x06result\"\x1a\n" +
	"\x18GetRepositoryInfoRequest\"3\n" +
	"\x19GetRepositoryInfoResponse\x12\x16\n" +
	"\x06result\x18\x01 \x01(\tR\x06result\"2\n" +
	"\x11GetFeatureRequest\x12\x1d\n" +
	"\n" +
	"feature_id\x18\x01 \x01(\x03R\tfeatureId\",\n" +
	"\x12GetFeatureResponse\x12\x16\n" +
	"\x06result\x18\x01 \x01(\tR\x06result\"\x18\n" +
	"\x16PurgeRepositoryRequest\"\x19\n" +
	"\x17PurgeRepositoryResponse\"2\n" +
	"\x13ReinitializeRequest\x12\x1b\n" +
	"\tconfig_id\x18\x01 \x01(\x03R\bconfigId\"\x16\n" +
	"\x14ReinitializeResponse2\x88\x04\n" +
	"\fSzDiagnostic\x12\x81\x01\n" +
	"\x1aCheckRepositoryPerformance\x12/.szdiagnostic.CheckRepositoryPerformanceRequest\x1a0.szdiagnostic.CheckRepositoryPerformanceResponse\"\x00\x12Q\n" +
	"\n" +
	"GetFeature\x12\x1f.szdiagnostic.GetFeatureRequest\x1a .szdiagnostic.GetFeatureResponse\"\x00\x12f\n" +
	"\x11GetRepositoryInfo\x12&.szdiagnostic.GetRepositoryInfoRequest\x1a'.szdiagnostic.GetRepositoryInfoResponse\"\x00\x12`\n" +
	"\x0fPurgeRepository\x12$.szdiagnostic.PurgeRepositoryRequest\x1a%.szdiagnostic.PurgeRepositoryResponse\"\x00\x12W\n" +
	"\fReinitialize\x12!.szdiagnostic.ReinitializeRequest\x1a\".szdiagnostic.ReinitializeResponse\"\x00B`\n" +
	"\x14com.senzing.sdk.grpcB\x11SzDiagnosticProtoZ5github.com/senzing-garage/sz-sdk-go-grpc/szdiagnosticb\x06proto3"

var (
	file_szdiagnostic_proto_rawDescOnce sync.Once
	file_szdiagnostic_proto_rawDescData []byte
)

func file_szdiagnostic_proto_rawDescGZIP() []byte {
	file_szdiagnostic_proto_rawDescOnce.Do(func() {
		file_szdiagnostic_proto_rawDescData = protoimpl.X.CompressGZIP(unsafe.Slice(unsafe.StringData(file_szdiagnostic_proto_rawDesc), len(file_szdiagnostic_proto_rawDesc)))
	})
	return file_szdiagnostic_proto_rawDescData
}

var file_szdiagnostic_proto_msgTypes = make([]protoimpl.MessageInfo, 10)
var file_szdiagnostic_proto_goTypes = []any{
	(*CheckRepositoryPerformanceRequest)(nil),  // 0: szdiagnostic.CheckRepositoryPerformanceRequest
	(*CheckRepositoryPerformanceResponse)(nil), // 1: szdiagnostic.CheckRepositoryPerformanceResponse
	(*GetRepositoryInfoRequest)(nil),           // 2: szdiagnostic.GetRepositoryInfoRequest
	(*GetRepositoryInfoResponse)(nil),          // 3: szdiagnostic.GetRepositoryInfoResponse
	(*GetFeatureRequest)(nil),                  // 4: szdiagnostic.GetFeatureRequest
	(*GetFeatureResponse)(nil),                 // 5: szdiagnostic.GetFeatureResponse
	(*PurgeRepositoryRequest)(nil),             // 6: szdiagnostic.PurgeRepositoryRequest
	(*PurgeRepositoryResponse)(nil),            // 7: szdiagnostic.PurgeRepositoryResponse
	(*ReinitializeRequest)(nil),                // 8: szdiagnostic.ReinitializeRequest
	(*ReinitializeResponse)(nil),               // 9: szdiagnostic.ReinitializeResponse
}
var file_szdiagnostic_proto_depIdxs = []int32{
	0, // 0: szdiagnostic.SzDiagnostic.CheckRepositoryPerformance:input_type -> szdiagnostic.CheckRepositoryPerformanceRequest
	4, // 1: szdiagnostic.SzDiagnostic.GetFeature:input_type -> szdiagnostic.GetFeatureRequest
	2, // 2: szdiagnostic.SzDiagnostic.GetRepositoryInfo:input_type -> szdiagnostic.GetRepositoryInfoRequest
	6, // 3: szdiagnostic.SzDiagnostic.PurgeRepository:input_type -> szdiagnostic.PurgeRepositoryRequest
	8, // 4: szdiagnostic.SzDiagnostic.Reinitialize:input_type -> szdiagnostic.ReinitializeRequest
	1, // 5: szdiagnostic.SzDiagnostic.CheckRepositoryPerformance:output_type -> szdiagnostic.CheckRepositoryPerformanceResponse
	5, // 6: szdiagnostic.SzDiagnostic.GetFeature:output_type -> szdiagnostic.GetFeatureResponse
	3, // 7: szdiagnostic.SzDiagnostic.GetRepositoryInfo:output_type -> szdiagnostic.GetRepositoryInfoResponse
	7, // 8: szdiagnostic.SzDiagnostic.PurgeRepository:output_type -> szdiagnostic.PurgeRepositoryResponse
	9, // 9: szdiagnostic.SzDiagnostic.Reinitialize:output_type -> szdiagnostic.ReinitializeResponse
	5, // [5:10] is the sub-list for method output_type
	0, // [0:5] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_szdiagnostic_proto_init() }
func file_szdiagnostic_proto_init() {
	if File_szdiagnostic_proto != nil {
		return
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: unsafe.Slice(unsafe.StringData(file_szdiagnostic_proto_rawDesc), len(file_szdiagnostic_proto_rawDesc)),
			NumEnums:      0,
			NumMessages:   10,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_szdiagnostic_proto_goTypes,
		DependencyIndexes: file_szdiagnostic_proto_depIdxs,
		MessageInfos:      file_szdiagnostic_proto_msgTypes,
	}.Build()
	File_szdiagnostic_proto = out.File
	file_szdiagnostic_proto_goTypes = nil
	file_szdiagnostic_proto_depIdxs = nil
}
