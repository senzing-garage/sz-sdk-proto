// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: g2configmgr.proto

package com.senzing.g2.engine.grpc.G2ConfigMgr;

public interface ReplaceDefaultConfigIDRequestOrBuilder extends
    // @@protoc_insertion_point(interface_extends:g2configmgr.ReplaceDefaultConfigIDRequest)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <code>int64 oldConfigID = 1;</code>
   * @return The oldConfigID.
   */
  long getOldConfigID();

  /**
   * <code>int64 newConfigID = 2;</code>
   * @return The newConfigID.
   */
  long getNewConfigID();
}