# Makefile extensions for linux.

# -----------------------------------------------------------------------------
# OS specific targets
# -----------------------------------------------------------------------------

.PHONY: dependencies-for-development-osarch-specific
dependencies-for-development-osarch-specific:
	@protoc --version
	@sudo apt install --upgrade -y cmake protobuf-compiler php-pear php-dev ruby-grpc-tools
	@protoc --version


.PHONY: documentation-osarch-specific
documentation-osarch-specific:
	@pkill godoc || true
	@godoc &
	@xdg-open http://localhost:6060


.PHONY: package-osarch-specific
package-osarch-specific:
	@$(activate-venv); python3 -m build

.PHONY: venv-osarch-specific
venv-osarch-specific:
	@python3 -m venv .venv

# -----------------------------------------------------------------------------
# Makefile targets supported only by this platform.
# -----------------------------------------------------------------------------

.PHONY: only-linux
only-linux:
	$(info Only linux has this Makefile target.)
