PG_CONFIG   ?= $(shell which pg_config)
DISTNAME     = $(shell perl -nE '/^name\s*=\s*"([^"]+)/ && do { say $$1; exit }' Cargo.toml)
DISTVERSION  = $(shell perl -nE '/^version\s*=\s*"([^"]+)/ && do { say $$1; exit }' Cargo.toml)
PGRXV        = $(shell perl -nE '/^pgrx\s+=\s"=?([^"]+)/ && do { say $$1; exit }' Cargo.toml)
PGV          = $(shell perl -E 'shift =~ /(\d+)/ && say $$1' "$(shell $(PG_CONFIG) --version)")

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

.PHONY: install-pgrx # Install the version of PGRX specified in Cargo.toml.
install-pgrx: Cargo.toml
	@cargo install --locked cargo-pgrx --version "$(PGRXV)"

## clean: Remove build artifacts and intermediate files.
clean: target
	@cargo clean
	@rm -rf META.json $(DISTNAME)-$(DISTVERSION).zip

# Create the PGXN META.json file.
META.json: META.json.in Cargo.toml
	@sed "s/@CARGO_VERSION@/$(DISTVERSION)/g" $< > $@

# Create a PGXN-compatible zip file.
$(DISTNAME)-$(DISTVERSION).zip: META.json
	git archive --format zip --prefix $(DISTNAME)-$(DISTVERSION)/ --add-file $< -o $(DISTNAME)-$(DISTVERSION).zip HEAD

## pgxn-zip: Create a PGXN-compatible zip file.
pgxn-zip: $(DISTNAME)-$(DISTVERSION).zip

target/release-notes.md: CHANGELOG.md .ci/mknotes
	@./.ci/mknotes -v $(DISTVERSION) -f $< -r https://github.com/$(or $(GITHUB_REPOSITORY),tembo-io/pg-jsonschema) -o $@
