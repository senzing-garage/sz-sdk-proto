# Makefile extensions for linux.

# -----------------------------------------------------------------------------
# OS specific targets
# -----------------------------------------------------------------------------

.PHONY: clean-osarch-specific
clean-osarch-specific:
	@pkill godoc || true


.PHONY: dependencies-for-development-osarch-specific
dependencies-for-development-osarch-specific:
	@protoc --version
	@sudo apt install --upgrade -y protobuf-compiler
	@protoc --version


.PHONY: documentation-osarch-specific
documentation-osarch-specific:
	@godoc &
	@xdg-open http://localhost:6060

# -----------------------------------------------------------------------------
# Makefile targets supported only by this platform.
# -----------------------------------------------------------------------------

.PHONY: only-linux
only-linux:
	$(info Only linux has this Makefile target.)
