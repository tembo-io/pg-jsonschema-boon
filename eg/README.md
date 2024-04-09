jsonschema Examples
===================

This directory contains example use cases for the PostgreSQL jsonschema
extension.

Schemas
-------

The following schema files are used in one or more example scripts (detailed
below) and in the unit test suite (run by `make test`).

### `address.schema.json`

A sample JSON schema for validating mail addresses. Borrowed from the [Address
Example].

### `user-profile.schema.json`

A sample JSON schema for validating user profiles. Borrowed from the [User
Profile Example], but augmented with an `address` property that references
`address.schema.json`.

Use Cases
---------

### `bench.sql`

This file contains a simple benchmark SQL script that demonstrates the
performance of jsonschema vs [pg_jsonschema]. It requires that each be built
and installed. Parameters:

*   `extension`: The extension to test, `jsonschema` or `pg_jsonschema`.
    If not set there will be no validation.
*   `iterations`: Number of iterations of the task to run (inserting rowed
    into a table). Defaults to `200_000`.

 To test jsonschema, run:

```sh
psql -Xf eg/bench.sql --set extension=jsonschema

```

And to test [pg_jsonschema]:

```sh
psql -Xf eg/bench.sql --set extension=pg_jsonschema
```

Omit the `extension` parameter to see the performance with no schema
validation:

```sh
psql -Xf eg/bench.sql
```

Set `iterations` to change the number of iterations (defaults to 200,000):

```sh
psql -Xf eg/bench.sql --set iterations=500_000
```

### `user.sql`

This SQL script demonstrates the use of composed schemas to validate records
in a table. It uses [jq] to load  [address.schema.json`](#addressschemajson)
and [user-profile.schema.json](#user-profileschemajson) into a table,
validates them both, then constructs a function that uses them to validate a
user profile. It then creates a table for users with a check constraint using
that function. It demonstrates inserting valid and invalid user objects.

The script must be run from the project root directory so that it can find and
load the schema files. Run it like so:

``` sh
psql -Xf eg/user.sql
```

  [Address Example]: https://json-schema.org/learn/json-schema-examples#address
  [User Profile Example]: https://json-schema.org/learn/json-schema-examples#user-profile
  [pg_jsonschema]: https://github.com/supabase/pg_jsonschema
  [jq]: https://jqlang.github.io/jq/manual/