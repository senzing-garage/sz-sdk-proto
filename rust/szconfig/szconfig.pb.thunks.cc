#include "szconfig.pb.h"
#include "google/protobuf/map.h"
#include "google/protobuf/repeated_field.h"
#include "google/protobuf/repeated_ptr_field.h"
#include "rust/cpp_kernel/serialized_data.h"
#include "rust/cpp_kernel/strings.h"
// szconfig.GetDataSourceRegistryRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfig_GetDataSourceRegistryRequest_new() { return new ::szconfig::GetDataSourceRegistryRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfig_GetDataSourceRegistryRequest_default_instance() {
  return &::szconfig::GetDataSourceRegistryRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfig_GetDataSourceRegistryRequest_config_definition_get(::szconfig::GetDataSourceRegistryRequest* msg) {
  absl::string_view val = msg->config_definition();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfig_GetDataSourceRegistryRequest_config_definition_set(::szconfig::GetDataSourceRegistryRequest* msg, std::string* s) {
  msg->set_config_definition(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szconfig.GetDataSourceRegistryResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfig_GetDataSourceRegistryResponse_new() { return new ::szconfig::GetDataSourceRegistryResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfig_GetDataSourceRegistryResponse_default_instance() {
  return &::szconfig::GetDataSourceRegistryResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfig_GetDataSourceRegistryResponse_result_get(::szconfig::GetDataSourceRegistryResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfig_GetDataSourceRegistryResponse_result_set(::szconfig::GetDataSourceRegistryResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szconfig.RegisterDataSourceRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfig_RegisterDataSourceRequest_new() { return new ::szconfig::RegisterDataSourceRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfig_RegisterDataSourceRequest_default_instance() {
  return &::szconfig::RegisterDataSourceRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfig_RegisterDataSourceRequest_config_definition_get(::szconfig::RegisterDataSourceRequest* msg) {
  absl::string_view val = msg->config_definition();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfig_RegisterDataSourceRequest_config_definition_set(::szconfig::RegisterDataSourceRequest* msg, std::string* s) {
  msg->set_config_definition(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfig_RegisterDataSourceRequest_data_source_code_get(::szconfig::RegisterDataSourceRequest* msg) {
  absl::string_view val = msg->data_source_code();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfig_RegisterDataSourceRequest_data_source_code_set(::szconfig::RegisterDataSourceRequest* msg, std::string* s) {
  msg->set_data_source_code(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szconfig.RegisterDataSourceResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfig_RegisterDataSourceResponse_new() { return new ::szconfig::RegisterDataSourceResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfig_RegisterDataSourceResponse_default_instance() {
  return &::szconfig::RegisterDataSourceResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfig_RegisterDataSourceResponse_result_get(::szconfig::RegisterDataSourceResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfig_RegisterDataSourceResponse_result_set(::szconfig::RegisterDataSourceResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfig_RegisterDataSourceResponse_config_definition_get(::szconfig::RegisterDataSourceResponse* msg) {
  absl::string_view val = msg->config_definition();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfig_RegisterDataSourceResponse_config_definition_set(::szconfig::RegisterDataSourceResponse* msg, std::string* s) {
  msg->set_config_definition(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szconfig.UnregisterDataSourceRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfig_UnregisterDataSourceRequest_new() { return new ::szconfig::UnregisterDataSourceRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfig_UnregisterDataSourceRequest_default_instance() {
  return &::szconfig::UnregisterDataSourceRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_config_definition_get(::szconfig::UnregisterDataSourceRequest* msg) {
  absl::string_view val = msg->config_definition();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_config_definition_set(::szconfig::UnregisterDataSourceRequest* msg, std::string* s) {
  msg->set_config_definition(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_data_source_code_get(::szconfig::UnregisterDataSourceRequest* msg) {
  absl::string_view val = msg->data_source_code();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_data_source_code_set(::szconfig::UnregisterDataSourceRequest* msg, std::string* s) {
  msg->set_data_source_code(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szconfig.UnregisterDataSourceResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfig_UnregisterDataSourceResponse_new() { return new ::szconfig::UnregisterDataSourceResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfig_UnregisterDataSourceResponse_default_instance() {
  return &::szconfig::UnregisterDataSourceResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_result_get(::szconfig::UnregisterDataSourceResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_result_set(::szconfig::UnregisterDataSourceResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_config_definition_get(::szconfig::UnregisterDataSourceResponse* msg) {
  absl::string_view val = msg->config_definition();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_config_definition_set(::szconfig::UnregisterDataSourceResponse* msg, std::string* s) {
  msg->set_config_definition(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szconfig.VerifyConfigRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfig_VerifyConfigRequest_new() { return new ::szconfig::VerifyConfigRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfig_VerifyConfigRequest_default_instance() {
  return &::szconfig::VerifyConfigRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfig_VerifyConfigRequest_config_definition_get(::szconfig::VerifyConfigRequest* msg) {
  absl::string_view val = msg->config_definition();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfig_VerifyConfigRequest_config_definition_set(::szconfig::VerifyConfigRequest* msg, std::string* s) {
  msg->set_config_definition(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szconfig.VerifyConfigResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfig_VerifyConfigResponse_new() { return new ::szconfig::VerifyConfigResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfig_VerifyConfigResponse_default_instance() {
  return &::szconfig::VerifyConfigResponse::default_instance();
}

bool proto2_rust_thunk_szconfig_VerifyConfigResponse_result_get(::szconfig::VerifyConfigResponse* msg) {
  return msg->result();
}
void proto2_rust_thunk_szconfig_VerifyConfigResponse_result_set(::szconfig::VerifyConfigResponse* msg, bool val) {
  msg->set_result(val);
}


}  // extern "C"
// clang-format on


