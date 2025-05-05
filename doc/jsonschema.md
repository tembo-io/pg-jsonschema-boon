jsonschema
==========

Synopsis
--------

```psql
try=# -- install the extension
try=# CREATE EXTENSION jsonschema;
CREATE EXTENSION

try=# -- Define a JSON schema
try=# SELECT '{
   "type": "object",
   "required": [ "name", "email" ],
   "properties": {
     "name": { "type": "string" },
     "age": { "type": "number", "minimum": 0 },
     "email": {"type": "string", "format": "email" }
   }
 }' AS schema \gset

try=# -- Make sure it's valid
try=# SELECT jsonschema_is_valid(:'schema'::json);
 jsonschema_is_valid
---------------------
 t

try=# -- Define an object to validate
try=# SELECT '{
   "name": "Amos Burton",
   "email": "amos@rocinante.ship"
 }' AS person \gset

try=# -- Validate it against the schema
try=# SELECT jsonschema_validates(:'person'::json, :'schema'::json);
 jsonschema_validates
----------------------
 t
```

Description
-----------

This extension adds two functions (with varying signatures) to Postgres:
`jsonschema_is_valid()`, which validates a [JSON Schema] against the
meta-schemas that define the [JSON specification][spec], and
`jsonschema_validates()`, which validates a JSON value against a JSON Schema.
It supports the following [specification drafts][spec]:

*   [![draft 4 badge]][draft 4 report]
*   [![draft 6 badge]][draft 6 report]
*   [![draft 7 badge]][draft 7 report]
*   [![draft 2019-09 badge]][draft 2019-09 report]
*   [![draft 2020-12 badge]][draft 2020-12 report]

What, Another One?
------------------

