// Generated by the gRPC C++ plugin.
// If you make any local change, they will be lost.
// source: szconfig.proto

#include "szconfig.pb.h"
#include "szconfig.grpc.pb.h"

#include <functional>
#include <grpcpp/impl/codegen/async_stream.h>
#include <grpcpp/impl/codegen/async_unary_call.h>
#include <grpcpp/impl/codegen/channel_interface.h>
#include <grpcpp/impl/codegen/client_unary_call.h>
#include <grpcpp/impl/codegen/client_callback.h>
#include <grpcpp/impl/codegen/message_allocator.h>
#include <grpcpp/impl/codegen/method_handler.h>
#include <grpcpp/impl/codegen/rpc_service_method.h>
#include <grpcpp/impl/codegen/server_callback.h>
#include <grpcpp/impl/codegen/server_callback_handlers.h>
#include <grpcpp/impl/codegen/server_context.h>
#include <grpcpp/impl/codegen/service_type.h>
#include <grpcpp/impl/codegen/sync_stream.h>
namespace szconfig {

static const char* SzConfig_method_names[] = {
  "/szconfig.SzConfig/AddDataSource",
  "/szconfig.SzConfig/DeleteDataSource",
  "/szconfig.SzConfig/GetDataSources",
};

std::unique_ptr< SzConfig::Stub> SzConfig::NewStub(const std::shared_ptr< ::grpc::ChannelInterface>& channel, const ::grpc::StubOptions& options) {
  (void)options;
  std::unique_ptr< SzConfig::Stub> stub(new SzConfig::Stub(channel, options));
  return stub;
}

SzConfig::Stub::Stub(const std::shared_ptr< ::grpc::ChannelInterface>& channel, const ::grpc::StubOptions& options)
  : channel_(channel), rpcmethod_AddDataSource_(SzConfig_method_names[0], options.suffix_for_stats(),::grpc::internal::RpcMethod::NORMAL_RPC, channel)
  , rpcmethod_DeleteDataSource_(SzConfig_method_names[1], options.suffix_for_stats(),::grpc::internal::RpcMethod::NORMAL_RPC, channel)
  , rpcmethod_GetDataSources_(SzConfig_method_names[2], options.suffix_for_stats(),::grpc::internal::RpcMethod::NORMAL_RPC, channel)
  {}

::grpc::Status SzConfig::Stub::AddDataSource(::grpc::ClientContext* context, const ::szconfig::AddDataSourceRequest& request, ::szconfig::AddDataSourceResponse* response) {
  return ::grpc::internal::BlockingUnaryCall< ::szconfig::AddDataSourceRequest, ::szconfig::AddDataSourceResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), rpcmethod_AddDataSource_, context, request, response);
}

void SzConfig::Stub::async::AddDataSource(::grpc::ClientContext* context, const ::szconfig::AddDataSourceRequest* request, ::szconfig::AddDataSourceResponse* response, std::function<void(::grpc::Status)> f) {
  ::grpc::internal::CallbackUnaryCall< ::szconfig::AddDataSourceRequest, ::szconfig::AddDataSourceResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_AddDataSource_, context, request, response, std::move(f));
}

void SzConfig::Stub::async::AddDataSource(::grpc::ClientContext* context, const ::szconfig::AddDataSourceRequest* request, ::szconfig::AddDataSourceResponse* response, ::grpc::ClientUnaryReactor* reactor) {
  ::grpc::internal::ClientCallbackUnaryFactory::Create< ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_AddDataSource_, context, request, response, reactor);
}

::grpc::ClientAsyncResponseReader< ::szconfig::AddDataSourceResponse>* SzConfig::Stub::PrepareAsyncAddDataSourceRaw(::grpc::ClientContext* context, const ::szconfig::AddDataSourceRequest& request, ::grpc::CompletionQueue* cq) {
  return ::grpc::internal::ClientAsyncResponseReaderHelper::Create< ::szconfig::AddDataSourceResponse, ::szconfig::AddDataSourceRequest, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), cq, rpcmethod_AddDataSource_, context, request);
}

