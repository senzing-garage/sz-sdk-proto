#include "szconfigmanager.pb.h"
#include "google/protobuf/map.h"
#include "google/protobuf/repeated_field.h"
#include "google/protobuf/repeated_ptr_field.h"
#include "rust/cpp_kernel/serialized_data.h"
#include "rust/cpp_kernel/strings.h"
// szconfigmanager.GetConfigRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_GetConfigRequest_new() { return new ::szconfigmanager::GetConfigRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_GetConfigRequest_default_instance() {
  return &::szconfigmanager::GetConfigRequest::default_instance();
}

::int64_t proto2_rust_thunk_szconfigmanager_GetConfigRequest_config_id_get(::szconfigmanager::GetConfigRequest* msg) {
  return msg->config_id();
}
void proto2_rust_thunk_szconfigmanager_GetConfigRequest_config_id_set(::szconfigmanager::GetConfigRequest* msg, ::int64_t val) {
  msg->set_config_id(val);
}


}  // extern "C"
// clang-format on


// szconfigmanager.GetConfigResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_GetConfigResponse_new() { return new ::szconfigmanager::GetConfigResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_GetConfigResponse_default_instance() {
  return &::szconfigmanager::GetConfigResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfigmanager_GetConfigResponse_result_get(::szconfigmanager::GetConfigResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfigmanager_GetConfigResponse_result_set(::szconfigmanager::GetConfigResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szconfigmanager.GetConfigRegistryRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_GetConfigRegistryRequest_new() { return new ::szconfigmanager::GetConfigRegistryRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_GetConfigRegistryRequest_default_instance() {
  return &::szconfigmanager::GetConfigRegistryRequest::default_instance();
}


}  // extern "C"
// clang-format on


// szconfigmanager.GetConfigRegistryResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_GetConfigRegistryResponse_new() { return new ::szconfigmanager::GetConfigRegistryResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_GetConfigRegistryResponse_default_instance() {
  return &::szconfigmanager::GetConfigRegistryResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfigmanager_GetConfigRegistryResponse_result_get(::szconfigmanager::GetConfigRegistryResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfigmanager_GetConfigRegistryResponse_result_set(::szconfigmanager::GetConfigRegistryResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szconfigmanager.GetDefaultConfigIdRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_GetDefaultConfigIdRequest_new() { return new ::szconfigmanager::GetDefaultConfigIdRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_GetDefaultConfigIdRequest_default_instance() {
  return &::szconfigmanager::GetDefaultConfigIdRequest::default_instance();
}


}  // extern "C"
// clang-format on


// szconfigmanager.GetDefaultConfigIdResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_GetDefaultConfigIdResponse_new() { return new ::szconfigmanager::GetDefaultConfigIdResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_GetDefaultConfigIdResponse_default_instance() {
  return &::szconfigmanager::GetDefaultConfigIdResponse::default_instance();
}

::int64_t proto2_rust_thunk_szconfigmanager_GetDefaultConfigIdResponse_result_get(::szconfigmanager::GetDefaultConfigIdResponse* msg) {
  return msg->result();
}
void proto2_rust_thunk_szconfigmanager_GetDefaultConfigIdResponse_result_set(::szconfigmanager::GetDefaultConfigIdResponse* msg, ::int64_t val) {
  msg->set_result(val);
}


}  // extern "C"
// clang-format on


// szconfigmanager.GetTemplateConfigRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_GetTemplateConfigRequest_new() { return new ::szconfigmanager::GetTemplateConfigRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_GetTemplateConfigRequest_default_instance() {
  return &::szconfigmanager::GetTemplateConfigRequest::default_instance();
}


}  // extern "C"
// clang-format on


