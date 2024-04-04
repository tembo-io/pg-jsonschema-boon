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
try'#   "type": "object",
try'#   "required": [ "name", "email" ],
try'#   "properties": {
try'#     "name": { "type": "string" },
try'#     "age": { "type": "number", "minimum": 0 },
try'#     "email": {"type": "string", "format": "email" }
try'#   }
try'# }' AS schema \gset

try=# -- Make sure it's valid
jsonschema=# SELECT jsonschema_is_valid(:'schema'::json);
 jsonschema_is_valid
---------------------
 t

try=# -- Define an object to validate
try=# SELECT '{
try'#   "name": "Amos Burton",
try'#   "email": "amos@rocinante.ship"
try'# }' AS person \gset

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
SET jsonschema.default_draft TO 'V2019_09';
```

For a system-wide default, set it in the `postgresql.conf` file:

```ini
jsonschema.default_draft = 'V7'
```

The supported values are:

*   `V4`: Draft for `http://json-schema.org/draft-04/schema`
*   `V6`: Draft for `http://json-schema.org/draft-06/schema`
*   `V7`: Draft for `http://json-schema.org/draft-07/schema`
*   `V2019_09`: Draft for `https://json-schema.org/draft/2019-09/schema`
*   `V2020_12`: Draft for `https://json-schema.org/draft/2020-12/schema`

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

Support
-------

This library is stored in a public [GitHub repository]. Feel free to fork and
contribute! Please file bug reports via [GitHub Issues].

Authors
-------

* [David E. Wheeler](https://justatheory.com/)

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
  [GitHub repository]: https://github.com/tembo-io/pg-jsonschema
  [GitHub Issues]: https://github.com/tembo-io/pg-jsonschema/issues/
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
  [`$schema` field]: https://json-schema.org/draft/2020-12/json-schema-core#name-the-schema-keyword
  [`$id` field]: https://json-schema.org/draft/2020-12/json-schema-core#name-the-id-keyword
  [pg_jsonschema]: https://github.com/supabase/pg_jsonschema