::grpc::ClientAsyncResponseReader< ::szconfig::AddDataSourceResponse>* SzConfig::Stub::AsyncAddDataSourceRaw(::grpc::ClientContext* context, const ::szconfig::AddDataSourceRequest& request, ::grpc::CompletionQueue* cq) {
  auto* result =
    this->PrepareAsyncAddDataSourceRaw(context, request, cq);
  result->StartCall();
  return result;
}

::grpc::Status SzConfig::Stub::DeleteDataSource(::grpc::ClientContext* context, const ::szconfig::DeleteDataSourceRequest& request, ::szconfig::DeleteDataSourceResponse* response) {
  return ::grpc::internal::BlockingUnaryCall< ::szconfig::DeleteDataSourceRequest, ::szconfig::DeleteDataSourceResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), rpcmethod_DeleteDataSource_, context, request, response);
}

void SzConfig::Stub::async::DeleteDataSource(::grpc::ClientContext* context, const ::szconfig::DeleteDataSourceRequest* request, ::szconfig::DeleteDataSourceResponse* response, std::function<void(::grpc::Status)> f) {
  ::grpc::internal::CallbackUnaryCall< ::szconfig::DeleteDataSourceRequest, ::szconfig::DeleteDataSourceResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_DeleteDataSource_, context, request, response, std::move(f));
}

void SzConfig::Stub::async::DeleteDataSource(::grpc::ClientContext* context, const ::szconfig::DeleteDataSourceRequest* request, ::szconfig::DeleteDataSourceResponse* response, ::grpc::ClientUnaryReactor* reactor) {
  ::grpc::internal::ClientCallbackUnaryFactory::Create< ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_DeleteDataSource_, context, request, response, reactor);
}

::grpc::ClientAsyncResponseReader< ::szconfig::DeleteDataSourceResponse>* SzConfig::Stub::PrepareAsyncDeleteDataSourceRaw(::grpc::ClientContext* context, const ::szconfig::DeleteDataSourceRequest& request, ::grpc::CompletionQueue* cq) {
  return ::grpc::internal::ClientAsyncResponseReaderHelper::Create< ::szconfig::DeleteDataSourceResponse, ::szconfig::DeleteDataSourceRequest, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), cq, rpcmethod_DeleteDataSource_, context, request);
}

::grpc::ClientAsyncResponseReader< ::szconfig::DeleteDataSourceResponse>* SzConfig::Stub::AsyncDeleteDataSourceRaw(::grpc::ClientContext* context, const ::szconfig::DeleteDataSourceRequest& request, ::grpc::CompletionQueue* cq) {
  auto* result =
    this->PrepareAsyncDeleteDataSourceRaw(context, request, cq);
  result->StartCall();
  return result;
}

::grpc::Status SzConfig::Stub::GetDataSources(::grpc::ClientContext* context, const ::szconfig::GetDataSourcesRequest& request, ::szconfig::GetDataSourcesResponse* response) {
  return ::grpc::internal::BlockingUnaryCall< ::szconfig::GetDataSourcesRequest, ::szconfig::GetDataSourcesResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), rpcmethod_GetDataSources_, context, request, response);
}

void SzConfig::Stub::async::GetDataSources(::grpc::ClientContext* context, const ::szconfig::GetDataSourcesRequest* request, ::szconfig::GetDataSourcesResponse* response, std::function<void(::grpc::Status)> f) {
  ::grpc::internal::CallbackUnaryCall< ::szconfig::GetDataSourcesRequest, ::szconfig::GetDataSourcesResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_GetDataSources_, context, request, response, std::move(f));
}

void SzConfig::Stub::async::GetDataSources(::grpc::ClientContext* context, const ::szconfig::GetDataSourcesRequest* request, ::szconfig::GetDataSourcesResponse* response, ::grpc::ClientUnaryReactor* reactor) {
  ::grpc::internal::ClientCallbackUnaryFactory::Create< ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_GetDataSources_, context, request, response, reactor);
}

