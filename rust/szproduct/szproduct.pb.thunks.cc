#include "szproduct.pb.h"
#include "google/protobuf/map.h"
#include "google/protobuf/repeated_field.h"
#include "google/protobuf/repeated_ptr_field.h"
#include "rust/cpp_kernel/serialized_data.h"
#include "rust/cpp_kernel/strings.h"
// szproduct.GetLicenseRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szproduct_GetLicenseRequest_new() { return new ::szproduct::GetLicenseRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szproduct_GetLicenseRequest_default_instance() {
  return &::szproduct::GetLicenseRequest::default_instance();
}


}  // extern "C"
// clang-format on


// szproduct.GetLicenseResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szproduct_GetLicenseResponse_new() { return new ::szproduct::GetLicenseResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szproduct_GetLicenseResponse_default_instance() {
  return &::szproduct::GetLicenseResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szproduct_GetLicenseResponse_result_get(::szproduct::GetLicenseResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szproduct_GetLicenseResponse_result_set(::szproduct::GetLicenseResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szproduct.GetVersionRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szproduct_GetVersionRequest_new() { return new ::szproduct::GetVersionRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szproduct_GetVersionRequest_default_instance() {
  return &::szproduct::GetVersionRequest::default_instance();
}


}  // extern "C"
// clang-format on


// szproduct.GetVersionResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szproduct_GetVersionResponse_new() { return new ::szproduct::GetVersionResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szproduct_GetVersionResponse_default_instance() {
  return &::szproduct::GetVersionResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szproduct_GetVersionResponse_result_get(::szproduct::GetVersionResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szproduct_GetVersionResponse_result_set(::szproduct::GetVersionResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


