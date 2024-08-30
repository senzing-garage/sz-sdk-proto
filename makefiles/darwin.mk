# Makefile extensions for darwin.

# -----------------------------------------------------------------------------
# OS specific targets
# -----------------------------------------------------------------------------

.PHONY: clean-osarch-specific
clean-osarch-specific:
	@pkill godoc || true


.PHONY: documentation-osarch-specific
documentation-osarch-specific:
	@godoc &
	@open http://localhost:6060

# -----------------------------------------------------------------------------
# Makefile targets supported only by this platform.
# -----------------------------------------------------------------------------

.PHONY: only-darwin
only-darwin:
	$(info Only darwin has this Makefile target.)