::grpc::ClientAsyncResponseReader< ::szconfig::GetDataSourcesResponse>* SzConfig::Stub::PrepareAsyncGetDataSourcesRaw(::grpc::ClientContext* context, const ::szconfig::GetDataSourcesRequest& request, ::grpc::CompletionQueue* cq) {
  return ::grpc::internal::ClientAsyncResponseReaderHelper::Create< ::szconfig::GetDataSourcesResponse, ::szconfig::GetDataSourcesRequest, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), cq, rpcmethod_GetDataSources_, context, request);
}

::grpc::ClientAsyncResponseReader< ::szconfig::GetDataSourcesResponse>* SzConfig::Stub::AsyncGetDataSourcesRaw(::grpc::ClientContext* context, const ::szconfig::GetDataSourcesRequest& request, ::grpc::CompletionQueue* cq) {
  auto* result =
    this->PrepareAsyncGetDataSourcesRaw(context, request, cq);
  result->StartCall();
  return result;
}

SzConfig::Service::Service() {
  AddMethod(new ::grpc::internal::RpcServiceMethod(
      SzConfig_method_names[0],
      ::grpc::internal::RpcMethod::NORMAL_RPC,
      new ::grpc::internal::RpcMethodHandler< SzConfig::Service, ::szconfig::AddDataSourceRequest, ::szconfig::AddDataSourceResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(
          [](SzConfig::Service* service,
             ::grpc::ServerContext* ctx,
             const ::szconfig::AddDataSourceRequest* req,
             ::szconfig::AddDataSourceResponse* resp) {
               return service->AddDataSource(ctx, req, resp);
             }, this)));
  AddMethod(new ::grpc::internal::RpcServiceMethod(
      SzConfig_method_names[1],
      ::grpc::internal::RpcMethod::NORMAL_RPC,
      new ::grpc::internal::RpcMethodHandler< SzConfig::Service, ::szconfig::DeleteDataSourceRequest, ::szconfig::DeleteDataSourceResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(
          [](SzConfig::Service* service,
             ::grpc::ServerContext* ctx,
             const ::szconfig::DeleteDataSourceRequest* req,
             ::szconfig::DeleteDataSourceResponse* resp) {
               return service->DeleteDataSource(ctx, req, resp);
             }, this)));
  AddMethod(new ::grpc::internal::RpcServiceMethod(
      SzConfig_method_names[2],
      ::grpc::internal::RpcMethod::NORMAL_RPC,
      new ::grpc::internal::RpcMethodHandler< SzConfig::Service, ::szconfig::GetDataSourcesRequest, ::szconfig::GetDataSourcesResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(
          [](SzConfig::Service* service,
             ::grpc::ServerContext* ctx,
             const ::szconfig::GetDataSourcesRequest* req,
             ::szconfig::GetDataSourcesResponse* resp) {
               return service->GetDataSources(ctx, req, resp);
             }, this)));
}

SzConfig::Service::~Service() {
}

::grpc::Status SzConfig::Service::AddDataSource(::grpc::ServerContext* context, const ::szconfig::AddDataSourceRequest* request, ::szconfig::AddDataSourceResponse* response) {
  (void) context;
  (void) request;
  (void) response;
  return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
}

::grpc::Status SzConfig::Service::DeleteDataSource(::grpc::ServerContext* context, const ::szconfig::DeleteDataSourceRequest* request, ::szconfig::DeleteDataSourceResponse* response) {
  (void) context;
  (void) request;
  (void) response;
  return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
}

::grpc::Status SzConfig::Service::GetDataSources(::grpc::ServerContext* context, const ::szconfig::GetDataSourcesRequest* request, ::szconfig::GetDataSourcesResponse* response) {
  (void) context;
  (void) request;
  (void) response;
  return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
}


}  // namespace szconfig

