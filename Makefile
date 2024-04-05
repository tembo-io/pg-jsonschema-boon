PG_CONFIG   ?= $(shell which pg_config)
PGRXV="$(shell perl -nE '/^pgrx\s+=\s"=?([^"]+)/ && do { say $$1; exit }' Cargo.toml)"
PGV=$(shell perl -E 'shift =~ /(\d+)/ && say $$1' "$(shell $(PG_CONFIG) --version)")

.DEFAULT_GOAL: package # Build jsonshcmea for the PostgreSQL cluster identified by pg_config.
package:
	@cargo pgrx package --pg-config "$(PG_CONFIG)"

.PHONY: install # Install jsonschema into the PostgreSQL cluster identified by pg_config.
install:
	@cargo pgrx install --release --pg-config "$(PG_CONFIG)"

.PHONY: test # Run the full test suite against the PostgreSQL version identified by pg_config.
test:
	@cargo test --all --no-default-features --features "pg$(PGV) pg_test" -- --nocapture

.PHONY: install-check # An alias for the test target for PGXS compatability.
install-check: test

.PHONY: cover # Run cover tests and generate & open a report.
cover:
	@./.ci/test-cover "$(PGV)" "$(PGRXV)"

.PHONY: pg-version # Print the current PGRX version from Cargo.toml
pgrx-version:
	@echo $(PGRXV)

.PHONY: pg-version # Print the current Postgres version reported by pg_config.
pg-version: Cargo.toml
	@echo $(PGV)

## cleaan: Remove build artifacts and intermediate files.
clean: target
	@cargo clean
