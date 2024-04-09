# Changelog

All notable changes to this project will be documented in this file. It uses the
[Keep a Changelog] format, and this project adheres to [Semantic Versioning].

  [Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
  [Semantic Versioning]: https://semver.org/spec/v2.0.0.html
    "Semantic Versioning 2.0.0"

## [v0.1.0] --- Unreleased

The theme of this release is *learning Rust and pgrx.*

### ⚡ Improvements

*   First release, everything is new!
*   JSON Schema validation using [boon]
*   Fully supports draft 2020-12, draft 2019-09, draft-7, draft-6, and draft-4
*   Multi-object schema compsition

### 🏗️ Build Setup

*   Built with Rust
*   Use `make` for most actions
*   Download from [PGXN] or [GitHub]

### 📚 Documentation

*   Build and install docs in the [README]
*   Full [reference documentation]
*   Performance [benchmark script]
*   Multi-schema [composition example]

  [v0.1.0]: https://github.com/tembo-io/pg-jsonschema/compare/34d5d49...HEAD
  [boon]: https://github.com/santhosh-tekuri/boon
  [README]: https://github.com/tembo-io/pg-jsonschema/blob/v0.1.0/README.md
  [PGXN]: https://pgxn.org/dist/jsonschema/
  [GitHub]: https://github.com/tembo-io/pg-jsonschema/releases
  [reference documentation]: https://github.com/tembo-io/pg-jsonschema/blob/v0.1.0/doc/jsonschema.md
  [benchmark script]: https://github.com/tembo-io/pg-jsonschema/blob/v0.1.0/eg/bench.sql
  [composition example]: https://github.com/tembo-io/pg-jsonschema/blob/v0.1.0/eg/user.sql
