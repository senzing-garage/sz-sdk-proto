# Makefile for sz-sdk-proto

# Detect the operating system and architecture.

include makefiles/osdetect.mk

# -----------------------------------------------------------------------------
# Variables
# -----------------------------------------------------------------------------

# "Simple expanded" variables (':=')

# PROGRAM_NAME is the name of the GIT repository.
PROGRAM_NAME := $(shell basename `git rev-parse --show-toplevel`)
MAKEFILE_PATH := $(abspath $(firstword $(MAKEFILE_LIST)))
MAKEFILE_DIRECTORY := $(shell dirname $(MAKEFILE_PATH))
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

.PHONY: venv
venv: venv-osarch-specific


.PHONY: dependencies-for-development
dependencies-for-development: venv dependencies-for-development-osarch-specific
	$(activate-venv); \
		python3 -m pip install --upgrade pip; \
		python3 -m pip install --requirement development-requirements.txt
	@go install golang.org/x/tools/cmd/godoc@latest
	@go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
	@go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
	@sudo pecl channel-update pecl.php.net
	@sudo pecl install grpc


dependencies-for-nodejs:
	@npm install -g grpc-tools
	@npm install -D @grpc/grpc-js @grpc/proto-loader grpc_tools_node_protoc_ts typescript tsx


.PHONY: dependencies
dependencies: venv
	$(activate-venv); \
		python3 -m pip install --upgrade pip; \
		python3 -m pip install --requirement requirements.txt
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
	OUTPUT_DIR=example_generated_source_code/java/; \
	mkdir -p $${OUTPUT_DIR}; \
	for SENZING_COMPONENT in $(SENZING_COMPONENTS); do \
		protoc --java_out=$${OUTPUT_DIR}  $${SENZING_COMPONENT}.proto; \
	done


.PHONY: generate-nodejs
generate-nodejs:
	for SENZING_COMPONENT in $(SENZING_COMPONENTS); do \
		OUTPUT_DIR=example_generated_source_code/nodejs/$${SENZING_COMPONENT}; \
		PROTO_DIR=example_generated_source_code/nodejs/proto; \
		mkdir -p $${OUTPUT_DIR}; \
		mkdir -p $${PROTO_DIR}; \
		cp -r $${SENZING_COMPONENT}.proto $${PROTO_DIR}/; \
		grpc_tools_node_protoc \
			--js_out=import_style=commonjs,binary:$${OUTPUT_DIR} \
			--ts_out=import_style=commonjs,binary:$${OUTPUT_DIR} \
			--grpc_out=grpc_js:$${OUTPUT_DIR} \
			$${SENZING_COMPONENT}.proto; \
	done


.PHONY: generate-php
generate-php:
	for SENZING_COMPONENT in $(SENZING_COMPONENTS); do \
		OUTPUT_DIR=example_generated_source_code/php/$${SENZING_COMPONENT}; \
		mkdir -p $${OUTPUT_DIR}; \
		protoc --proto_path=. --php_out=$${OUTPUT_DIR}  --grpc_out=$${OUTPUT_DIR}  --plugin=protoc-gen-grpc=`which grpc_php_plugin`  $${SENZING_COMPONENT}.proto; \
	done


.PHONY: generate-python
generate-python:
	$(activate-venv); \
	OUTPUT_DIR=python/src/senzing_grpc_protobuf; \
	mkdir -p $${OUTPUT_DIR}; \
	for SENZING_COMPONENT in $(SENZING_COMPONENTS); do \
		python -m grpc_tools.protoc --proto_path=. --python_out=$${OUTPUT_DIR} --pyi_out=$${OUTPUT_DIR} --grpc_python_out=$${OUTPUT_DIR} $${SENZING_COMPONENT}.proto; \
	done


.PHONY: generate-ruby
generate-ruby:
	for SENZING_COMPONENT in $(SENZING_COMPONENTS); do \
		OUTPUT_DIR=example_generated_source_code/ruby/$${SENZING_COMPONENT}; \
		mkdir -p $${OUTPUT_DIR}; \
		grpc_tools_ruby_protoc --proto_path=. --ruby_out=$${OUTPUT_DIR}  --grpc_out=$${OUTPUT_DIR}  $${SENZING_COMPONENT}.proto; \
	done


