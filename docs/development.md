# sz-sdk-proto development

The following instructions are useful during development.

**Note:** This has been tested on Linux and Darwin/macOS.
It has not been tested on Windows.

## Prerequisites for development

:thinking: The following tasks need to be complete before proceeding.
These are "one-time tasks" which may already have been completed.

1. The following software programs need to be installed:
    1. [git]
    1. [make]
    1. [docker]
    1. [go]

1. The following software programs need to be installed:
    1. [protoc]

## Install Git repository

1. Identify git repository.

    ```console
    export GIT_ACCOUNT=senzing-garage
    export GIT_REPOSITORY=sz-sdk-proto
    export GIT_ACCOUNT_DIR=~/${GIT_ACCOUNT}.git
    export GIT_REPOSITORY_DIR="${GIT_ACCOUNT_DIR}/${GIT_REPOSITORY}"

    ```

1. Using the environment variables values just set, follow
   steps in [clone-repository] to install the Git repository.

## Dependencies

1. A one-time command to install dependencies needed for `make` targets.
   Example:

    ```console
    cd ${GIT_REPOSITORY_DIR}
    make dependencies-for-development

    ```

1. Install dependencies needed for [Go] code.
   Example:

    ```console
    cd ${GIT_REPOSITORY_DIR}
    make dependencies

    ```

## Generate code

1. Generate code across multiple languages.
   Example:

    ```console
    cd ${GIT_REPOSITORY_DIR}
    make generate

    ```

## Language

These are manual steps for the `make generate` described above.

The following instructions were used to create a [go module] and other [example generated source code].

### Identify Senzing subcomponents

1. Enumerate the Senzing components for use in language-specific commands below.
   Example:

    ```console
    export SENZING_COMPONENTS=( \
      "szconfig" \
      "szconfigmanager" \
      "szdiagnostic" \
      "szengine" \
      "szproduct" \
    )

    ```

### C++

1. [Clone repository].
1. Follow the [C++ Quick start] tutorial to prepare an environment.
1. [Identify Senzing subcomponents].
1. Generate message handling code.
   Example:

    ```console
    for SENZING_COMPONENT in ${SENZING_COMPONENTS[@]}; \
    do \
      export SENZING_OUTPUT_DIR=${GIT_REPOSITORY_DIR}/example_generated_source_code/cpp/${SENZING_COMPONENT}
      mkdir -p ${SENZING_OUTPUT_DIR}
      protoc \
        --proto_path=${GIT_REPOSITORY_DIR}/ \
        --cpp_out=${SENZING_OUTPUT_DIR} \
        ${GIT_REPOSITORY_DIR}/${SENZING_COMPONENT}.proto
    done

    ```

1. Generating client and server code.
   Example:

    ```console
    for SENZING_COMPONENT in ${SENZING_COMPONENTS[@]}; \
    do \
      export SENZING_OUTPUT_DIR=${GIT_REPOSITORY_DIR}/example_generated_source_code/cpp/${SENZING_COMPONENT}
      mkdir -p ${SENZING_OUTPUT_DIR}
      protoc \
        --proto_path=${GIT_REPOSITORY_DIR}/ \
        --grpc_out=${SENZING_OUTPUT_DIR} \
        --plugin=protoc-gen-grpc=`which grpc_cpp_plugin` \
        ${GIT_REPOSITORY_DIR}/${SENZING_COMPONENT}.proto
    done

    ```

### Go

1. [Clone repository].
1. Follow the [Go Quick start] tutorial to prepare an environment.
1. [Identify Senzing subcomponents].
1. [Generating client and server code].
   Example:

    ```console
    for SENZING_COMPONENT in ${SENZING_COMPONENTS[@]}; \
    do \
      export SENZING_OUTPUT_DIR=${GIT_REPOSITORY_DIR}/go/${SENZING_COMPONENT}
      mkdir -p ${SENZING_OUTPUT_DIR}
      protoc \
        --proto_path=${GIT_REPOSITORY_DIR}/ \
        --go_out=${SENZING_OUTPUT_DIR} \
        --go_opt=paths=source_relative \
        --go-grpc_out=${SENZING_OUTPUT_DIR} \
        --go-grpc_opt=paths=source_relative \
        ${GIT_REPOSITORY_DIR}/${SENZING_COMPONENT}.proto
    done

    ```

    1. In `${SENZING_OUTPUT_DIR}`, files *with* `_grpc.` in the filename contain the following:
        - Interface types (or stubs) for clients to call with the methods defined in the services.
        - Interface types for servers to implement, also with the methods defined in the services.
        - In other words, its the "gRPC" code that handles the network traffic, not the message content.
    1. In `${SENZING_OUTPUT_DIR}`, files *without* `_grpc.` in the filename contain the following:
        - protocol buffer code to populate, serialize, and retrieve request and response message types.
        - In other workds, it manages message content, not the network traffic.
1. **References:**
    1. [gRPC Documents for Go]
        1. [Go Quick start]
    1. [Thread safety]

### Java

