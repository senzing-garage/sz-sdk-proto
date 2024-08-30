# Makefile extensions for linux.

# -----------------------------------------------------------------------------
# OS specific targets
# -----------------------------------------------------------------------------

.PHONY: dependencies-for-development-osarch-specific
dependencies-for-development-osarch-specific:
	protoc --version
	apt install -y protobuf-compiler
	protoc --version

# -----------------------------------------------------------------------------
# Makefile targets supported only by this platform.
# -----------------------------------------------------------------------------

.PHONY: only-linux
only-linux:
	$(info Only linux has this Makefile target.)
