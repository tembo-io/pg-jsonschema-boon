JSON Schema Postgres Extension
==============================

[![MIT License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT "⚖️ MIT License")
[![PGXN Version](https://badge.fury.io/pg/jsonschema.svg)](https://badge.fury.io/pg/jsonschema "⚙️ PGXN Version")
[![Build Status](https://github.com/tembo-io/pg-jsonschema-boon/actions/workflows/lint-and-test.yml/badge.svg)](https://github.com/tembo-io/pg-jsonschema-boon/actions/workflows/lint-and-test.yml "🧪 Lint and Test")
[![Code Coverage](https://codecov.io/gh/tembo-io/pg-jsonschema-boon/graph/badge.svg?token=DIFED324ZY)](https://codecov.io/gh/tembo-io/pg-jsonschema-boon "📊 Code Coverage")
[![Dependency Status](https://deps.rs/repo/github/tembo-io/pg-jsonschema-boon/status.svg)](https://deps.rs/repo/github/tembo-io/pg-jsonschema-boon "📦 Dependency Status")

**[Change Log](CHANGELOG.md)** | **[Documentation](doc/jsonschema.md)**

This package provides the `jsonschema` extension for validating JSON and JSONB
against a [JSON Schema] in Postgres. It relies on the [boon] crate, and
therefore supports the following [specification drafts] as validated by the
[JSON-Schema-Test-Suite] excluding optional features:

*   [![draft 4 badge]][draft 4 report]
*   [![draft 6 badge]][draft 6 report]
*   [![draft 7 badge]][draft 7 report]
*   [![draft 2019-09 badge]][draft 2019-09 report]
*   [![draft 2020-12 badge]][draft 2020-12 report]

Installation
------------

The jsonschema extension is written in [Rust], using the [boon] JSON Schema
validation library, and requires the Rust toolchain and [pgrx] to build. The
simplest way to install Rust is [rustup]:

``` sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then install `pgrx`:

```sh
make install-pgrx
```

Now build and jsonschema against a working PostgreSQL server, including
development libraries and [pg_config], which must be in the path:

``` sh
make
make install
```

To build with a different `pg_config`, pass it to `make`:

``` sh
make PG_CONFIG=/path/to_pg_config
make install PG_CONFIG=/path/to_pg_config
```

Once jsonschema is installed, you can add it to a database. Simply connect to
a database as a super user and run the [CREATE EXTENSION] command:

``` postgres
CREATE EXTENSION jsonschema;
```

If you want to install jsonschema into a specific schema, `WITH SCHEMA`:

``` postgres
CREATE EXTENSION jsonschema WITH SCHEMA extensions;
```

See [the documentation](./doc/jsonschema.md) for usage details and features.

Dependencies
------------

The `jsonschema` data type has no run-time dependencies other than PostgreSQL.
At build time it requires [Rust] and [pgrx].

Prior Art
---------

*   [pg_jsonschema]: JSON Schema Postgres extension written with pgrx +
    the [jsonschema crate]
*   [pgx_json_schema]: Slightly older JSON Schema Postgres extension written
    with pgrx + the [jsonschema crate]
*   [postgres-json-schema]: JSON Schema Postgres extension written in PL/pgSQL
*   [is_jsonb_valid]: JSON Schema Postgres extension written in C

Benchmark
---------

A quick benchmark in [`eg/bench.sql`](eg/bench.sql) compares the performance
for a simple validation a check constraint between the jsonschema and
[pg_jsonschema]. Example testing `jsonschema` with PostgreSQL 16 on an M3 Max
MacBook Pro with 32G of RAM:

``` console
$ psql -f eg/bench.sql -X --set extension=jsonschema

######################################################################
# Test jsonschema JSON validation for 200_000 iterations
######################################################################
Time: 2686.546 ms (00:02.687)

######################################################################
# Test jsonschema JSONB validation for 200_000 iterations
######################################################################
Time: 2643.178 ms (00:02.643)
```

Testing [pg_jsonschema]:

``` console
$ psql -f eg/bench.sql -X --set extension=pg_jsonschema

######################################################################
# Test pg_jsonschema JSON validation for 200_000 iterations
######################################################################
Time: 1855.604 ms (00:01.856)

######################################################################
# Test pg_jsonschema JSONB validation for 200_000 iterations
######################################################################
Time: 1834.598 ms (00:01.835)
```

And a control test with no validation:

``` console
$ psql -f eg/bench.sql -X

######################################################################
# Test without JSON validation for 200_000 iterations
######################################################################
Time: 668.716 ms

######################################################################
# Test without JSONB validation for 200_000 iterations
######################################################################
Time: 741.202 ms
```

Copyright and License
---------------------

Copyright (c) 2024 Tembo

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

  [JSON Schema]: https://json-schema.org
  [specification drafts]: https://json-schema.org/specification
  [JSON-Schema-Test-Suite]: https://github.com/json-schema-org/JSON-Schema-Test-Suite
  [draft 4 badge]: https://img.shields.io/endpoint?url=https://bowtie.report/badges/rust-boon/compliance/draft4.json
  [draft 4 report]: https://bowtie.report/#/dialects/draft4 "boon draft 4 report"
  [draft 6 badge]: https://img.shields.io/endpoint?url=https://bowtie.report/badges/rust-boon/compliance/draft6.json
  [draft 6 report]: https://bowtie.report/#/dialects/draft6 "boon draft 6 report"
  [draft 7 badge]: https://img.shields.io/endpoint?url=https://bowtie.report/badges/rust-boon/compliance/draft7.json
  [draft 7 report]: https://bowtie.report/#/dialects/draft7 "boon draft 7 report"
  [draft 2019-09 badge]: https://img.shields.io/endpoint?url=https://bowtie.report/badges/rust-boon/compliance/draft2019-09.json
  [draft 2019-09 report]: https://bowtie.report/#/dialects/draft2019-09 "boon draft 2019-09 report"
  [draft 2020-12 badge]: https://img.shields.io/endpoint?url=https://bowtie.report/badges/rust-boon/compliance/draft2020-12.json
  [draft 2020-12 report]: https://bowtie.report/#/dialects/draft2020-12 "boon draft 2020-12 report"
  [boon]: https://github.com/santhosh-tekuri/boon/ "boon: JSONSchema (draft 2020-12, draft 2019-09, draft-7, draft-6, draft-4) Validation in Rust"
  [pgrx]: https://github.com/pgcentralfoundation/pgrx "pgrx: Build Postgres Extensions with Rust!"
  [PostgreSQL]: https://postgresql.org "PostgreSQL: The World's Most Advanced Open Source Relational Database"
  [rustup]: https://rustup.rs "rustup is an installer for Rust"
  [Rust]: https://www.rust-lang.org/ "Rust: A language empowering everyone to build reliable and efficient software"
  [pg_config]: https://www.postgresql.org/docs/current/app-pgconfig.html "PostgreSQL Docs: pg_config"
  [CREATE EXTENSION]: https://www.postgresql.org/docs/current/sql-createextension.html
    "PostgreSQL Docs: CREATE EXTENSION"
  [jsonschema crate]: https://docs.rs/jsonschema/latest/jsonschema/
  [pg_jsonschema]: https://github.com/supabase/pg_jsonschema
  [postgres-json-schema]: https://github.com/gavinwahl/postgres-json-schema
  [is_jsonb_valid]: https://github.com/furstenheim/is_jsonb_valid
  [pgx_json_schema]: https://github.com/jefbarn/pgx_json_schema
