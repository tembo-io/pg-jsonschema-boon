# Changelog

All notable changes to this project will be documented in this file. It uses the
[Keep a Changelog] format, and this project adheres to [Semantic Versioning].

  [Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
  [Semantic Versioning]: https://semver.org/spec/v2.0.0.html
    "Semantic Versioning 2.0.0"

## [v0.1.1] --- Unreleased

The theme of this release is *fast follows.*

### ⬆️ Dependency Updates

*   Updated boon to v0.6.

### ⚡ Improvements

*   Removed custom schema loader, taking advantage of the feature of boon v0.6
    to remove all loaders (santhosh-tekuri/boon#12).

## [v0.1.0] --- 2024-04-30

The theme of this release is *learning Rust and pgrx.*

### ⚡ Improvements

*   First release, everything is new!
*   JSON Schema validation using [boon]
*   Fully supports draft 2020-12, draft 2019-09, draft-7, draft-6, and draft-4
*   Multi-object schema composition
*   Remote fetching of resources disabled

### 🏗️ Build Setup

*   Built with Rust
*   Use `make` for most actions
*   Download from [PGXN] or [GitHub]
*   [CI testing] on PostgreSQL 11--16

### 📚 Documentation

*   Build and install docs in the [README]
*   Full [reference documentation]
*   Performance [benchmark script]
*   Multi-schema [composition example]

  [v0.1.0]: https://github.com/tembo-io/pg-jsonschema-boon/compare/34d5d49...HEAD
  [boon]: https://github.com/santhosh-tekuri/boon
  [README]: https://github.com/tembo-io/pg-jsonschema-boon/blob/v0.1.0/README.md
  [PGXN]: https://pgxn.org/dist/jsonschema/
  [GitHub]: https://github.com/tembo-io/pg-jsonschema-boon/releases
  [reference documentation]: https://github.com/tembo-io/pg-jsonschema-boon/blob/v0.1.0/doc/jsonschema.md
  [benchmark script]: https://github.com/tembo-io/pg-jsonschema-boon/blob/v0.1.0/eg/bench.sql
  [composition example]: https://github.com/tembo-io/pg-jsonschema-boon/blob/v0.1.0/eg/user.sql
  [CI testing]: https://github.com/tembo-io/pg-jsonschema-boon/actions/workflows/lint-and-test.yml
