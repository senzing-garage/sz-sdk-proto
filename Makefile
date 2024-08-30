# Makefile for sz-sdk-proto

# Detect the operating system and architecture.

include makefiles/osdetect.mk

# -----------------------------------------------------------------------------
# Variables
# -----------------------------------------------------------------------------

# "Simple expanded" variables (':=')

# PROGRAM_NAME is the name of the GIT repository.
PROGRAM_NAME := $(shell basename `git rev-parse --show-toplevel`)
MAKEFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIRECTORY := $(dir $(MAKEFILE_PATH))
TARGET_DIRECTORY := $(MAKEFILE_DIRECTORY)/target
BUILD_VERSION := $(shell git describe --always --tags --abbrev=0 --dirty  | sed 's/v//')
BUILD_TAG := $(shell git describe --always --tags --abbrev=0  | sed 's/v//')
BUILD_ITERATION := $(shell git log $(BUILD_TAG)..HEAD --oneline | wc -l | sed 's/^ *//')
GIT_REMOTE_URL := $(shell git config --get remote.origin.url)
GO_PACKAGE_NAME := $(shell echo $(GIT_REMOTE_URL) | sed -e 's|^git@github.com:|github.com/|' -e 's|\.git$$||' -e 's|Senzing|senzing|')
SENZING_COMPONENTS := szconfig szconfigmanager szdiagnostic szengine szproduct

# Recursive assignment ('=')

# Conditional assignment. ('?=')
# Can be overridden with "export"

# Export environment variables.

.EXPORT_ALL_VARIABLES:

# -----------------------------------------------------------------------------
# The first "make" target runs as default.
# -----------------------------------------------------------------------------

.PHONY: default
default: help

# -----------------------------------------------------------------------------
# Operating System / Architecture targets
# -----------------------------------------------------------------------------

-include makefiles/$(OSTYPE).mk
-include makefiles/$(OSTYPE)_$(OSARCH).mk


.PHONY: hello-world
hello-world: hello-world-osarch-specific

# -----------------------------------------------------------------------------
# Dependency management
# -----------------------------------------------------------------------------

.PHONY: dependencies-for-development
dependencies-for-development: dependencies-for-development-osarch-specific
	@go install golang.org/x/tools/cmd/godoc@latest


.PHONY: dependencies
dependencies:
	@go get -u ./...
	@go get -t -u ./...
	@go mod tidy

# -----------------------------------------------------------------------------
# Setup
# -----------------------------------------------------------------------------

.PHONY: setup
setup: generate

# -----------------------------------------------------------------------------
# Generate code
# -----------------------------------------------------------------------------

.PHONY: generate
generate: generate-csharp generate-go generate-java generate-php generate-python generate-ruby


.PHONY: generate-csharp
generate-csharp:
	for SENZING_COMPONENT in $(SENZING_COMPONENTS); do \
		OUTPUT_DIR=example_generated_source_code/cpp/$${SENZING_COMPONENT}; \
		mkdir -p $${OUTPUT_DIR}; \
		protoc --cpp_out=$${OUTPUT_DIR}  $${SENZING_COMPONENT}.proto; \
		protoc --grpc_out=$${OUTPUT_DIR}  --plugin=protoc-gen-grpc=`which grpc_cpp_plugin`  $${SENZING_COMPONENT}.proto; \
	done


.PHONY: generate-go
generate-go:
	for SENZING_COMPONENT in $(SENZING_COMPONENTS); do \
		OUTPUT_DIR=go/$${SENZING_COMPONENT}; \
		mkdir -p $${OUTPUT_DIR}; \
		protoc --go_out=$${OUTPUT_DIR} --go_opt=paths=source_relative --go-grpc_out=$${OUTPUT_DIR} --go-grpc_opt=paths=source_relative $${SENZING_COMPONENT}.proto; \
	done


.PHONY: generate-java
generate-java:
	for SENZING_COMPONENT in $(SENZING_COMPONENTS); do \
		OUTPUT_DIR=example_generated_source_code/java/$${SENZING_COMPONENT}; \
		mkdir -p $${OUTPUT_DIR}; \
		protoc --java_out=$${OUTPUT_DIR}  $${SENZING_COMPONENT}.proto; \
	done