1. [Clone repository].
1. Follow the [Java Quick start] tutorial to prepare an environment.
1. [Identify Senzing subcomponents].
1. Generate client and server code.
   Example:

    ```console
    for SENZING_COMPONENT in ${SENZING_COMPONENTS[@]}; \
    do \
      export SENZING_OUTPUT_DIR=${GIT_REPOSITORY_DIR}/example_generated_source_code/java/${SENZING_COMPONENT}
      mkdir -p ${SENZING_OUTPUT_DIR}
      protoc \
        --proto_path=${GIT_REPOSITORY_DIR}/ \
        --java_out=${SENZING_OUTPUT_DIR} \
        ${GIT_REPOSITORY_DIR}/${SENZING_COMPONENT}.proto
    done

    ```

### PHP

1. [Clone repository].
1. Follow the [PHP Quick start] tutorial to prepare an environment.
1. [Identify Senzing subcomponents].
1. Generate client and server code.
   Example:

    ```console
    for SENZING_COMPONENT in ${SENZING_COMPONENTS[@]}; \
    do \
      export SENZING_OUTPUT_DIR=${GIT_REPOSITORY_DIR}/example_generated_source_code/php/${SENZING_COMPONENT}
      mkdir -p ${SENZING_OUTPUT_DIR}
      protoc \
        --proto_path=${GIT_REPOSITORY_DIR}/ \
        --php_out=${SENZING_OUTPUT_DIR} \
        --grpc_out=${SENZING_OUTPUT_DIR} \
        --plugin=protoc-gen-grpc=`which grpc_php_plugin` \
        ${GIT_REPOSITORY_DIR}/${SENZING_COMPONENT}.proto
    done

    ```

### Python

1. [Clone repository].
1. Follow the [Python Quick start] tutorial to prepare an environment.
1. [Identify Senzing subcomponents].
1. Generate client and server code.
   Example:

    ```console
    for SENZING_COMPONENT in ${SENZING_COMPONENTS[@]}; \
    do \
      export SENZING_OUTPUT_DIR=${GIT_REPOSITORY_DIR}/example_generated_source_code/python/${SENZING_COMPONENT}
      mkdir -p ${SENZING_OUTPUT_DIR}
      python3 -m grpc_tools.protoc \
        --proto_path=${GIT_REPOSITORY_DIR} \
        --python_out=${SENZING_OUTPUT_DIR} \
        --grpc_python_out=${SENZING_OUTPUT_DIR} \
        ${GIT_REPOSITORY_DIR}/${SENZING_COMPONENT}.proto
    done

    ```

### Ruby

1. [Clone repository].
1. Follow the [Ruby Quick start] tutorial to prepare an environment.
1. [Identify Senzing subcomponents].
1. Generate client and server code.
   Example:

    ```console
    for SENZING_COMPONENT in ${SENZING_COMPONENTS[@]}; \
    do \
      export SENZING_OUTPUT_DIR=${GIT_REPOSITORY_DIR}/example_generated_source_code/ruby/${SENZING_COMPONENT}
      mkdir -p ${SENZING_OUTPUT_DIR}
      grpc_tools_ruby_protoc \
        --proto_path=${GIT_REPOSITORY_DIR} \
        --ruby_out=${SENZING_OUTPUT_DIR} \
        --grpc_out=${SENZING_OUTPUT_DIR} \
        ${GIT_REPOSITORY_DIR}/${SENZING_COMPONENT}.proto
    done

    ```

### .NET

  1. [Overview for gRPC on .NET]

## References

[C++ Quick start]: https://grpc.io/docs/languages/cpp/quickstart/
[Clone repository]: #clone-repository
[clone-repository]: https://github.com/senzing-garage/knowledge-base/blob/main/HOWTO/clone-repository.md
[example generated source code]: example_generated_source_code
[Generating client and server code]: https://grpc.io/docs/languages/go/basics/#generating-client-and-server-code
[go module]: go
[Go Quick start]: https://grpc.io/docs/languages/go/quickstart/
[gRPC Documents for Go]: https://grpc.io/docs/languages/go/
[Identify Senzing subcomponents]: #identify-senzing-subcomponents
[Java Quick start]: https://grpc.io/docs/languages/java/quickstart/
[Overview for gRPC on .NET]: https://learn.microsoft.com/en-us/aspnet/core/grpc
[PHP Quick start]: https://grpc.io/docs/languages/php/quickstart/
[protoc]: https://github.com/senzing-garage/knowledge-base/blob/main/WHATIS/protoc.md
[Python Quick start]: https://grpc.io/docs/languages/python/quickstart/
[Ruby Quick start]: https://grpc.io/docs/languages/ruby/quickstart/
[Thread safety]: https://grpc.io/docs/languages/go/generated-code/
[docker]: https://github.com/senzing-garage/knowledge-base/blob/main/WHATIS/docker.md
[git]: https://github.com/senzing-garage/knowledge-base/blob/main/WHATIS/git.md
[go]: https://github.com/senzing-garage/knowledge-base/blob/main/WHATIS/go.md
[make]: https://github.com/senzing-garage/knowledge-base/blob/main/WHATIS/make.md
