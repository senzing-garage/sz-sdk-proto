#include "szengine.pb.h"
#include "google/protobuf/map.h"
#include "google/protobuf/repeated_field.h"
#include "google/protobuf/repeated_ptr_field.h"
#include "rust/cpp_kernel/serialized_data.h"
#include "rust/cpp_kernel/strings.h"
// szengine.AddRecordRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_AddRecordRequest_new() { return new ::szengine::AddRecordRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_AddRecordRequest_default_instance() {
  return &::szengine::AddRecordRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_AddRecordRequest_data_source_code_get(::szengine::AddRecordRequest* msg) {
  absl::string_view val = msg->data_source_code();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_AddRecordRequest_data_source_code_set(::szengine::AddRecordRequest* msg, std::string* s) {
  msg->set_data_source_code(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_AddRecordRequest_record_id_get(::szengine::AddRecordRequest* msg) {
  absl::string_view val = msg->record_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_AddRecordRequest_record_id_set(::szengine::AddRecordRequest* msg, std::string* s) {
  msg->set_record_id(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_AddRecordRequest_record_definition_get(::szengine::AddRecordRequest* msg) {
  absl::string_view val = msg->record_definition();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_AddRecordRequest_record_definition_set(::szengine::AddRecordRequest* msg, std::string* s) {
  msg->set_record_definition(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_AddRecordRequest_flags_get(::szengine::AddRecordRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_AddRecordRequest_flags_set(::szengine::AddRecordRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.AddRecordResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_AddRecordResponse_new() { return new ::szengine::AddRecordResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_AddRecordResponse_default_instance() {
  return &::szengine::AddRecordResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_AddRecordResponse_result_get(::szengine::AddRecordResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_AddRecordResponse_result_set(::szengine::AddRecordResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.CloseExportReportRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_CloseExportReportRequest_new() { return new ::szengine::CloseExportReportRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_CloseExportReportRequest_default_instance() {
  return &::szengine::CloseExportReportRequest::default_instance();
}

::int64_t proto2_rust_thunk_szengine_CloseExportReportRequest_export_handle_get(::szengine::CloseExportReportRequest* msg) {
  return msg->export_handle();
}
void proto2_rust_thunk_szengine_CloseExportReportRequest_export_handle_set(::szengine::CloseExportReportRequest* msg, ::int64_t val) {
  msg->set_export_handle(val);
}


}  // extern "C"
// clang-format on


// szengine.CloseExportReportResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_CloseExportReportResponse_new() { return new ::szengine::CloseExportReportResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_CloseExportReportResponse_default_instance() {
  return &::szengine::CloseExportReportResponse::default_instance();
}


}  // extern "C"
// clang-format on


// szengine.CountRedoRecordsRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_CountRedoRecordsRequest_new() { return new ::szengine::CountRedoRecordsRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_CountRedoRecordsRequest_default_instance() {
  return &::szengine::CountRedoRecordsRequest::default_instance();
}


}  // extern "C"
// clang-format on


// szengine.CountRedoRecordsResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_CountRedoRecordsResponse_new() { return new ::szengine::CountRedoRecordsResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_CountRedoRecordsResponse_default_instance() {
  return &::szengine::CountRedoRecordsResponse::default_instance();
}

::int64_t proto2_rust_thunk_szengine_CountRedoRecordsResponse_result_get(::szengine::CountRedoRecordsResponse* msg) {
  return msg->result();
}
void proto2_rust_thunk_szengine_CountRedoRecordsResponse_result_set(::szengine::CountRedoRecordsResponse* msg, ::int64_t val) {
  msg->set_result(val);
}


}  // extern "C"
// clang-format on


// szengine.DeleteRecordRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_DeleteRecordRequest_new() { return new ::szengine::DeleteRecordRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_DeleteRecordRequest_default_instance() {
  return &::szengine::DeleteRecordRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_DeleteRecordRequest_data_source_code_get(::szengine::DeleteRecordRequest* msg) {
  absl::string_view val = msg->data_source_code();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_DeleteRecordRequest_data_source_code_set(::szengine::DeleteRecordRequest* msg, std::string* s) {
  msg->set_data_source_code(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_DeleteRecordRequest_record_id_get(::szengine::DeleteRecordRequest* msg) {
  absl::string_view val = msg->record_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_DeleteRecordRequest_record_id_set(::szengine::DeleteRecordRequest* msg, std::string* s) {
  msg->set_record_id(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_DeleteRecordRequest_flags_get(::szengine::DeleteRecordRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_DeleteRecordRequest_flags_set(::szengine::DeleteRecordRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.DeleteRecordResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_DeleteRecordResponse_new() { return new ::szengine::DeleteRecordResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_DeleteRecordResponse_default_instance() {
  return &::szengine::DeleteRecordResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_DeleteRecordResponse_result_get(::szengine::DeleteRecordResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_DeleteRecordResponse_result_set(::szengine::DeleteRecordResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.ExportCsvEntityReportRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_ExportCsvEntityReportRequest_new() { return new ::szengine::ExportCsvEntityReportRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_ExportCsvEntityReportRequest_default_instance() {
  return &::szengine::ExportCsvEntityReportRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_ExportCsvEntityReportRequest_csv_column_list_get(::szengine::ExportCsvEntityReportRequest* msg) {
  absl::string_view val = msg->csv_column_list();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_ExportCsvEntityReportRequest_csv_column_list_set(::szengine::ExportCsvEntityReportRequest* msg, std::string* s) {
  msg->set_csv_column_list(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_ExportCsvEntityReportRequest_flags_get(::szengine::ExportCsvEntityReportRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_ExportCsvEntityReportRequest_flags_set(::szengine::ExportCsvEntityReportRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.ExportCsvEntityReportResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_ExportCsvEntityReportResponse_new() { return new ::szengine::ExportCsvEntityReportResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_ExportCsvEntityReportResponse_default_instance() {
  return &::szengine::ExportCsvEntityReportResponse::default_instance();
}

::int64_t proto2_rust_thunk_szengine_ExportCsvEntityReportResponse_result_get(::szengine::ExportCsvEntityReportResponse* msg) {
  return msg->result();
}
void proto2_rust_thunk_szengine_ExportCsvEntityReportResponse_result_set(::szengine::ExportCsvEntityReportResponse* msg, ::int64_t val) {
  msg->set_result(val);
}


}  // extern "C"
// clang-format on


// szengine.ExportJsonEntityReportRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_ExportJsonEntityReportRequest_new() { return new ::szengine::ExportJsonEntityReportRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_ExportJsonEntityReportRequest_default_instance() {
  return &::szengine::ExportJsonEntityReportRequest::default_instance();
}

::int64_t proto2_rust_thunk_szengine_ExportJsonEntityReportRequest_flags_get(::szengine::ExportJsonEntityReportRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_ExportJsonEntityReportRequest_flags_set(::szengine::ExportJsonEntityReportRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.ExportJsonEntityReportResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_ExportJsonEntityReportResponse_new() { return new ::szengine::ExportJsonEntityReportResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_ExportJsonEntityReportResponse_default_instance() {
  return &::szengine::ExportJsonEntityReportResponse::default_instance();
}

::int64_t proto2_rust_thunk_szengine_ExportJsonEntityReportResponse_result_get(::szengine::ExportJsonEntityReportResponse* msg) {
  return msg->result();
}
void proto2_rust_thunk_szengine_ExportJsonEntityReportResponse_result_set(::szengine::ExportJsonEntityReportResponse* msg, ::int64_t val) {
  msg->set_result(val);
}


}  // extern "C"
// clang-format on


// szengine.FetchNextRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_FetchNextRequest_new() { return new ::szengine::FetchNextRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_FetchNextRequest_default_instance() {
  return &::szengine::FetchNextRequest::default_instance();
}

::int64_t proto2_rust_thunk_szengine_FetchNextRequest_export_handle_get(::szengine::FetchNextRequest* msg) {
  return msg->export_handle();
}
void proto2_rust_thunk_szengine_FetchNextRequest_export_handle_set(::szengine::FetchNextRequest* msg, ::int64_t val) {
  msg->set_export_handle(val);
}


}  // extern "C"
// clang-format on


// szengine.FetchNextResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_FetchNextResponse_new() { return new ::szengine::FetchNextResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_FetchNextResponse_default_instance() {
  return &::szengine::FetchNextResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FetchNextResponse_result_get(::szengine::FetchNextResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FetchNextResponse_result_set(::szengine::FetchNextResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.FindInterestingEntitiesByEntityIdRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_FindInterestingEntitiesByEntityIdRequest_new() { return new ::szengine::FindInterestingEntitiesByEntityIdRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_FindInterestingEntitiesByEntityIdRequest_default_instance() {
  return &::szengine::FindInterestingEntitiesByEntityIdRequest::default_instance();
}

::int64_t proto2_rust_thunk_szengine_FindInterestingEntitiesByEntityIdRequest_entity_id_get(::szengine::FindInterestingEntitiesByEntityIdRequest* msg) {
  return msg->entity_id();
}
void proto2_rust_thunk_szengine_FindInterestingEntitiesByEntityIdRequest_entity_id_set(::szengine::FindInterestingEntitiesByEntityIdRequest* msg, ::int64_t val) {
  msg->set_entity_id(val);
}

::int64_t proto2_rust_thunk_szengine_FindInterestingEntitiesByEntityIdRequest_flags_get(::szengine::FindInterestingEntitiesByEntityIdRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_FindInterestingEntitiesByEntityIdRequest_flags_set(::szengine::FindInterestingEntitiesByEntityIdRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.FindInterestingEntitiesByEntityIdResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_FindInterestingEntitiesByEntityIdResponse_new() { return new ::szengine::FindInterestingEntitiesByEntityIdResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_FindInterestingEntitiesByEntityIdResponse_default_instance() {
  return &::szengine::FindInterestingEntitiesByEntityIdResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindInterestingEntitiesByEntityIdResponse_result_get(::szengine::FindInterestingEntitiesByEntityIdResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindInterestingEntitiesByEntityIdResponse_result_set(::szengine::FindInterestingEntitiesByEntityIdResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.FindInterestingEntitiesByRecordIdRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_FindInterestingEntitiesByRecordIdRequest_new() { return new ::szengine::FindInterestingEntitiesByRecordIdRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_FindInterestingEntitiesByRecordIdRequest_default_instance() {
  return &::szengine::FindInterestingEntitiesByRecordIdRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindInterestingEntitiesByRecordIdRequest_data_source_code_get(::szengine::FindInterestingEntitiesByRecordIdRequest* msg) {
  absl::string_view val = msg->data_source_code();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindInterestingEntitiesByRecordIdRequest_data_source_code_set(::szengine::FindInterestingEntitiesByRecordIdRequest* msg, std::string* s) {
  msg->set_data_source_code(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindInterestingEntitiesByRecordIdRequest_record_id_get(::szengine::FindInterestingEntitiesByRecordIdRequest* msg) {
  absl::string_view val = msg->record_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindInterestingEntitiesByRecordIdRequest_record_id_set(::szengine::FindInterestingEntitiesByRecordIdRequest* msg, std::string* s) {
  msg->set_record_id(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_FindInterestingEntitiesByRecordIdRequest_flags_get(::szengine::FindInterestingEntitiesByRecordIdRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_FindInterestingEntitiesByRecordIdRequest_flags_set(::szengine::FindInterestingEntitiesByRecordIdRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.FindInterestingEntitiesByRecordIdResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_FindInterestingEntitiesByRecordIdResponse_new() { return new ::szengine::FindInterestingEntitiesByRecordIdResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_FindInterestingEntitiesByRecordIdResponse_default_instance() {
  return &::szengine::FindInterestingEntitiesByRecordIdResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindInterestingEntitiesByRecordIdResponse_result_get(::szengine::FindInterestingEntitiesByRecordIdResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindInterestingEntitiesByRecordIdResponse_result_set(::szengine::FindInterestingEntitiesByRecordIdResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.FindNetworkByEntityIdRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_FindNetworkByEntityIdRequest_new() { return new ::szengine::FindNetworkByEntityIdRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_FindNetworkByEntityIdRequest_default_instance() {
  return &::szengine::FindNetworkByEntityIdRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindNetworkByEntityIdRequest_entity_ids_get(::szengine::FindNetworkByEntityIdRequest* msg) {
  absl::string_view val = msg->entity_ids();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindNetworkByEntityIdRequest_entity_ids_set(::szengine::FindNetworkByEntityIdRequest* msg, std::string* s) {
  msg->set_entity_ids(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_FindNetworkByEntityIdRequest_max_degrees_get(::szengine::FindNetworkByEntityIdRequest* msg) {
  return msg->max_degrees();
}
void proto2_rust_thunk_szengine_FindNetworkByEntityIdRequest_max_degrees_set(::szengine::FindNetworkByEntityIdRequest* msg, ::int64_t val) {
  msg->set_max_degrees(val);
}

::int64_t proto2_rust_thunk_szengine_FindNetworkByEntityIdRequest_build_out_degrees_get(::szengine::FindNetworkByEntityIdRequest* msg) {
  return msg->build_out_degrees();
}
void proto2_rust_thunk_szengine_FindNetworkByEntityIdRequest_build_out_degrees_set(::szengine::FindNetworkByEntityIdRequest* msg, ::int64_t val) {
  msg->set_build_out_degrees(val);
}

::int64_t proto2_rust_thunk_szengine_FindNetworkByEntityIdRequest_build_out_max_entities_get(::szengine::FindNetworkByEntityIdRequest* msg) {
  return msg->build_out_max_entities();
}
void proto2_rust_thunk_szengine_FindNetworkByEntityIdRequest_build_out_max_entities_set(::szengine::FindNetworkByEntityIdRequest* msg, ::int64_t val) {
  msg->set_build_out_max_entities(val);
}

::int64_t proto2_rust_thunk_szengine_FindNetworkByEntityIdRequest_flags_get(::szengine::FindNetworkByEntityIdRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_FindNetworkByEntityIdRequest_flags_set(::szengine::FindNetworkByEntityIdRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.FindNetworkByEntityIdResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_FindNetworkByEntityIdResponse_new() { return new ::szengine::FindNetworkByEntityIdResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_FindNetworkByEntityIdResponse_default_instance() {
  return &::szengine::FindNetworkByEntityIdResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindNetworkByEntityIdResponse_result_get(::szengine::FindNetworkByEntityIdResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindNetworkByEntityIdResponse_result_set(::szengine::FindNetworkByEntityIdResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.FindNetworkByRecordIdRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_FindNetworkByRecordIdRequest_new() { return new ::szengine::FindNetworkByRecordIdRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_FindNetworkByRecordIdRequest_default_instance() {
  return &::szengine::FindNetworkByRecordIdRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindNetworkByRecordIdRequest_record_keys_get(::szengine::FindNetworkByRecordIdRequest* msg) {
  absl::string_view val = msg->record_keys();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindNetworkByRecordIdRequest_record_keys_set(::szengine::FindNetworkByRecordIdRequest* msg, std::string* s) {
  msg->set_record_keys(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_FindNetworkByRecordIdRequest_max_degrees_get(::szengine::FindNetworkByRecordIdRequest* msg) {
  return msg->max_degrees();
}
void proto2_rust_thunk_szengine_FindNetworkByRecordIdRequest_max_degrees_set(::szengine::FindNetworkByRecordIdRequest* msg, ::int64_t val) {
  msg->set_max_degrees(val);
}

::int64_t proto2_rust_thunk_szengine_FindNetworkByRecordIdRequest_build_out_degrees_get(::szengine::FindNetworkByRecordIdRequest* msg) {
  return msg->build_out_degrees();
}
void proto2_rust_thunk_szengine_FindNetworkByRecordIdRequest_build_out_degrees_set(::szengine::FindNetworkByRecordIdRequest* msg, ::int64_t val) {
  msg->set_build_out_degrees(val);
}

::int64_t proto2_rust_thunk_szengine_FindNetworkByRecordIdRequest_build_out_max_entities_get(::szengine::FindNetworkByRecordIdRequest* msg) {
  return msg->build_out_max_entities();
}
void proto2_rust_thunk_szengine_FindNetworkByRecordIdRequest_build_out_max_entities_set(::szengine::FindNetworkByRecordIdRequest* msg, ::int64_t val) {
  msg->set_build_out_max_entities(val);
}

::int64_t proto2_rust_thunk_szengine_FindNetworkByRecordIdRequest_flags_get(::szengine::FindNetworkByRecordIdRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_FindNetworkByRecordIdRequest_flags_set(::szengine::FindNetworkByRecordIdRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.FindNetworkByRecordIdResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_FindNetworkByRecordIdResponse_new() { return new ::szengine::FindNetworkByRecordIdResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_FindNetworkByRecordIdResponse_default_instance() {
  return &::szengine::FindNetworkByRecordIdResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindNetworkByRecordIdResponse_result_get(::szengine::FindNetworkByRecordIdResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindNetworkByRecordIdResponse_result_set(::szengine::FindNetworkByRecordIdResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.FindPathByEntityIdRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_FindPathByEntityIdRequest_new() { return new ::szengine::FindPathByEntityIdRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_FindPathByEntityIdRequest_default_instance() {
  return &::szengine::FindPathByEntityIdRequest::default_instance();
}

::int64_t proto2_rust_thunk_szengine_FindPathByEntityIdRequest_start_entity_id_get(::szengine::FindPathByEntityIdRequest* msg) {
  return msg->start_entity_id();
}
void proto2_rust_thunk_szengine_FindPathByEntityIdRequest_start_entity_id_set(::szengine::FindPathByEntityIdRequest* msg, ::int64_t val) {
  msg->set_start_entity_id(val);
}

::int64_t proto2_rust_thunk_szengine_FindPathByEntityIdRequest_end_entity_id_get(::szengine::FindPathByEntityIdRequest* msg) {
  return msg->end_entity_id();
}
void proto2_rust_thunk_szengine_FindPathByEntityIdRequest_end_entity_id_set(::szengine::FindPathByEntityIdRequest* msg, ::int64_t val) {
  msg->set_end_entity_id(val);
}

::int64_t proto2_rust_thunk_szengine_FindPathByEntityIdRequest_max_degrees_get(::szengine::FindPathByEntityIdRequest* msg) {
  return msg->max_degrees();
}
void proto2_rust_thunk_szengine_FindPathByEntityIdRequest_max_degrees_set(::szengine::FindPathByEntityIdRequest* msg, ::int64_t val) {
  msg->set_max_degrees(val);
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindPathByEntityIdRequest_avoid_entity_ids_get(::szengine::FindPathByEntityIdRequest* msg) {
  absl::string_view val = msg->avoid_entity_ids();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindPathByEntityIdRequest_avoid_entity_ids_set(::szengine::FindPathByEntityIdRequest* msg, std::string* s) {
  msg->set_avoid_entity_ids(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindPathByEntityIdRequest_required_data_sources_get(::szengine::FindPathByEntityIdRequest* msg) {
  absl::string_view val = msg->required_data_sources();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindPathByEntityIdRequest_required_data_sources_set(::szengine::FindPathByEntityIdRequest* msg, std::string* s) {
  msg->set_required_data_sources(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_FindPathByEntityIdRequest_flags_get(::szengine::FindPathByEntityIdRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_FindPathByEntityIdRequest_flags_set(::szengine::FindPathByEntityIdRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.FindPathByEntityIdResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_FindPathByEntityIdResponse_new() { return new ::szengine::FindPathByEntityIdResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_FindPathByEntityIdResponse_default_instance() {
  return &::szengine::FindPathByEntityIdResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindPathByEntityIdResponse_result_get(::szengine::FindPathByEntityIdResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindPathByEntityIdResponse_result_set(::szengine::FindPathByEntityIdResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.FindPathByRecordIdRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_FindPathByRecordIdRequest_new() { return new ::szengine::FindPathByRecordIdRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_FindPathByRecordIdRequest_default_instance() {
  return &::szengine::FindPathByRecordIdRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindPathByRecordIdRequest_start_data_source_code_get(::szengine::FindPathByRecordIdRequest* msg) {
  absl::string_view val = msg->start_data_source_code();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindPathByRecordIdRequest_start_data_source_code_set(::szengine::FindPathByRecordIdRequest* msg, std::string* s) {
  msg->set_start_data_source_code(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindPathByRecordIdRequest_start_record_id_get(::szengine::FindPathByRecordIdRequest* msg) {
  absl::string_view val = msg->start_record_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindPathByRecordIdRequest_start_record_id_set(::szengine::FindPathByRecordIdRequest* msg, std::string* s) {
  msg->set_start_record_id(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindPathByRecordIdRequest_end_data_source_code_get(::szengine::FindPathByRecordIdRequest* msg) {
  absl::string_view val = msg->end_data_source_code();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindPathByRecordIdRequest_end_data_source_code_set(::szengine::FindPathByRecordIdRequest* msg, std::string* s) {
  msg->set_end_data_source_code(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindPathByRecordIdRequest_end_record_id_get(::szengine::FindPathByRecordIdRequest* msg) {
  absl::string_view val = msg->end_record_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindPathByRecordIdRequest_end_record_id_set(::szengine::FindPathByRecordIdRequest* msg, std::string* s) {
  msg->set_end_record_id(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_FindPathByRecordIdRequest_max_degrees_get(::szengine::FindPathByRecordIdRequest* msg) {
  return msg->max_degrees();
}
void proto2_rust_thunk_szengine_FindPathByRecordIdRequest_max_degrees_set(::szengine::FindPathByRecordIdRequest* msg, ::int64_t val) {
  msg->set_max_degrees(val);
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindPathByRecordIdRequest_avoid_record_keys_get(::szengine::FindPathByRecordIdRequest* msg) {
  absl::string_view val = msg->avoid_record_keys();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindPathByRecordIdRequest_avoid_record_keys_set(::szengine::FindPathByRecordIdRequest* msg, std::string* s) {
  msg->set_avoid_record_keys(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindPathByRecordIdRequest_required_data_sources_get(::szengine::FindPathByRecordIdRequest* msg) {
  absl::string_view val = msg->required_data_sources();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindPathByRecordIdRequest_required_data_sources_set(::szengine::FindPathByRecordIdRequest* msg, std::string* s) {
  msg->set_required_data_sources(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_FindPathByRecordIdRequest_flags_get(::szengine::FindPathByRecordIdRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_FindPathByRecordIdRequest_flags_set(::szengine::FindPathByRecordIdRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.FindPathByRecordIdResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_FindPathByRecordIdResponse_new() { return new ::szengine::FindPathByRecordIdResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_FindPathByRecordIdResponse_default_instance() {
  return &::szengine::FindPathByRecordIdResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_FindPathByRecordIdResponse_result_get(::szengine::FindPathByRecordIdResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_FindPathByRecordIdResponse_result_set(::szengine::FindPathByRecordIdResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.GetActiveConfigIdRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetActiveConfigIdRequest_new() { return new ::szengine::GetActiveConfigIdRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetActiveConfigIdRequest_default_instance() {
  return &::szengine::GetActiveConfigIdRequest::default_instance();
}


}  // extern "C"
// clang-format on


// szengine.GetActiveConfigIdResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetActiveConfigIdResponse_new() { return new ::szengine::GetActiveConfigIdResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetActiveConfigIdResponse_default_instance() {
  return &::szengine::GetActiveConfigIdResponse::default_instance();
}

::int64_t proto2_rust_thunk_szengine_GetActiveConfigIdResponse_result_get(::szengine::GetActiveConfigIdResponse* msg) {
  return msg->result();
}
void proto2_rust_thunk_szengine_GetActiveConfigIdResponse_result_set(::szengine::GetActiveConfigIdResponse* msg, ::int64_t val) {
  msg->set_result(val);
}


}  // extern "C"
// clang-format on


// szengine.GetEntityByEntityIdRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetEntityByEntityIdRequest_new() { return new ::szengine::GetEntityByEntityIdRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetEntityByEntityIdRequest_default_instance() {
  return &::szengine::GetEntityByEntityIdRequest::default_instance();
}

::int64_t proto2_rust_thunk_szengine_GetEntityByEntityIdRequest_entity_id_get(::szengine::GetEntityByEntityIdRequest* msg) {
  return msg->entity_id();
}
void proto2_rust_thunk_szengine_GetEntityByEntityIdRequest_entity_id_set(::szengine::GetEntityByEntityIdRequest* msg, ::int64_t val) {
  msg->set_entity_id(val);
}

::int64_t proto2_rust_thunk_szengine_GetEntityByEntityIdRequest_flags_get(::szengine::GetEntityByEntityIdRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_GetEntityByEntityIdRequest_flags_set(::szengine::GetEntityByEntityIdRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.GetEntityByEntityIdResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetEntityByEntityIdResponse_new() { return new ::szengine::GetEntityByEntityIdResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetEntityByEntityIdResponse_default_instance() {
  return &::szengine::GetEntityByEntityIdResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_GetEntityByEntityIdResponse_result_get(::szengine::GetEntityByEntityIdResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_GetEntityByEntityIdResponse_result_set(::szengine::GetEntityByEntityIdResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.GetEntityByRecordIdRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetEntityByRecordIdRequest_new() { return new ::szengine::GetEntityByRecordIdRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetEntityByRecordIdRequest_default_instance() {
  return &::szengine::GetEntityByRecordIdRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_GetEntityByRecordIdRequest_data_source_code_get(::szengine::GetEntityByRecordIdRequest* msg) {
  absl::string_view val = msg->data_source_code();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_GetEntityByRecordIdRequest_data_source_code_set(::szengine::GetEntityByRecordIdRequest* msg, std::string* s) {
  msg->set_data_source_code(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_GetEntityByRecordIdRequest_record_id_get(::szengine::GetEntityByRecordIdRequest* msg) {
  absl::string_view val = msg->record_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_GetEntityByRecordIdRequest_record_id_set(::szengine::GetEntityByRecordIdRequest* msg, std::string* s) {
  msg->set_record_id(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_GetEntityByRecordIdRequest_flags_get(::szengine::GetEntityByRecordIdRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_GetEntityByRecordIdRequest_flags_set(::szengine::GetEntityByRecordIdRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.GetEntityByRecordIdResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetEntityByRecordIdResponse_new() { return new ::szengine::GetEntityByRecordIdResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetEntityByRecordIdResponse_default_instance() {
  return &::szengine::GetEntityByRecordIdResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_GetEntityByRecordIdResponse_result_get(::szengine::GetEntityByRecordIdResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_GetEntityByRecordIdResponse_result_set(::szengine::GetEntityByRecordIdResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.GetRecordPreviewRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetRecordPreviewRequest_new() { return new ::szengine::GetRecordPreviewRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetRecordPreviewRequest_default_instance() {
  return &::szengine::GetRecordPreviewRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_GetRecordPreviewRequest_record_definition_get(::szengine::GetRecordPreviewRequest* msg) {
  absl::string_view val = msg->record_definition();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_GetRecordPreviewRequest_record_definition_set(::szengine::GetRecordPreviewRequest* msg, std::string* s) {
  msg->set_record_definition(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_GetRecordPreviewRequest_flags_get(::szengine::GetRecordPreviewRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_GetRecordPreviewRequest_flags_set(::szengine::GetRecordPreviewRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.GetRecordPreviewResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetRecordPreviewResponse_new() { return new ::szengine::GetRecordPreviewResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetRecordPreviewResponse_default_instance() {
  return &::szengine::GetRecordPreviewResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_GetRecordPreviewResponse_result_get(::szengine::GetRecordPreviewResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_GetRecordPreviewResponse_result_set(::szengine::GetRecordPreviewResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.GetRecordRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetRecordRequest_new() { return new ::szengine::GetRecordRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetRecordRequest_default_instance() {
  return &::szengine::GetRecordRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_GetRecordRequest_data_source_code_get(::szengine::GetRecordRequest* msg) {
  absl::string_view val = msg->data_source_code();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_GetRecordRequest_data_source_code_set(::szengine::GetRecordRequest* msg, std::string* s) {
  msg->set_data_source_code(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_GetRecordRequest_record_id_get(::szengine::GetRecordRequest* msg) {
  absl::string_view val = msg->record_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_GetRecordRequest_record_id_set(::szengine::GetRecordRequest* msg, std::string* s) {
  msg->set_record_id(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_GetRecordRequest_flags_get(::szengine::GetRecordRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_GetRecordRequest_flags_set(::szengine::GetRecordRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.GetRecordResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetRecordResponse_new() { return new ::szengine::GetRecordResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetRecordResponse_default_instance() {
  return &::szengine::GetRecordResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_GetRecordResponse_result_get(::szengine::GetRecordResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_GetRecordResponse_result_set(::szengine::GetRecordResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.GetRedoRecordRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetRedoRecordRequest_new() { return new ::szengine::GetRedoRecordRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetRedoRecordRequest_default_instance() {
  return &::szengine::GetRedoRecordRequest::default_instance();
}


}  // extern "C"
// clang-format on


// szengine.GetRedoRecordResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetRedoRecordResponse_new() { return new ::szengine::GetRedoRecordResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetRedoRecordResponse_default_instance() {
  return &::szengine::GetRedoRecordResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_GetRedoRecordResponse_result_get(::szengine::GetRedoRecordResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_GetRedoRecordResponse_result_set(::szengine::GetRedoRecordResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.GetStatsRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetStatsRequest_new() { return new ::szengine::GetStatsRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetStatsRequest_default_instance() {
  return &::szengine::GetStatsRequest::default_instance();
}


}  // extern "C"
// clang-format on


// szengine.GetStatsResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetStatsResponse_new() { return new ::szengine::GetStatsResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetStatsResponse_default_instance() {
  return &::szengine::GetStatsResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_GetStatsResponse_result_get(::szengine::GetStatsResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_GetStatsResponse_result_set(::szengine::GetStatsResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.GetVirtualEntityByRecordIdRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetVirtualEntityByRecordIdRequest_new() { return new ::szengine::GetVirtualEntityByRecordIdRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetVirtualEntityByRecordIdRequest_default_instance() {
  return &::szengine::GetVirtualEntityByRecordIdRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_GetVirtualEntityByRecordIdRequest_record_keys_get(::szengine::GetVirtualEntityByRecordIdRequest* msg) {
  absl::string_view val = msg->record_keys();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_GetVirtualEntityByRecordIdRequest_record_keys_set(::szengine::GetVirtualEntityByRecordIdRequest* msg, std::string* s) {
  msg->set_record_keys(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_GetVirtualEntityByRecordIdRequest_flags_get(::szengine::GetVirtualEntityByRecordIdRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_GetVirtualEntityByRecordIdRequest_flags_set(::szengine::GetVirtualEntityByRecordIdRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.GetVirtualEntityByRecordIdResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_GetVirtualEntityByRecordIdResponse_new() { return new ::szengine::GetVirtualEntityByRecordIdResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_GetVirtualEntityByRecordIdResponse_default_instance() {
  return &::szengine::GetVirtualEntityByRecordIdResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_GetVirtualEntityByRecordIdResponse_result_get(::szengine::GetVirtualEntityByRecordIdResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_GetVirtualEntityByRecordIdResponse_result_set(::szengine::GetVirtualEntityByRecordIdResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.HowEntityByEntityIdRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_HowEntityByEntityIdRequest_new() { return new ::szengine::HowEntityByEntityIdRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_HowEntityByEntityIdRequest_default_instance() {
  return &::szengine::HowEntityByEntityIdRequest::default_instance();
}

::int64_t proto2_rust_thunk_szengine_HowEntityByEntityIdRequest_entity_id_get(::szengine::HowEntityByEntityIdRequest* msg) {
  return msg->entity_id();
}
void proto2_rust_thunk_szengine_HowEntityByEntityIdRequest_entity_id_set(::szengine::HowEntityByEntityIdRequest* msg, ::int64_t val) {
  msg->set_entity_id(val);
}

::int64_t proto2_rust_thunk_szengine_HowEntityByEntityIdRequest_flags_get(::szengine::HowEntityByEntityIdRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_HowEntityByEntityIdRequest_flags_set(::szengine::HowEntityByEntityIdRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.HowEntityByEntityIdResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_HowEntityByEntityIdResponse_new() { return new ::szengine::HowEntityByEntityIdResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_HowEntityByEntityIdResponse_default_instance() {
  return &::szengine::HowEntityByEntityIdResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_HowEntityByEntityIdResponse_result_get(::szengine::HowEntityByEntityIdResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_HowEntityByEntityIdResponse_result_set(::szengine::HowEntityByEntityIdResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.PrimeEngineRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_PrimeEngineRequest_new() { return new ::szengine::PrimeEngineRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_PrimeEngineRequest_default_instance() {
  return &::szengine::PrimeEngineRequest::default_instance();
}


}  // extern "C"
// clang-format on


// szengine.PrimeEngineResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_PrimeEngineResponse_new() { return new ::szengine::PrimeEngineResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_PrimeEngineResponse_default_instance() {
  return &::szengine::PrimeEngineResponse::default_instance();
}


}  // extern "C"
// clang-format on


// szengine.ProcessRedoRecordRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_ProcessRedoRecordRequest_new() { return new ::szengine::ProcessRedoRecordRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_ProcessRedoRecordRequest_default_instance() {
  return &::szengine::ProcessRedoRecordRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_ProcessRedoRecordRequest_redo_record_get(::szengine::ProcessRedoRecordRequest* msg) {
  absl::string_view val = msg->redo_record();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_ProcessRedoRecordRequest_redo_record_set(::szengine::ProcessRedoRecordRequest* msg, std::string* s) {
  msg->set_redo_record(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_ProcessRedoRecordRequest_flags_get(::szengine::ProcessRedoRecordRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_ProcessRedoRecordRequest_flags_set(::szengine::ProcessRedoRecordRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.ProcessRedoRecordResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_ProcessRedoRecordResponse_new() { return new ::szengine::ProcessRedoRecordResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_ProcessRedoRecordResponse_default_instance() {
  return &::szengine::ProcessRedoRecordResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_ProcessRedoRecordResponse_result_get(::szengine::ProcessRedoRecordResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_ProcessRedoRecordResponse_result_set(::szengine::ProcessRedoRecordResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.ReevaluateEntityRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_ReevaluateEntityRequest_new() { return new ::szengine::ReevaluateEntityRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_ReevaluateEntityRequest_default_instance() {
  return &::szengine::ReevaluateEntityRequest::default_instance();
}

::int64_t proto2_rust_thunk_szengine_ReevaluateEntityRequest_entity_id_get(::szengine::ReevaluateEntityRequest* msg) {
  return msg->entity_id();
}
void proto2_rust_thunk_szengine_ReevaluateEntityRequest_entity_id_set(::szengine::ReevaluateEntityRequest* msg, ::int64_t val) {
  msg->set_entity_id(val);
}

::int64_t proto2_rust_thunk_szengine_ReevaluateEntityRequest_flags_get(::szengine::ReevaluateEntityRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_ReevaluateEntityRequest_flags_set(::szengine::ReevaluateEntityRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.ReevaluateEntityResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_ReevaluateEntityResponse_new() { return new ::szengine::ReevaluateEntityResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_ReevaluateEntityResponse_default_instance() {
  return &::szengine::ReevaluateEntityResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_ReevaluateEntityResponse_result_get(::szengine::ReevaluateEntityResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_ReevaluateEntityResponse_result_set(::szengine::ReevaluateEntityResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.ReevaluateRecordRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_ReevaluateRecordRequest_new() { return new ::szengine::ReevaluateRecordRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_ReevaluateRecordRequest_default_instance() {
  return &::szengine::ReevaluateRecordRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_ReevaluateRecordRequest_data_source_code_get(::szengine::ReevaluateRecordRequest* msg) {
  absl::string_view val = msg->data_source_code();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_ReevaluateRecordRequest_data_source_code_set(::szengine::ReevaluateRecordRequest* msg, std::string* s) {
  msg->set_data_source_code(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_ReevaluateRecordRequest_record_id_get(::szengine::ReevaluateRecordRequest* msg) {
  absl::string_view val = msg->record_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_ReevaluateRecordRequest_record_id_set(::szengine::ReevaluateRecordRequest* msg, std::string* s) {
  msg->set_record_id(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_ReevaluateRecordRequest_flags_get(::szengine::ReevaluateRecordRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_ReevaluateRecordRequest_flags_set(::szengine::ReevaluateRecordRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.ReevaluateRecordResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_ReevaluateRecordResponse_new() { return new ::szengine::ReevaluateRecordResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_ReevaluateRecordResponse_default_instance() {
  return &::szengine::ReevaluateRecordResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_ReevaluateRecordResponse_result_get(::szengine::ReevaluateRecordResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_ReevaluateRecordResponse_result_set(::szengine::ReevaluateRecordResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.ReinitializeRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_ReinitializeRequest_new() { return new ::szengine::ReinitializeRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_ReinitializeRequest_default_instance() {
  return &::szengine::ReinitializeRequest::default_instance();
}

::int64_t proto2_rust_thunk_szengine_ReinitializeRequest_config_id_get(::szengine::ReinitializeRequest* msg) {
  return msg->config_id();
}
void proto2_rust_thunk_szengine_ReinitializeRequest_config_id_set(::szengine::ReinitializeRequest* msg, ::int64_t val) {
  msg->set_config_id(val);
}


}  // extern "C"
// clang-format on


// szengine.ReinitializeResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_ReinitializeResponse_new() { return new ::szengine::ReinitializeResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_ReinitializeResponse_default_instance() {
  return &::szengine::ReinitializeResponse::default_instance();
}


}  // extern "C"
// clang-format on


// szengine.SearchByAttributesRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_SearchByAttributesRequest_new() { return new ::szengine::SearchByAttributesRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_SearchByAttributesRequest_default_instance() {
  return &::szengine::SearchByAttributesRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_SearchByAttributesRequest_attributes_get(::szengine::SearchByAttributesRequest* msg) {
  absl::string_view val = msg->attributes();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_SearchByAttributesRequest_attributes_set(::szengine::SearchByAttributesRequest* msg, std::string* s) {
  msg->set_attributes(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_SearchByAttributesRequest_search_profile_get(::szengine::SearchByAttributesRequest* msg) {
  absl::string_view val = msg->search_profile();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_SearchByAttributesRequest_search_profile_set(::szengine::SearchByAttributesRequest* msg, std::string* s) {
  msg->set_search_profile(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_SearchByAttributesRequest_flags_get(::szengine::SearchByAttributesRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_SearchByAttributesRequest_flags_set(::szengine::SearchByAttributesRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.SearchByAttributesResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_SearchByAttributesResponse_new() { return new ::szengine::SearchByAttributesResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_SearchByAttributesResponse_default_instance() {
  return &::szengine::SearchByAttributesResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_SearchByAttributesResponse_result_get(::szengine::SearchByAttributesResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_SearchByAttributesResponse_result_set(::szengine::SearchByAttributesResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.StreamExportCsvEntityReportRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_StreamExportCsvEntityReportRequest_new() { return new ::szengine::StreamExportCsvEntityReportRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_StreamExportCsvEntityReportRequest_default_instance() {
  return &::szengine::StreamExportCsvEntityReportRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_StreamExportCsvEntityReportRequest_csv_column_list_get(::szengine::StreamExportCsvEntityReportRequest* msg) {
  absl::string_view val = msg->csv_column_list();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_StreamExportCsvEntityReportRequest_csv_column_list_set(::szengine::StreamExportCsvEntityReportRequest* msg, std::string* s) {
  msg->set_csv_column_list(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_StreamExportCsvEntityReportRequest_flags_get(::szengine::StreamExportCsvEntityReportRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_StreamExportCsvEntityReportRequest_flags_set(::szengine::StreamExportCsvEntityReportRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.StreamExportCsvEntityReportResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_StreamExportCsvEntityReportResponse_new() { return new ::szengine::StreamExportCsvEntityReportResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_StreamExportCsvEntityReportResponse_default_instance() {
  return &::szengine::StreamExportCsvEntityReportResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_StreamExportCsvEntityReportResponse_result_get(::szengine::StreamExportCsvEntityReportResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_StreamExportCsvEntityReportResponse_result_set(::szengine::StreamExportCsvEntityReportResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.StreamExportJsonEntityReportRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_StreamExportJsonEntityReportRequest_new() { return new ::szengine::StreamExportJsonEntityReportRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_StreamExportJsonEntityReportRequest_default_instance() {
  return &::szengine::StreamExportJsonEntityReportRequest::default_instance();
}

::int64_t proto2_rust_thunk_szengine_StreamExportJsonEntityReportRequest_flags_get(::szengine::StreamExportJsonEntityReportRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_StreamExportJsonEntityReportRequest_flags_set(::szengine::StreamExportJsonEntityReportRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.StreamExportJsonEntityReportResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_StreamExportJsonEntityReportResponse_new() { return new ::szengine::StreamExportJsonEntityReportResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_StreamExportJsonEntityReportResponse_default_instance() {
  return &::szengine::StreamExportJsonEntityReportResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_StreamExportJsonEntityReportResponse_result_get(::szengine::StreamExportJsonEntityReportResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_StreamExportJsonEntityReportResponse_result_set(::szengine::StreamExportJsonEntityReportResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.WhyEntitiesRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_WhyEntitiesRequest_new() { return new ::szengine::WhyEntitiesRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_WhyEntitiesRequest_default_instance() {
  return &::szengine::WhyEntitiesRequest::default_instance();
}

::int64_t proto2_rust_thunk_szengine_WhyEntitiesRequest_entity_id_1_get(::szengine::WhyEntitiesRequest* msg) {
  return msg->entity_id_1();
}
void proto2_rust_thunk_szengine_WhyEntitiesRequest_entity_id_1_set(::szengine::WhyEntitiesRequest* msg, ::int64_t val) {
  msg->set_entity_id_1(val);
}

::int64_t proto2_rust_thunk_szengine_WhyEntitiesRequest_entity_id_2_get(::szengine::WhyEntitiesRequest* msg) {
  return msg->entity_id_2();
}
void proto2_rust_thunk_szengine_WhyEntitiesRequest_entity_id_2_set(::szengine::WhyEntitiesRequest* msg, ::int64_t val) {
  msg->set_entity_id_2(val);
}

::int64_t proto2_rust_thunk_szengine_WhyEntitiesRequest_flags_get(::szengine::WhyEntitiesRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_WhyEntitiesRequest_flags_set(::szengine::WhyEntitiesRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.WhyEntitiesResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_WhyEntitiesResponse_new() { return new ::szengine::WhyEntitiesResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_WhyEntitiesResponse_default_instance() {
  return &::szengine::WhyEntitiesResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_WhyEntitiesResponse_result_get(::szengine::WhyEntitiesResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_WhyEntitiesResponse_result_set(::szengine::WhyEntitiesResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.WhyRecordInEntityRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_WhyRecordInEntityRequest_new() { return new ::szengine::WhyRecordInEntityRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_WhyRecordInEntityRequest_default_instance() {
  return &::szengine::WhyRecordInEntityRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_WhyRecordInEntityRequest_data_source_code_get(::szengine::WhyRecordInEntityRequest* msg) {
  absl::string_view val = msg->data_source_code();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_WhyRecordInEntityRequest_data_source_code_set(::szengine::WhyRecordInEntityRequest* msg, std::string* s) {
  msg->set_data_source_code(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_WhyRecordInEntityRequest_record_id_get(::szengine::WhyRecordInEntityRequest* msg) {
  absl::string_view val = msg->record_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_WhyRecordInEntityRequest_record_id_set(::szengine::WhyRecordInEntityRequest* msg, std::string* s) {
  msg->set_record_id(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_WhyRecordInEntityRequest_flags_get(::szengine::WhyRecordInEntityRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_WhyRecordInEntityRequest_flags_set(::szengine::WhyRecordInEntityRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.WhyRecordInEntityResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_WhyRecordInEntityResponse_new() { return new ::szengine::WhyRecordInEntityResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_WhyRecordInEntityResponse_default_instance() {
  return &::szengine::WhyRecordInEntityResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_WhyRecordInEntityResponse_result_get(::szengine::WhyRecordInEntityResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_WhyRecordInEntityResponse_result_set(::szengine::WhyRecordInEntityResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.WhyRecordsRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_WhyRecordsRequest_new() { return new ::szengine::WhyRecordsRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_WhyRecordsRequest_default_instance() {
  return &::szengine::WhyRecordsRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_WhyRecordsRequest_data_source_code_1_get(::szengine::WhyRecordsRequest* msg) {
  absl::string_view val = msg->data_source_code_1();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_WhyRecordsRequest_data_source_code_1_set(::szengine::WhyRecordsRequest* msg, std::string* s) {
  msg->set_data_source_code_1(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_WhyRecordsRequest_record_id_1_get(::szengine::WhyRecordsRequest* msg) {
  absl::string_view val = msg->record_id_1();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_WhyRecordsRequest_record_id_1_set(::szengine::WhyRecordsRequest* msg, std::string* s) {
  msg->set_record_id_1(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_WhyRecordsRequest_data_source_code_2_get(::szengine::WhyRecordsRequest* msg) {
  absl::string_view val = msg->data_source_code_2();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_WhyRecordsRequest_data_source_code_2_set(::szengine::WhyRecordsRequest* msg, std::string* s) {
  msg->set_data_source_code_2(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_WhyRecordsRequest_record_id_2_get(::szengine::WhyRecordsRequest* msg) {
  absl::string_view val = msg->record_id_2();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_WhyRecordsRequest_record_id_2_set(::szengine::WhyRecordsRequest* msg, std::string* s) {
  msg->set_record_id_2(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_WhyRecordsRequest_flags_get(::szengine::WhyRecordsRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_WhyRecordsRequest_flags_set(::szengine::WhyRecordsRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.WhyRecordsResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_WhyRecordsResponse_new() { return new ::szengine::WhyRecordsResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_WhyRecordsResponse_default_instance() {
  return &::szengine::WhyRecordsResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_WhyRecordsResponse_result_get(::szengine::WhyRecordsResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_WhyRecordsResponse_result_set(::szengine::WhyRecordsResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


// szengine.WhySearchRequest
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_WhySearchRequest_new() { return new ::szengine::WhySearchRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_WhySearchRequest_default_instance() {
  return &::szengine::WhySearchRequest::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_WhySearchRequest_attributes_get(::szengine::WhySearchRequest* msg) {
  absl::string_view val = msg->attributes();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_WhySearchRequest_attributes_set(::szengine::WhySearchRequest* msg, std::string* s) {
  msg->set_attributes(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_WhySearchRequest_entity_id_get(::szengine::WhySearchRequest* msg) {
  return msg->entity_id();
}
void proto2_rust_thunk_szengine_WhySearchRequest_entity_id_set(::szengine::WhySearchRequest* msg, ::int64_t val) {
  msg->set_entity_id(val);
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_WhySearchRequest_search_profile_get(::szengine::WhySearchRequest* msg) {
  absl::string_view val = msg->search_profile();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_WhySearchRequest_search_profile_set(::szengine::WhySearchRequest* msg, std::string* s) {
  msg->set_search_profile(std::move(*s));
  delete s;
}

::int64_t proto2_rust_thunk_szengine_WhySearchRequest_flags_get(::szengine::WhySearchRequest* msg) {
  return msg->flags();
}
void proto2_rust_thunk_szengine_WhySearchRequest_flags_set(::szengine::WhySearchRequest* msg, ::int64_t val) {
  msg->set_flags(val);
}


}  // extern "C"
// clang-format on


// szengine.WhySearchResponse
// However, ~ that confuses clang-format (it refuses to keep the
// newline after ~ `"C"{`). Disabling clang-format for the block.
// clang-format off
extern "C" {
void* proto2_rust_thunk_Message_szengine_WhySearchResponse_new() { return new ::szengine::WhySearchResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_szengine_WhySearchResponse_default_instance() {
  return &::szengine::WhySearchResponse::default_instance();
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_szengine_WhySearchResponse_result_get(::szengine::WhySearchResponse* msg) {
  absl::string_view val = msg->result();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_szengine_WhySearchResponse_result_set(::szengine::WhySearchResponse* msg, std::string* s) {
  msg->set_result(std::move(*s));
  delete s;
}


}  // extern "C"
// clang-format on


