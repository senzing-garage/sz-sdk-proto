# Makefile extensions for windows.

# -----------------------------------------------------------------------------
# OS specific targets
# -----------------------------------------------------------------------------

.PHONY: clean-osarch-specific
clean-osarch-specific:
	@taskkill /f /t/im godoc


.PHONY: documentation-osarch-specific
documentation-osarch-specific:
	@start /b godoc
	@explorer http://localhost:6060

# -----------------------------------------------------------------------------
# Makefile targets supported only by this platform.
# -----------------------------------------------------------------------------

.PHONY: only-windows
only-windows:
	$(info Only windows has this Makefile target.)
