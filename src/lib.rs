use boon::{Compiler, Schemas};
use pgrx::prelude::*;
use serde_json::Value;

pgrx::pg_module_magic!();

// id_for returns the schema `$id` for $x, falling back on "schema.json" if $x
// has no `$id`.
macro_rules! id_for {
    ($x:expr) => {
        if let Value::String(s) = &$x["$id"] {
            s
        } else {
            "schema.json"
        }
    };
}

// Converts schemas from `pgrx::Array<_>` to `Vec<serde_json::Value>` and
// returns the result. Used by the variadic functions.
macro_rules! values_for {
    ($x:expr) => {
        $x.iter_deny_null().map(|x| x.0).collect::<Vec<_>>()
    };
}

// pg_jsonschema-compatible functions.
#[pg_extern(immutable, strict)]
fn json_matches_schema(schema: pgrx::Json, instance: pgrx::Json) -> bool {
    let schemas = [schema.0];
    validate(id_for!(&schemas[0]), &schemas, instance.0)
}

#[pg_extern(immutable, strict)]
fn jsonb_matches_schema(schema: pgrx::Json, instance: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    validate(id_for!(&schemas[0]), &schemas, instance.0)
}

// Schema validation functions.

// jsonschema_is_valid(schema::json)
// jsonschema_is_valid(schema::jsonb)
// jsonschema_is_valid(id::text, VARIADIC schema::json)
// jsonschema_is_valid(id::text, VARIADIC schema::jsonb)

/// json_schema_is_valid validates `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn json_schema_is_valid(schema: pgrx::Json) -> bool {
    let schemas = [schema.0];
    compiles(id_for!(&schemas[0]), &schemas)
}

/// jsonb_schema_is_valid validates `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn jsonb_schema_is_valid(schema: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    compiles(id_for!(&schemas[0]), &schemas)
}

/// json_schema_id_is_valid validates the schema with the `$id` `id` from the
/// `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn json_schema_id_is_valid(id: &str, schemas: pgrx::VariadicArray<pgrx::Json>) -> bool {
    let schemas = values_for!(schemas);
    compiles(id, &schemas)
}

/// jsonb_schema_id_is_valid validates the schema with the `$id` `id` from the
/// `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn jsonb_schema_id_is_valid(id: &str, schemas: pgrx::VariadicArray<pgrx::JsonB>) -> bool {
    let schemas = values_for!(schemas);
    compiles(id, &schemas)
}

// Document validation functions.

// jsonschema_validates(doc::json,  schema::json)
// jsonschema_validates(doc::jsonb, schema::jsonb)
// jsonschema_validates(doc::json,  schema::jsonb)
// jsonschema_validates(doc::jsonb, schema::json)

/// json_schema_validates_json validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_validates_json(json: pgrx::Json, schema: pgrx::Json) -> bool {
    let schemas = [schema.0];
    validate(id_for!(&schemas[0]), &schemas, json.0)
}

/// jsonb_schema_validates_jsonb validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_validates_jsonb(json: pgrx::JsonB, schema: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    validate(id_for!(&schemas[0]), &schemas, json.0)
}

/// json_schema_validates_jsonb validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_validates_jsonb(json: pgrx::Json, schema: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    validate(id_for!(&schemas[0]), &schemas, json.0)
}

/// jsonb_schema_validates_json validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_validates_json(json: pgrx::JsonB, schema: pgrx::Json) -> bool {
    let schemas = [schema.0];
    validate(id_for!(&schemas[0]), &schemas, json.0)
}

// jsonschema_validates(doc::json,  id::text, VARIADIC schema::json)
// jsonschema_validates(doc::jsonb, id::text, VARIADIC schema::jsonb)
// jsonschema_validates(doc::json,  id::text, VARIADIC schema::jsonb)
// jsonschema_validates(doc::jsonb, id::text, VARIADIC schema::json)

/// json_schema_id_validates_json validates `json` against the schema with the
/// `$id` `id` in `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_id_validates_json(
    json: pgrx::Json,
    id: &str,
    schemas: pgrx::VariadicArray<pgrx::Json>,
) -> bool {
    let schemas = values_for!(schemas);
    validate(id, &schemas, json.0)
}

/// jsonb_schema_id_validates_jsonb validates `json` against the schema with
/// the `$id` `id` in `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_id_validates_jsonb(
    json: pgrx::JsonB,
    id: &str,
    schemas: pgrx::VariadicArray<pgrx::JsonB>,
) -> bool {
    let schemas = values_for!(schemas);
    validate(id, &schemas, json.0)
}

/// json_schema_id_validates_jsonb validates `json` against the schema with
/// the `$id` `id` in `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_id_validates_jsonb(
    json: pgrx::Json,
    id: &str,
    schemas: pgrx::VariadicArray<pgrx::JsonB>,
) -> bool {
    let schemas = values_for!(schemas);
    validate(id, &schemas, json.0)
}

