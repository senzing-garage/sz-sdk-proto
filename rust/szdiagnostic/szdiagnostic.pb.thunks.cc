#include "szdiagnostic.pb.h"
#include "google/protobuf/map.h"
#include "google/protobuf/repeated_field.h"
#include "google/protobuf/repeated_ptr_field.h"
#include "rust/cpp_kernel/serialized_data.h"
#include "rust/cpp_kernel/strings.h"
// szdiagnostic.CheckRepositoryPerformanceRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szdiagnostic_CheckRepositoryPerformanceRequest_new() { return new ::szdiagnostic::CheckRepositoryPerformanceRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szdiagnostic_CheckRepositoryPerformanceRequest_default_instance() {
  return &::szdiagnostic::CheckRepositoryPerformanceRequest::default_instance();
}

::int32_t proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceRequest_seconds_to_run_get(::szdiagnostic::CheckRepositoryPerformanceRequest* msg) {
  return msg->seconds_to_run();
}
void proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceRequest_seconds_to_run_set(::szdiagnostic::CheckRepositoryPerformanceRequest* msg, ::int32_t val) {
  msg->set_seconds_to_run(val);
}


}  // extern "C"
// clang-format on


// szdiagnostic.CheckRepositoryPerformanceResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szdiagnostic_CheckRepositoryPerformanceResponse_new() { return new ::szdiagnostic::CheckRepositoryPerformanceResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szdiagnostic_CheckRepositoryPerformanceResponse_default_instance() {
  return &::szdiagnostic::CheckRepositoryPerformanceResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceResponse_result_get(::szdiagnostic::CheckRepositoryPerformanceResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceResponse_result_set(::szdiagnostic::CheckRepositoryPerformanceResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szdiagnostic.GetRepositoryInfoRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szdiagnostic_GetRepositoryInfoRequest_new() { return new ::szdiagnostic::GetRepositoryInfoRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szdiagnostic_GetRepositoryInfoRequest_default_instance() {
  return &::szdiagnostic::GetRepositoryInfoRequest::default_instance();
}


}  // extern "C"
// clang-format on


// szdiagnostic.GetRepositoryInfoResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szdiagnostic_GetRepositoryInfoResponse_new() { return new ::szdiagnostic::GetRepositoryInfoResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szdiagnostic_GetRepositoryInfoResponse_default_instance() {
  return &::szdiagnostic::GetRepositoryInfoResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szdiagnostic_GetRepositoryInfoResponse_result_get(::szdiagnostic::GetRepositoryInfoResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szdiagnostic_GetRepositoryInfoResponse_result_set(::szdiagnostic::GetRepositoryInfoResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szdiagnostic.GetFeatureRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szdiagnostic_GetFeatureRequest_new() { return new ::szdiagnostic::GetFeatureRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szdiagnostic_GetFeatureRequest_default_instance() {
  return &::szdiagnostic::GetFeatureRequest::default_instance();
}

::int64_t proto2_rust_thunk_szdiagnostic_GetFeatureRequest_feature_id_get(::szdiagnostic::GetFeatureRequest* msg) {
  return msg->feature_id();
}
void proto2_rust_thunk_szdiagnostic_GetFeatureRequest_feature_id_set(::szdiagnostic::GetFeatureRequest* msg, ::int64_t val) {
  msg->set_feature_id(val);
}


}  // extern "C"
// clang-format on


// szdiagnostic.GetFeatureResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szdiagnostic_GetFeatureResponse_new() { return new ::szdiagnostic::GetFeatureResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szdiagnostic_GetFeatureResponse_default_instance() {
  return &::szdiagnostic::GetFeatureResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szdiagnostic_GetFeatureResponse_result_get(::szdiagnostic::GetFeatureResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szdiagnostic_GetFeatureResponse_result_set(::szdiagnostic::GetFeatureResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szdiagnostic.PurgeRepositoryRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szdiagnostic_PurgeRepositoryRequest_new() { return new ::szdiagnostic::PurgeRepositoryRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szdiagnostic_PurgeRepositoryRequest_default_instance() {
  return &::szdiagnostic::PurgeRepositoryRequest::default_instance();
}


}  // extern "C"
// clang-format on


// szdiagnostic.PurgeRepositoryResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szdiagnostic_PurgeRepositoryResponse_new() { return new ::szdiagnostic::PurgeRepositoryResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szdiagnostic_PurgeRepositoryResponse_default_instance() {
  return &::szdiagnostic::PurgeRepositoryResponse::default_instance();
}


}  // extern "C"
// clang-format on


// szdiagnostic.ReinitializeRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szdiagnostic_ReinitializeRequest_new() { return new ::szdiagnostic::ReinitializeRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szdiagnostic_ReinitializeRequest_default_instance() {
  return &::szdiagnostic::ReinitializeRequest::default_instance();
}

::int64_t proto2_rust_thunk_szdiagnostic_ReinitializeRequest_config_id_get(::szdiagnostic::ReinitializeRequest* msg) {
  return msg->config_id();
}
void proto2_rust_thunk_szdiagnostic_ReinitializeRequest_config_id_set(::szdiagnostic::ReinitializeRequest* msg, ::int64_t val) {
  msg->set_config_id(val);
}


}  // extern "C"
// clang-format on


// szdiagnostic.ReinitializeResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szdiagnostic_ReinitializeResponse_new() { return new ::szdiagnostic::ReinitializeResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szdiagnostic_ReinitializeResponse_default_instance() {
  return &::szdiagnostic::ReinitializeResponse::default_instance();
}


}  // extern "C"
// clang-format on


