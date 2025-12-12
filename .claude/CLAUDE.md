# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This repository contains Protocol Buffer `.proto` files for the Senzing SDK gRPC interface. It generates client/server code for multiple languages from these definitions.

## Build Commands

```bash
# Install development dependencies (one-time)
make dependencies-for-development

# Install runtime dependencies (activates venv, installs Python/Go deps)
make dependencies

# Generate code for all languages (C++, Go, Java, PHP, Python, Ruby)
make generate

# Generate for specific language
make generate-go
make generate-python
make generate-csharp
make generate-java
make generate-php
make generate-ruby
make generate-rust
make generate-nodejs      # Requires: make dependencies-for-nodejs
make generate-typescript  # or: make generate-ts

# Clean generated files
make clean
make clean-go
make clean-python

# Package Python wheel
make package

# View Go documentation (starts godoc server at localhost:6060)
make documentation
```

## Architecture

### Proto Files (root directory)

Five `.proto` files define the Senzing gRPC services:

- `szconfig.proto` - Configuration management (data source registration)
- `szconfigmanager.proto` - Config persistence (get/set default configs)
- `szdiagnostic.proto` - System diagnostics (repository info, performance checks)
- `szengine.proto` - Core entity resolution (add/delete records, search, why analysis)
- `szproduct.proto` - Product info (license, version)

### Generated Code Locations

- `go/` - **Production** Go module (`github.com/senzing-garage/sz-sdk-proto`)
- `python/src/senzing_grpc_protobuf/` - Python package (published to PyPI)
- `example_generated_source_code/` - Example output for other languages (cpp, java, php, ruby, nodejs, ts)

### Generated File Patterns

- `*_grpc.pb.go` / `*_grpc_pb2.py` - gRPC service stubs (client/server interfaces)
- `*.pb.go` / `*_pb2.py` - Protocol buffer message definitions

## Modifying Proto Files

When editing `.proto` files:

1. Each service has `option go_package` pointing to `github.com/senzing-garage/sz-sdk-go-grpc/...`
2. Each service has `option java_package` set to `com.senzing.sdk.grpc.proto`
3. After changes, run `make generate` to regenerate all language bindings
4. The Go code in `go/` is the primary artifact; other languages are examples

## Python Virtual Environment

The Makefile manages a `.venv` Python environment. To activate manually:

```bash
source .venv/bin/activate
```