.PHONY: generate-php
generate-php:
	for SENZING_COMPONENT in $(SENZING_COMPONENTS); do \
		OUTPUT_DIR=example_generated_source_code/php/$${SENZING_COMPONENT}; \
		mkdir -p $${OUTPUT_DIR}; \
		protoc --php_out=$${OUTPUT_DIR}  --grpc_out=$${OUTPUT_DIR}  --plugin=protoc-gen-grpc=`which grpc_php_plugin`  $${SENZING_COMPONENT}.proto; \
	done


.PHONY: generate-python
generate-python:
	for SENZING_COMPONENT in $(SENZING_COMPONENTS); do \
		OUTPUT_DIR=example_generated_source_code/python/$${SENZING_COMPONENT}; \
		mkdir -p $${OUTPUT_DIR}; \
		python3 -m grpc_tools.protoc --proto_path=. --python_out=$${OUTPUT_DIR}  --grpc_python_out=$${OUTPUT_DIR}  $${SENZING_COMPONENT}.proto; \
	done


.PHONY: generate-ruby
generate-ruby:
	for SENZING_COMPONENT in $(SENZING_COMPONENTS); do \
		OUTPUT_DIR=example_generated_source_code/ruby/$${SENZING_COMPONENT}; \
		mkdir -p $${OUTPUT_DIR}; \
		grpc_tools_ruby_protoc --proto_path=. --ruby_out=$${OUTPUT_DIR}  --grpc_out=$${OUTPUT_DIR}  $${SENZING_COMPONENT}.proto; \
	done

# -----------------------------------------------------------------------------
# Documentation
# -----------------------------------------------------------------------------

.PHONY: documentation
documentation: documentation-osarch-specific

# -----------------------------------------------------------------------------
# Clean
# -----------------------------------------------------------------------------

.PHONY: clean
clean: clean-csharp clean-go clean-java clean-php clean-python clean-ruby clean-osarch-specific


.PHONY: clean-csharp
clean-csharp:
	@rm -rf $(MAKEFILE_DIRECTORY)example_generated_source_code/cpp/* || true


.PHONY: clean-go
clean-go:
	rm -rf $(MAKEFILE_DIRECTORY)go/szconfig || true
	@rm -rf $(MAKEFILE_DIRECTORY)go/szconfigmgr || true
	@rm -rf $(MAKEFILE_DIRECTORY)go/szdiagnostic || true
	@rm -rf $(MAKEFILE_DIRECTORY)go/szengine || true
	@rm -rf $(MAKEFILE_DIRECTORY)go/szproduct || true


.PHONY: clean-java
clean-java:
	@rm -rf $(MAKEFILE_DIRECTORY)example_generated_source_code/java/* || true


.PHONY: clean-php
clean-php:
	@rm -rf $(MAKEFILE_DIRECTORY)example_generated_source_code/php/* || true


.PHONY: clean-python
clean-python:
	@rm -rf $(MAKEFILE_DIRECTORY)example_generated_source_code/python/szconfig/*        || true
	@rm -rf $(MAKEFILE_DIRECTORY)example_generated_source_code/python/szconfigmanager/* || true
	@rm -rf $(MAKEFILE_DIRECTORY)example_generated_source_code/python/szdiagnostic/*    || true
	@rm -rf $(MAKEFILE_DIRECTORY)example_generated_source_code/python/szengine/*        || true
	@rm -rf $(MAKEFILE_DIRECTORY)example_generated_source_code/python/szproduct/*       || true


.PHONY: clean-ruby
clean-ruby:
	@rm -rf $(MAKEFILE_DIRECTORY)example_generated_source_code/ruby/* || true

# -----------------------------------------------------------------------------
# Utility targets
# -----------------------------------------------------------------------------

.PHONY: help
help:
	$(info Build $(PROGRAM_NAME) version $(BUILD_VERSION)-$(BUILD_ITERATION))
	$(info Makefile targets:)
	@$(MAKE) -pRrq -f $(firstword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/^# File/,/^# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | egrep -v -e '^[^[:alnum:]]' -e '^$@$$' | xargs


.PHONY: print-make-variables
print-make-variables:
	@$(foreach V,$(sort $(.VARIABLES)), \
		$(if $(filter-out environment% default automatic, \
		$(origin $V)),$(info $V=$($V) ($(value $V)))))


.PHONY: update-pkg-cache
update-pkg-cache:
	@GOPROXY=https://proxy.golang.org GO111MODULE=on \
		go get $(GO_PACKAGE_NAME)@$(BUILD_TAG)