// szconfigmanager.GetTemplateConfigResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_GetTemplateConfigResponse_new() { return new ::szconfigmanager::GetTemplateConfigResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_GetTemplateConfigResponse_default_instance() {
  return &::szconfigmanager::GetTemplateConfigResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfigmanager_GetTemplateConfigResponse_result_get(::szconfigmanager::GetTemplateConfigResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfigmanager_GetTemplateConfigResponse_result_set(::szconfigmanager::GetTemplateConfigResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szconfigmanager.RegisterConfigRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_RegisterConfigRequest_new() { return new ::szconfigmanager::RegisterConfigRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_RegisterConfigRequest_default_instance() {
  return &::szconfigmanager::RegisterConfigRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_definition_get(::szconfigmanager::RegisterConfigRequest* msg) {
  absl::string_view val = msg->config_definition();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_definition_set(::szconfigmanager::RegisterConfigRequest* msg, std::string* s) {
  msg->set_config_definition(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_comment_get(::szconfigmanager::RegisterConfigRequest* msg) {
  absl::string_view val = msg->config_comment();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_comment_set(::szconfigmanager::RegisterConfigRequest* msg, std::string* s) {
  msg->set_config_comment(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szconfigmanager.RegisterConfigResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_RegisterConfigResponse_new() { return new ::szconfigmanager::RegisterConfigResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_RegisterConfigResponse_default_instance() {
  return &::szconfigmanager::RegisterConfigResponse::default_instance();
}

::int64_t proto2_rust_thunk_szconfigmanager_RegisterConfigResponse_result_get(::szconfigmanager::RegisterConfigResponse* msg) {
  return msg->result();
}
void proto2_rust_thunk_szconfigmanager_RegisterConfigResponse_result_set(::szconfigmanager::RegisterConfigResponse* msg, ::int64_t val) {
  msg->set_result(val);
}


}  // extern "C"
// clang-format on


// szconfigmanager.ReplaceDefaultConfigIdRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_ReplaceDefaultConfigIdRequest_new() { return new ::szconfigmanager::ReplaceDefaultConfigIdRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_ReplaceDefaultConfigIdRequest_default_instance() {
  return &::szconfigmanager::ReplaceDefaultConfigIdRequest::default_instance();
}

::int64_t proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_current_default_config_id_get(::szconfigmanager::ReplaceDefaultConfigIdRequest* msg) {
  return msg->current_default_config_id();
}
void proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_current_default_config_id_set(::szconfigmanager::ReplaceDefaultConfigIdRequest* msg, ::int64_t val) {
  msg->set_current_default_config_id(val);
}

::int64_t proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_new_default_config_id_get(::szconfigmanager::ReplaceDefaultConfigIdRequest* msg) {
  return msg->new_default_config_id();
}
void proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_new_default_config_id_set(::szconfigmanager::ReplaceDefaultConfigIdRequest* msg, ::int64_t val) {
  msg->set_new_default_config_id(val);
}


}  // extern "C"
// clang-format on


// szconfigmanager.ReplaceDefaultConfigIdResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_ReplaceDefaultConfigIdResponse_new() { return new ::szconfigmanager::ReplaceDefaultConfigIdResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_ReplaceDefaultConfigIdResponse_default_instance() {
  return &::szconfigmanager::ReplaceDefaultConfigIdResponse::default_instance();
}


}  // extern "C"
// clang-format on


// szconfigmanager.SetDefaultConfigRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigRequest_new() { return new ::szconfigmanager::SetDefaultConfigRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigRequest_default_instance() {
  return &::szconfigmanager::SetDefaultConfigRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_definition_get(::szconfigmanager::SetDefaultConfigRequest* msg) {
  absl::string_view val = msg->config_definition();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_definition_set(::szconfigmanager::SetDefaultConfigRequest* msg, std::string* s) {
  msg->set_config_definition(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_comment_get(::szconfigmanager::SetDefaultConfigRequest* msg) {
  absl::string_view val = msg->config_comment();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_comment_set(::szconfigmanager::SetDefaultConfigRequest* msg, std::string* s) {
  msg->set_config_comment(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szconfigmanager.SetDefaultConfigResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigResponse_new() { return new ::szconfigmanager::SetDefaultConfigResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigResponse_default_instance() {
  return &::szconfigmanager::SetDefaultConfigResponse::default_instance();
}

::int64_t proto2_rust_thunk_szconfigmanager_SetDefaultConfigResponse_result_get(::szconfigmanager::SetDefaultConfigResponse* msg) {
  return msg->result();
}
void proto2_rust_thunk_szconfigmanager_SetDefaultConfigResponse_result_set(::szconfigmanager::SetDefaultConfigResponse* msg, ::int64_t val) {
  msg->set_result(val);
}


}  // extern "C"
// clang-format on


// szconfigmanager.SetDefaultConfigIdRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigIdRequest_new() { return new ::szconfigmanager::SetDefaultConfigIdRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigIdRequest_default_instance() {
  return &::szconfigmanager::SetDefaultConfigIdRequest::default_instance();
}

::int64_t proto2_rust_thunk_szconfigmanager_SetDefaultConfigIdRequest_config_id_get(::szconfigmanager::SetDefaultConfigIdRequest* msg) {
  return msg->config_id();
}
void proto2_rust_thunk_szconfigmanager_SetDefaultConfigIdRequest_config_id_set(::szconfigmanager::SetDefaultConfigIdRequest* msg, ::int64_t val) {
  msg->set_config_id(val);
}


}  // extern "C"
// clang-format on


// szconfigmanager.SetDefaultConfigIdResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigIdResponse_new() { return new ::szconfigmanager::SetDefaultConfigIdResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigIdResponse_default_instance() {
  return &::szconfigmanager::SetDefaultConfigIdResponse::default_instance();
}


}  // extern "C"
// clang-format on


