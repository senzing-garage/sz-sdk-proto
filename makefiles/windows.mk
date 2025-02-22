# Makefile extensions for windows.

# -----------------------------------------------------------------------------
# OS specific targets
# -----------------------------------------------------------------------------

.PHONY: documentation-osarch-specific
documentation-osarch-specific:
	@taskkill /f /t/im godoc
	@start /b godoc
	@explorer http://localhost:6060


.PHONY: venv-osarch-specific
venv-osarch-specific:
	@python -m venv .venv

# -----------------------------------------------------------------------------
# Makefile targets supported only by this platform.
# -----------------------------------------------------------------------------

.PHONY: only-windows
only-windows:
	$(info Only windows has this Makefile target.)
