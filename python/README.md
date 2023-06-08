# senzing-ce-grpc-protobuf

## Synopsis

Auto-generated python protobuf source for calling Senzing through grpc

## Overview

The git repository at
[github.com/Senzing/senzing-ce-grpc-protobuf](https://github.com/Senzing/g2-sdk-proto)
contains the .proto files and generated code for many languages

It also contains:

- Tooling to create Python "wheel" packages

### Contents

1. [Build](#build)
2. [Install](#install)

## Build

1. Run the create_package.py script to generate and build the package
   Example:

   ```console
   python3 create_package.py
   ```

## Install

1. Use the [pip install](https://pip.pypa.io/en/stable/cli/pip_install/)
   command to install
   Example:

    ```console
    pip install dist/senzing_grpc_protobuf-0.0.3-py3-none-any.whl
    ```