/// jsonb_schema_id_validates_json validates `json` against the schema with
/// the `$id` `id` in `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_id_validates_json(
    json: pgrx::JsonB,
    id: &str,
    schemas: pgrx::VariadicArray<pgrx::Json>,
) -> bool {
    let schemas = values_for!(schemas);
    validate(id, &schemas, json.0)
}

/// new_compiler creates and returns a new `boon::Compiler` loaded with
/// `schemas`. Each schema in `schemas` is named for its `$id` field or, if it
/// has none, `id` is used for the first schema, and `"{id}{i}"` for
/// subsequent schemas.
fn new_compiler(id: &str, schemas: &[Value]) -> Option<Compiler> {
    let mut compiler = Compiler::new();

    if schemas.is_empty() {
        error!("No schemas passed");
    }

    for (i, s) in schemas.iter().enumerate() {
        let sid = if let Value::String(s) = &s["$id"] {
            s.to_string()
        } else if i == 0 {
            // Use id for the first item.
            id.to_string()
        } else {
            // Use loc{i} for others.
            format!("{id}{i}")
        };

        if let Err(e) = compiler.add_resource(&sid, s.clone()) {
            error!("{e}");
        }
    }

    Some(compiler)
}

/// compiles compiles the schema named `id` in `schemas`, returning `true` on
/// success and `false` on failure.
fn compiles(id: &str, schemas: &[Value]) -> bool {
    if let Some(mut c) = new_compiler(id, schemas) {
        let mut schemas = Schemas::new();
        if let Err(e) = c.compile(id, &mut schemas) {
            error!("{e}");
        }
        return true;
    }

    false
}

/// validate validates `instance` against schema `id` in `schemas`.
pub fn validate(id: &str, schemas: &[Value], instance: Value) -> bool {
    if let Some(mut c) = new_compiler(id, schemas) {
        let mut schemas = Schemas::new();
        match c.compile(id, &mut schemas) {
            Err(e) => error!("{e}"),
            Ok(index) => {
                if let Err(e) = schemas.validate(&instance, index) {
                    info!("{e}");
                    return false;
                }
                return true;
            }
        }
    }

    false
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;
    use pgrx::{Json, JsonB};
    use serde_json::json;
    // use std::error::Error;

    #[pg_test]
    fn test_id_for() {
        assert_eq!(id_for!(json!({"$id": "foo"})), "foo");
        assert_eq!(id_for!(json!({"$id": "bar"})), "bar");
        assert_eq!(id_for!(json!({"$id": "yo", "id": "hi"})), "yo");
        assert_eq!(id_for!(json!({"id": "yo"})), "schema.json");
        assert_eq!(id_for!(json!(null)), "schema.json");
    }

    // #[pg_test]
    // fn test_values_for() {
    //     let v = vec![Json(json!("hi")), Json(json!({"x": "y"}))];
    //     let datum = v.into_datum();
    //     let varray = VariadicArray::<Json>::from_datum(datum, false);
    //     assert_eq!(values_for!(&varray), v);
    // }

    #[pg_test]
    fn test_jsonb_matches_schema() {
        assert!(crate::jsonb_matches_schema(
            Json(json!({"type": "object"})),
            JsonB(json!({"hi": "there"})),
        ));
    }

    #[pg_test]
    fn test_jsonb_matches_schema_sql() -> spi::Result<()> {
        // Valid.
        let query = format!(
            "SELECT jsonb_matches_schema('{}', '{}')",
            json!({"type": "object"}),
            json!({"x": "y"})
        );
        let result = Spi::get_one(&query)?;
        assert_eq!(result, Some(true));

        // Invalid json.
        let query = format!(
            "SELECT jsonb_matches_schema('{}', '{}')",
            json!({"type": "object"}),
            json!(["x", "y"]),
        );
        let result = Spi::get_one(&query)?;
        assert_eq!(result, Some(false));

        // Invalid schema.
        // let query = format!(
        //     "SELECT jsonb_matches_schema('{}', '{}')",
        //     json!({"type": "nonesuch"}),
        //     json!({"x": "y"})
        // );
        // let result = Spi::get_one(&query)?;
        // assert_eq!(result, Some(false));

        // NULL instance.
        let query = format!("SELECT jsonb_matches_schema(NULL, '{}')", json!({"x": "y"}));
        let result: Option<bool> = Spi::get_one(&query)?;
        assert_eq!(result, None);

        // NULL schema.
        let query = format!(
            "SELECT jsonb_matches_schema('{}', NULL)",
            json!({"type": "object"})
        );
        let result: Option<bool> = Spi::get_one(&query)?;
        assert_eq!(result, None);

        Ok(())
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
