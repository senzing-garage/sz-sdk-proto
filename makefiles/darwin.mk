# Makefile extensions for darwin.

# -----------------------------------------------------------------------------
# OS specific targets
# -----------------------------------------------------------------------------

.PHONY: documentation-osarch-specific
documentation-osarch-specific:
	@pkill godoc || true
	@godoc &
	@open http://localhost:6060


.PHONY: venv-osarch-specific
venv-osarch-specific:
	@python3 -m venv .venv

# -----------------------------------------------------------------------------
# Makefile targets supported only by this platform.
# -----------------------------------------------------------------------------

.PHONY: only-darwin
only-darwin:
	$(info Only darwin has this Makefile target.)