This is not the first JSON Schema validation extension for Postgres. Why
another one? (Let's just ignore that the work was pretty far along before
learning of the prior art.) What's different about this extension compared to
the [prior art](#prior-art)?

*   **Full 2020-12 draft compatibility.** The [boon crate] used by jsonschema
    is the only Rust crate that fully supports the JSON Schema [2020-12
    draft].
*   **Complex Schema Composition.** While the existing extensions nicely
    support simple, single-file JSON Schemas, this extension fully supports
    [multi-file schema definition][composition]. This allows multiple smaller
    schemas to be composed into larger, more complicated schemas.
*   **Performance.** The use of Rust provides the highest level of schema
    validation performance. In a [simple test], the jsonschema extension
    validates JSON and JSONB objects in a `CHECK` constraint at around 77,000
    inserts/second.

For many use cases these features aren't necessary. But once schemas become
more complicated or require more advanced features of the [2020-12 draft],
give the jsonschema extension a try.

Schema Composition
------------------

Schema composition enables componentized structuring of a schema across
multiple sub-schemas. JSON Schema refers to this pattern as [Structuring a
complex schema][composition]. The idea is that each schema has a URI in an
`$id` property that identifiers it, and other schemas can reference it.

For example, an address schema defined like so:

``` json
{
  "$id": "https://example.com/schemas/address",
  "type": "object",
  "properties": {
    "street_address": { "type": "string" },
    "city": { "type": "string" },
    "state": { "type": "string" }
  },
  "required": ["street_address", "city", "state"]
}
```

Can be referenced in another schema by using the `$ref` keyword. This is handy
to avoid duplication, as in this example:

``` json
{
  "$id": "https://example.com/schemas/customer",
  "type": "object",
  "properties": {
    "first_name": { "type": "string" },
    "last_name": { "type": "string" },
    "shipping_address": { "$ref": "/schemas/address" },
    "billing_address": { "$ref": "/schemas/address" }
  },
  "required": ["first_name", "last_name", "shipping_address", "billing_address"]
}
```

Note that `shipping_address` and `billing_address` both reference
`/schemas/address`. This URI is resolved against the schema `$id`,
`https://example.com/schemas/customer`, which is the base URI for the schema.
Relative to that URI, `/schemas/address` becomes
`https://example.com/schemas/address` resolving to the address schema's `$id`.

The jsonschema extension supports this pattern by allowing multiple schemas to
be passed to its functions. Say we have the above two schemas loaded into
[psql] variables (you can paste these commands into `psql` to refer to in the
example below):

``` psql
SELECT '{
  "$id": "https://example.com/schemas/address",
  "type": "object",
  "properties": {
    "street_address": { "type": "string" },
    "city": { "type": "string" },
    "state": { "type": "string" }
  },
  "required": ["street_address", "city", "state"]
}'  AS addr_schema \gset

SELECT '{
  "$id": "https://example.com/schemas/customer",
  "type": "object",
  "properties": {
    "first_name": { "type": "string" },
    "last_name": { "type": "string" },
    "shipping_address": { "$ref": "/schemas/address" },
    "billing_address": { "$ref": "/schemas/address" }
  },
  "required": ["first_name", "last_name", "shipping_address", "billing_address"]
}' AS cust_schema \gset
```

We validate the customer schema by its ID and passing both schemas to
`jsonschema_is_valid()`:

```psql
SELECT jsonschema_is_valid(
    'https://example.com/schemas/customer',
    :'addr_schema'::json, :'cust_schema'::json
);
 jsonschema_is_valid
---------------------
 t
```

Any number of schemas can be passed, allowing for arbitrarily complex schemas.
The same is true for validating JSON values against a composed schema:

```psql
SELECT jsonschema_validates(
    json_build_object(
      'first_name', 'Naomi',
      'last_name', 'Nagata',
      'shipping_address', json_build_object(
        'street_address', '1 Rocinante Way',
        'city', 'Ceres Station',
        'state', 'The Belt'
      ),
      'billing_address', json_build_object(
        'street_address', '2112 Rush Ave',
        'city', 'Londres Nova',
        'state', 'Mars'
      )
    ),
    'https://example.com/schemas/customer',
    :'addr_schema'::json, :'cust_schema'::json
);
 jsonschema_validates
----------------------
 t
```

Of course, if your build pipeline supports it you can also [bundle] all of the
sub-schemas required to compose a schema and then just have the one, with no
need to refer to it by `$id`. For example,

```psql
SELECT '{
  "$id": "https://example.com/schemas/customer",
  "type": "object",
  "properties": {
    "first_name": { "type": "string" },
    "last_name": { "type": "string" },
    "shipping_address": { "$ref": "/schemas/address" },
    "billing_address": { "$ref": "/schemas/address" }
  },
  "required": ["first_name", "last_name", "shipping_address", "billing_address"],
  "$defs": {
    "https://example.com/schemas/address": {
      "$id": "https://example.com/schemas/address",
      "type": "object",
      "properties": {
        "street_address": { "type": "string" },
        "city": { "type": "string" },
        "state": { "type": "string" }
      },
      "required": ["street_address", "city", "state"]
    }
  }
}' AS cust_schema \gset
```

Note that the only change to the address schema is tha it has been embedded in
the `$defs` object using its `$id` for the key. With this schema bundle, we
can omit the `id` parameter:

```psql
SELECT jsonschema_is_valid(:'cust_schema'::json);
 jsonschema_is_valid
---------------------
 t

SELECT jsonschema_validates(
    json_build_object(
      'first_name', 'Naomi',
      'last_name', 'Nagata',
      'shipping_address', json_build_object(
        'street_address', '1 Rocinante Way',
        'city', 'Ceres Station',
        'state', 'The Belt'
      ),
      'billing_address', json_build_object(
        'street_address', '2112 Rush Ave',
        'city', 'Londres Nova',
        'state', 'Mars'
      )
    ),
    :'cust_schema'::json
);
 jsonschema_validates
----------------------
 t
 ```

Configuration
-------------

The jsonschema extension fully supports all of the drafts of the [spec], but a
schema is not required to identify its draft (as in the
[synopsis](#synopsis)). When none is specified, jsonschema defaults to the
latest draft, 2020-12. If, however, you need it to default to some other draft
(or ensure consistency of behavior should a new draft be released and become
the default), set the `jsonschema.default_draft` configuration to your
preferred default. To set the default for the current session to 2019-09, run:

``` postgres
SET jsonschema.default_draft TO 'V2019';
```

For a system-wide default, set it in the `postgresql.conf` file:

```ini
jsonschema.default_draft = 'V7'
```

The supported values are:

*   `V4`: Draft for `http://json-schema.org/draft-04/schema`
*   `V6`: Draft for `http://json-schema.org/draft-06/schema`
*   `V7`: Draft for `http://json-schema.org/draft-07/schema`
*   `V2019`: Draft for `https://json-schema.org/draft/2019-09/schema`
*   `V2020`: Draft for `https://json-schema.org/draft/2020-12/schema`

Functions
---------

### `jsonschema_is_valid(schema)` ###

```postgres
SELECT jsonschema_is_valid(schema::json);
SELECT jsonschema_is_valid(schema::jsonb);
```

**Parameters**

*   `schema`: A JSON Schema in a JSON or JSONB value

This function verifies that a JSON schema is valid against the JSON Schema
spec. Returns true if the schema validates. If `schema` does not have a
[`$schema` field], it will be validated against the draft defined by the
[`jsonschema.default_draft`](#configuration) configuration or, if it's not
defined, the latest draft, currently 2020-12.

If `schema` has no [`$id` field], the function will refer to it as
`file:///schema.json` in error messages.

Returns false if `schema` is invalid, or does not compile, logging the
reason to at the `INFO` level.

### `jsonschema_is_valid(id, schema)` ###

```postgres
SELECT jsonschema_is_valid(id::text, VARIADIC schema::json);
SELECT jsonschema_is_valid(id::text, VARIADIC schema::jsonb);
```

**Parameters**

*   `id`: The ID of the schema to validate
*   `schema`: A list JSON Schemas in JSON or JSONB values

This function verifies that the JSON schema with the [`$id` field]
corresponding to the `id` parameter is valid against the JSON Schema spec.
Returns true if the schema validates.

In general, each `schema` parameter should have an [`$id` field], since that's
how they reference each other.

However, if the first `schema` has no [`$id` field], the function will assume
its ID is `id`. Subsequent `schema` parameters without IDs will be assigned
the concatenation of `id` with an integer for its position in the variadic
list. But don't depend on that!

If any `schema` has no [`$schema` field], it will be validated against the
draft defined by the [`jsonschema.default_draft`](#configuration)
configuration or, if it's not defined, the latest draft, currently 2020-12.

Raises an error if any `schema` is `NULL`. Returns `false` if any `schema`
fails to compile, is invalid, none has an [`$id` field] matching the `id`
parameter. Logs the reason for the failure at the `INFO` level.

### `jsonschema_validates(data, schema)` ###

```postgres
SELECT jsonschema_validates(data::json,  schema::json);
SELECT jsonschema_validates(data::jsonb, schema::jsonb);
SELECT jsonschema_validates(data::json,  schema::jsonb);
SELECT jsonschema_validates(data::jsonb, schema::json);
```

**Parameters**

*   `data`: JSON or JSONB data to validate
*   `schema`: A JSON Schema in a JSON or JSONB value

This function validates data in JSON or JSONB against a JSON Schema in JSON or
JSONB. Returns `NULL` if either `data` or `schema` is `NULL`.

If `schema` has no [`$id` field], the function will refer to it as
`file:///schema.json` in error messages.

Raises an error if `schema` is invalid or does not compile. Returns `false` if
`data` fails to validate, logging validation errors at the `INFO` level.

### `jsonschema_validates(data, id, schema)` ###

```postgres
SELECT jsonschema_validates(data::json,  id::text, VARIADIC schema::json);
SELECT jsonschema_validates(data::jsonb, id::text, VARIADIC schema::jsonb);
SELECT jsonschema_validates(data::json,  id::text, VARIADIC schema::jsonb);
SELECT jsonschema_validates(data::jsonb, id::text, VARIADIC schema::json);
```

**Parameters**

*   `data`: JSON or JSONB data to validate
*   `id`: The ID of the schema to validate
*   `schema`: A list JSON Schemas in JSON or JSONB values

This function validates data in JSON or JSONB against he JSON schema with the
[`$id` field] corresponding to the `id` parameter. In general, each `schema`
parameter should have an [`$id` field], since that's how they reference each
other.

However, if the first `schema` has no [`$id` field], the function will assume
its ID is `id`. Subsequent `schema` parameters without IDs will be assigned
the concatenation of `id` with an integer for its position in the variadic
list. But don't depend on that!

Raises an error if  any`schema` is invalid or does not compile. Returns
`false` if `data` fails to validate, logging validation errors at the `INFO`
level.

### `json_matches_schema(schema, instance)`

```postgres
SELECT json_matches_schema(schema::json, instance::json);
SELECT jsonb_matches_schema(schema::json, instance::jsonb);
```

These functions correspond to [`jsonschema_validates(data,
schema)`](#jsonschema_validatesdata-schema) but match the API of
the [pg_jsonschema] extension.

The one other [pg_jsonschema] function, `jsonschema_is_valid()`, goes by [the
same name](#jsonschema_is_validschema) and has a compatible signature in this
extension.

Prior Art
---------

*   [pg_jsonschema]: JSON Schema Postgres extension written with pgrx +
    the [jsonschema crate]; ca. 20-30% faster in a [simple test].
*   [pgx_json_schema]: Slightly older JSON Schema Postgres extension written
    with pgrx + the [jsonschema crate]
*   [postgres-json-schema]: JSON Schema Postgres extension written in PL/pgSQL
*   [is_jsonb_valid]: JSON Schema Postgres extension written in C

Support
-------

This library is stored in a public [GitHub repository]. Feel free to fork and
contribute! Please file bug reports via [GitHub Issues].

Authors
-------

* [David E. Wheeler](https://justatheory.com/)

Copyright and License
---------------------

Copyright (c) 2025 David E. Wheeler, 2024-2025 Tembo

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
  [GitHub repository]: https://github.com/theory/pg-jsonschema-boon
  [GitHub Issues]: https://github.com/theory/pg-jsonschema-boon/issues/
  [spec]: https://json-schema.org/specification
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
  [boon crate]: https://github.com/santhosh-tekuri/boon/ "boon: JSON Schema (draft 2020-12, draft 2019-09, draft-7, draft-6, draft-4) Validation in Rust"
  [2020-12 draft]: https://json-schema.org/draft/2020-12/release-notes "JSON Schema: 2020-12 Release Notes"
  [composition]: https://json-schema.org/understanding-json-schema/structuring
    "JSON Schema: Structuring a complex schema"
  [simple test]: https://github.com/theory/pg-jsonschema-boon/#benchmark
  [psql]: https://www.postgresql.org/docs/current/app-psql.html "PostgreSQL Docs: psql"
  [bundle]: https://json-schema.org/understanding-json-schema/structuring#bundling
  [`$schema` field]: https://json-schema.org/draft/2020-12/json-schema-core#name-the-schema-keyword
  [`$id` field]: https://json-schema.org/draft/2020-12/json-schema-core#name-the-id-keyword
  [pg_jsonschema]: https://github.com/supabase/pg_jsonschema
  [jsonschema crate]: https://docs.rs/jsonschema/latest/jsonschema/
  [postgres-json-schema]: https://github.com/gavinwahl/postgres-json-schema
  [is_jsonb_valid]: https://github.com/furstenheim/is_jsonb_valid
  [pgx_json_schema]: https://github.com/jefbarn/pgx_json_schema
