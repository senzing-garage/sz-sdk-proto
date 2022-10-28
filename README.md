# g2-sdk-proto

## Synopsis

:warning:
This is a work-in-progress.
Feel free to comment on it, but do not use it in production, yet.
:warning:

The repository for Senzing's Protocol Buffer `.proto` files.

## Overview

Currently this repository is undergoing review.

### Contents

1. [Preamble](#preamble)
    1. [Legend](#legend)
1. [Related artifacts](#related-artifacts)
1. [Expectations](#expectations)
1. [Clone repository](#clone-repository)
1. [Language](#language)
    1. [C++](#c)
    1. [Go](#go)
    1. [Java](#java)
    1. [PHP](#php)
    1. [Python](#python)
    1. [Ruby](#ruby)
    1. [.NET](#net)
1. [Errors](#errors)
1. [References](#references)

## Preamble

At [Senzing](http://senzing.com),
we strive to create GitHub documentation in a
"[don't make me think](https://github.com/Senzing/knowledge-base/blob/main/WHATIS/dont-make-me-think.md)" style.
For the most part, instructions are copy and paste.
Whenever thinking is needed, it's marked with a "thinking" icon :thinking:.
Whenever customization is needed, it's marked with a "pencil" icon :pencil2:.
If the instructions are not clear, please let us know by opening a new
[Documentation issue](https://github.com/Senzing/template-python/issues/new?template=documentation_request.md)
describing where we can improve.   Now on with the show...

### Legend

1. :thinking: - A "thinker" icon means that a little extra thinking may be required.
   Perhaps there are some choices to be made.
   Perhaps it's an optional step.
1. :pencil2: - A "pencil" icon means that the instructions may need modification before performing.
1. :warning: - A "warning" icon means that something tricky is happening, so pay attention.

## Related artifacts

1. [DockerHub](https://hub.docker.com/r/senzing/xxxxxxxx)
1. [Helm Chart](https://github.com/Senzing/charts/tree/main/charts/xxxxxxxx)

## Expectations

- **Space:** This repository and demonstration require 6 GB free disk space.
- **Time:** Budget 40 minutes to get the demonstration up-and-running, depending on CPU and network speeds.
- **Background knowledge:** This repository assumes a working knowledge of:
  - [Docker](https://github.com/Senzing/knowledge-base/blob/main/WHATIS/docker.md)

## Clone repository

1. Set these environment variable values:

    ```console
    export GIT_ACCOUNT=senzing
    export GIT_REPOSITORY=g2-sdk-proto
    export GIT_ACCOUNT_DIR=~/${GIT_ACCOUNT}.git
    export GIT_REPOSITORY_DIR="${GIT_ACCOUNT_DIR}/${GIT_REPOSITORY}"

    ```

1. Using the environment variables values just set, follow steps in [clone-repository](https://github.com/Senzing/knowledge-base/blob/main/HOWTO/clone-repository.md) to install the Git repository.

## Language

The following instructions were used to create
[example generated source code](example_generated_source_code).

### C++

1. [Clone repository](#clone-repository).
1. Follow the
   [C++ Quick start](https://grpc.io/docs/languages/cpp/quickstart/)
   tutorial to prepare an environment.
1. Generate message handling code.

    ```console
    export SENZING_OUTPUT_DIR=${GIT_REPOSITORY_DIR}/example_generated_source_code/cpp
    mkdir -p ${SENZING_OUTPUT_DIR}

    protoc \
        --proto_path=${GIT_REPOSITORY_DIR} \
        --cpp_out=${SENZING_OUTPUT_DIR} \
        ${GIT_REPOSITORY_DIR}/g2config.proto \
        ${GIT_REPOSITORY_DIR}/g2configmgr.proto \
        ${GIT_REPOSITORY_DIR}/g2diagnostic.proto \
        ${GIT_REPOSITORY_DIR}/g2engine.proto \
        ${GIT_REPOSITORY_DIR}/g2hasher.proto \
        ${GIT_REPOSITORY_DIR}/g2product.proto \
        ${GIT_REPOSITORY_DIR}/g2ssadm.proto

    ```

1. Generating client and server code.
   Example:

    ```console
    export SENZING_OUTPUT_DIR=${GIT_REPOSITORY_DIR}/example_generated_source_code/cpp
    mkdir -p ${SENZING_OUTPUT_DIR}

    protoc \
        --proto_path=${GIT_REPOSITORY_DIR} \
        --grpc_out=${SENZING_OUTPUT_DIR} \
        --plugin=protoc-gen-grpc=`which grpc_cpp_plugin` \
        ${GIT_REPOSITORY_DIR}/g2config.proto \
        ${GIT_REPOSITORY_DIR}/g2configmgr.proto \
        ${GIT_REPOSITORY_DIR}/g2diagnostic.proto \
        ${GIT_REPOSITORY_DIR}/g2engine.proto \
        ${GIT_REPOSITORY_DIR}/g2hasher.proto \
        ${GIT_REPOSITORY_DIR}/g2product.proto \
        ${GIT_REPOSITORY_DIR}/g2ssadm.proto

    ```

### Go

1. [Clone repository](#clone-repository).
1. Follow the
   [Go Quick start](https://grpc.io/docs/languages/go/quickstart/)
   tutorial to prepare an environment.
1. [Generating client and server code](https://grpc.io/docs/languages/go/basics/#generating-client-and-server-code).
   Example:

    ```console
    export SENZING_OUTPUT_DIR=${GIT_REPOSITORY_DIR}/example_generated_source_code/go
    mkdir -p ${SENZING_OUTPUT_DIR}

    protoc \
        --proto_path=${GIT_REPOSITORY_DIR} \
        --go_out=${SENZING_OUTPUT_DIR} \
        --go_opt=paths=source_relative \
        --go-grpc_out=${SENZING_OUTPUT_DIR} \
        --go-grpc_opt=paths=source_relative \
        ${GIT_REPOSITORY_DIR}/g2config.proto \
        ${GIT_REPOSITORY_DIR}/g2configmgr.proto \
        ${GIT_REPOSITORY_DIR}/g2diagnostic.proto \
        ${GIT_REPOSITORY_DIR}/g2engine.proto \
        ${GIT_REPOSITORY_DIR}/g2hasher.proto \
        ${GIT_REPOSITORY_DIR}/g2product.proto \
        ${GIT_REPOSITORY_DIR}/g2ssadm.proto

    ```

    1. In `${SENZING_OUTPUT_DIR}`, files *with* `_grpc.` in the filename contain the following:
        - Interface types (or stubs) for clients to call with the methods defined in the services.
        - Interface types for servers to implement, also with the methods defined in the services.
        - In other words, its the "gRPC" code that handles the network traffic, not the message content.
    1. In `${SENZING_OUTPUT_DIR}`, files *without* `_grpc.` in the filename contain the following:
        - protocol buffer code to populate, serialize, and retrieve request and response message types.
        - In other workds, it manages message content, not the network traffic.
1. **References:**
    1. [gRPC Documents for Go](https://grpc.io/docs/languages/go/)
        1. [Go Quick start](https://grpc.io/docs/languages/go/quickstart/)
    1. [Thread safety](https://grpc.io/docs/languages/go/generated-code/)

### Java

1. [Clone repository](#clone-repository).
1. Follow the
   [Java Quick start](https://grpc.io/docs/languages/java/quickstart/)
   tutorial to prepare an environment.
1. Generate client and server code.
   Example:

    ```console
    export SENZING_OUTPUT_DIR=${GIT_REPOSITORY_DIR}/example_generated_source_code/java
    mkdir -p ${SENZING_OUTPUT_DIR}

    protoc \
        --proto_path=${GIT_REPOSITORY_DIR} \
        --java_out=${SENZING_OUTPUT_DIR} \
        ${GIT_REPOSITORY_DIR}/g2config.proto \
        ${GIT_REPOSITORY_DIR}/g2configmgr.proto \
        ${GIT_REPOSITORY_DIR}/g2diagnostic.proto \
        ${GIT_REPOSITORY_DIR}/g2engine.proto \
        ${GIT_REPOSITORY_DIR}/g2hasher.proto \
        ${GIT_REPOSITORY_DIR}/g2product.proto \
        ${GIT_REPOSITORY_DIR}/g2ssadm.proto

    ```

### PHP

1. [Clone repository](#clone-repository).
1. Follow the
   [PHP Quick start](https://grpc.io/docs/languages/php/quickstart/)
   tutorial to prepare an environment.
1. Generate client and server code.
   Example:

    ```console
    export SENZING_OUTPUT_DIR=${GIT_REPOSITORY_DIR}/example_generated_source_code/php
    mkdir -p ${SENZING_OUTPUT_DIR}

    protoc \
        --proto_path=${GIT_REPOSITORY_DIR} \
        --php_out=${SENZING_OUTPUT_DIR} \
        --grpc_out=${SENZING_OUTPUT_DIR} \
        --plugin=protoc-gen-grpc=`which grpc_php_plugin` \
        ${GIT_REPOSITORY_DIR}/g2config.proto \
        ${GIT_REPOSITORY_DIR}/g2configmgr.proto \
        ${GIT_REPOSITORY_DIR}/g2diagnostic.proto \
        ${GIT_REPOSITORY_DIR}/g2engine.proto \
        ${GIT_REPOSITORY_DIR}/g2hasher.proto \
        ${GIT_REPOSITORY_DIR}/g2product.proto \
        ${GIT_REPOSITORY_DIR}/g2ssadm.proto

    ```

### Python

1. [Clone repository](#clone-repository).
1. Follow the
   [Python Quick start](https://grpc.io/docs/languages/python/quickstart/)
   tutorial to prepare an environment.
1. Generate client and server code.
   Example:

    ```console
    export SENZING_OUTPUT_DIR=${GIT_REPOSITORY_DIR}/example_generated_source_code/python
    mkdir -p ${SENZING_OUTPUT_DIR}

    python3 -m grpc_tools.protoc \
        --proto_path=${GIT_REPOSITORY_DIR} \
        --python_out=${SENZING_OUTPUT_DIR} \
        --grpc_python_out=${SENZING_OUTPUT_DIR} \
        ${GIT_REPOSITORY_DIR}/g2config.proto \
        ${GIT_REPOSITORY_DIR}/g2configmgr.proto \
        ${GIT_REPOSITORY_DIR}/g2diagnostic.proto \
        ${GIT_REPOSITORY_DIR}/g2engine.proto \
        ${GIT_REPOSITORY_DIR}/g2hasher.proto \
        ${GIT_REPOSITORY_DIR}/g2product.proto \
        ${GIT_REPOSITORY_DIR}/g2ssadm.proto

    ```

### Ruby

1. [Clone repository](#clone-repository).
1. Follow the
   [Ruby Quick start](https://grpc.io/docs/languages/ruby/quickstart/)
   tutorial to prepare an environment.
1. Generate client and server code.
   Example:

    ```console
    export SENZING_OUTPUT_DIR=${GIT_REPOSITORY_DIR}/example_generated_source_code/ruby
    mkdir -p ${SENZING_OUTPUT_DIR}

    grpc_tools_ruby_protoc \
        --proto_path=${GIT_REPOSITORY_DIR} \
        --ruby_out=${SENZING_OUTPUT_DIR} \
        --grpc_out=${SENZING_OUTPUT_DIR} \
        ${GIT_REPOSITORY_DIR}/g2config.proto \
        ${GIT_REPOSITORY_DIR}/g2configmgr.proto \
        ${GIT_REPOSITORY_DIR}/g2diagnostic.proto \
        ${GIT_REPOSITORY_DIR}/g2engine.proto \
        ${GIT_REPOSITORY_DIR}/g2hasher.proto \
        ${GIT_REPOSITORY_DIR}/g2product.proto \
        ${GIT_REPOSITORY_DIR}/g2ssadm.proto

    ```

### .NET

1. **References:**
    1. [Overview for gRPC on .NET](https://learn.microsoft.com/en-us/aspnet/core/grpc)

## Errors

1. See [docs/errors.md](docs/errors.md).

## References

1. [Protocol Buffers Overview](https://developers.google.com/protocol-buffers/docs/overview)
    1. [Language Guide for proto3](https://developers.google.com/protocol-buffers/docs/proto3)
1. [Protocol buffers](https://developers.google.com/protocol-buffers)
1. [protoc man page](https://manpages.debian.org/testing/protobuf-compiler/protoc.1.en.html)
1. gPRC
    1. [Introduction to gRPC](https://grpc.io/docs/what-is-grpc/introduction/)

