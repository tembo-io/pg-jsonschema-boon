PG_CONFIG   ?= $(shell which pg_config)
DISTNAME     = $(shell perl -nE '/^name\s*=\s*"([^"]+)/ && do { say $$1; exit }' Cargo.toml)
DISTVERSION  = $(shell perl -nE '/^version\s*=\s*"([^"]+)/ && do { say $$1; exit }' Cargo.toml)
PGRXV        = $(shell perl -nE '/^pgrx\s+=\s"=?([^"]+)/ && do { say $$1; exit }' Cargo.toml)
PGV          = $(shell perl -E 'shift =~ /(\d+)/ && say $$1' "$(shell $(PG_CONFIG) --version)")
EXTRA_CLEAN  = META.json $(DISTNAME)-$(DISTVERSION).zip target
TESTS        = $(wildcard test/sql/*.sql)
REGRESS      = $(patsubst test/sql/%.sql,%,$(TESTS))
REGRESS_OPTS = --inputdir=test --load-extension=$(DISTNAME) --outputdir=target/installcheck

PGXS := $(shell $(PG_CONFIG) --pgxs)
include $(PGXS)

all: package

.DEFAULT_GOAL: package # Build jsonschema for the PostgreSQL cluster identified by pg_config.
package:
	@cargo pgrx package --pg-config "$(PG_CONFIG)"

.PHONY: install # Install jsonschema into the PostgreSQL cluster identified by pg_config.
install:
	@cargo pgrx install --release --pg-config "$(PG_CONFIG)"

.PHONY: test # Run the full test suite against the PostgreSQL version identified by pg_config.
test:
	@cargo test --all --no-default-features --features "pg$(PGV) pg_test" -- --nocapture

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

.PHONY: pgrx-init # Initialize pgrx for the PostgreSQL version identified by pg_config.
pgrx-init: Cargo.toml
	@cargo pgrx init "--pg$(PGV)"="$(PG_CONFIG)"

.PHONY: lint # Format and lint.
lint:
	@cargo fmt --all --check
	@cargo clippy --features "pg$(PGV)" --no-default-features

# Create the PGXN META.json file.
META.json: META.json.in Cargo.toml
	@sed "s/@CARGO_VERSION@/$(DISTVERSION)/g" $< > $@

# Expected test output regeneration
target/installcheck/results/%.out: installcheck
test/expected/%.out: target/installcheck/results/%.out
	@cp $^ $@

# Create a PGXN-compatible zip file.
$(DISTNAME)-$(DISTVERSION).zip: META.json
	git archive --format zip --prefix $(DISTNAME)-$(DISTVERSION)/ --add-file $< -o $(DISTNAME)-$(DISTVERSION).zip HEAD

## pgxn-zip: Create a PGXN-compatible zip file.
pgxn-zip: $(DISTNAME)-$(DISTVERSION).zip

target/release-notes.md: CHANGELOG.md .ci/mknotes
	@./.ci/mknotes -v $(DISTVERSION) -f $< -r https://github.com/$(or $(GITHUB_REPOSITORY),tembo-io/pg-jsonschema-boon) -o $@

## vendor: Vendor all crates.io and git dependencies.
vendor:
	@cargo vendor