# all typescript compilers follow short name abbr name convention
.PHONY: generate-ts
generate-ts: generate-typescript
generate-typescript:
	for SENZING_COMPONENT in $(SENZING_COMPONENTS); do \
		OUTPUT_DIR=example_generated_source_code/ts/$${SENZING_COMPONENT}; \
		PROTO_DIR=example_generated_source_code/ts/proto; \
		mkdir -p $${OUTPUT_DIR}; \
		mkdir -p $${PROTO_DIR}; \
		cp -r $${SENZING_COMPONENT}.proto $${PROTO_DIR}/; \
		grpc_tools_node_protoc \
			--js_out=import_style=commonjs,binary:$${OUTPUT_DIR} \
			--ts_out=import_style=commonjs,binary:$${OUTPUT_DIR} \
			--grpc_out=grpc_js:$${OUTPUT_DIR} \
			$${SENZING_COMPONENT}.proto; \
	done


.PHONY: generate-web
generate-web:
	for SENZING_COMPONENT in $(SENZING_COMPONENTS); do \
		OUTPUT_DIR=example_generated_source_code/web/$${SENZING_COMPONENT}; \
		PROTO_DIR=example_generated_source_code/web/proto; \
		mkdir -p $${OUTPUT_DIR}; \
		mkdir -p $${PROTO_DIR}; \
		cp -r $${SENZING_COMPONENT}.proto $${PROTO_DIR}/; \
		grpc_tools_node_protoc \
			--js_out=import_style=commonjs,binary:$${OUTPUT_DIR} \
			--grpc-web_out=import_style=typescript,mode=grpcwebtext:$${OUTPUT_DIR} \
			$${SENZING_COMPONENT}.proto; \
	done

# -----------------------------------------------------------------------------
# Documentation
# -----------------------------------------------------------------------------

.PHONY: documentation
documentation: documentation-osarch-specific

# -----------------------------------------------------------------------------
# Package
# -----------------------------------------------------------------------------

.PHONY: package
package: package-osarch-specific

# -----------------------------------------------------------------------------
# Clean
# -----------------------------------------------------------------------------

.PHONY: clean
clean: clean-csharp clean-go clean-java clean-php clean-python clean-ruby


.PHONY: clean-csharp
clean-csharp:
	@rm -rf $(MAKEFILE_DIRECTORY)/example_generated_source_code/cpp/* || true


.PHONY: clean-go
clean-go:
	@rm -rf $(MAKEFILE_DIRECTORY)/go/szconfig/sz* || true
	@rm -rf $(MAKEFILE_DIRECTORY)/go/szconfigmanager/sz* || true
	@rm -rf $(MAKEFILE_DIRECTORY)/go/szdiagnostic/sz* || true
	@rm -rf $(MAKEFILE_DIRECTORY)/go/szengine/sz* || true
	@rm -rf $(MAKEFILE_DIRECTORY)/go/szproduct/sz* || true


.PHONY: clean-java
clean-java:
	@rm -rf $(MAKEFILE_DIRECTORY)/example_generated_source_code/java/* || true


.PHONY: clean-node
clean-node: clean-nodejs
clean-nodejs:
	@rm -rf $(MAKEFILE_DIRECTORY)/example_generated_source_code/nodejs/* || true


.PHONY: clean-php
clean-php:
	@rm -rf $(MAKEFILE_DIRECTORY)/example_generated_source_code/php/* || true


.PHONY: clean-python
clean-python:
	@rm -rf $(MAKEFILE_DIRECTORY)/python/src/senzing_grpc_protobuf/szconfig*        || true
	@rm -rf $(MAKEFILE_DIRECTORY)/python/src/senzing_grpc_protobuf/szconfigmanager* || true
	@rm -rf $(MAKEFILE_DIRECTORY)/python/src/senzing_grpc_protobuf/szdiagnostic*    || true
	@rm -rf $(MAKEFILE_DIRECTORY)/python/src/senzing_grpc_protobuf/szengine*        || true
	@rm -rf $(MAKEFILE_DIRECTORY)/python/src/senzing_grpc_protobuf/szproduct*       || true


.PHONY: clean-ruby
clean-ruby:
	@rm -rf $(MAKEFILE_DIRECTORY)/example_generated_source_code/ruby/* || true


.PHONY: clean-ts
clean-ts: clean-typescript
clean-typescript:
	@rm -rf $(MAKEFILE_DIRECTORY)/example_generated_source_code/ts/* || true


.PHONY: clean-web
clean-web:
	@rm -rf $(MAKEFILE_DIRECTORY)/example_generated_source_code/web/* || true

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
