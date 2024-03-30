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
        $x.iter()
            .map(|x| match x {
                Some(s) => s.0,
                None => error!("NULL schema"),
            })
            .collect::<Vec<_>>()
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

#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn json_schema_is_valid(schema: pgrx::Json) -> bool {
    let schemas = [schema.0];
    compiles(id_for!(&schemas[0]), &schemas)
}

#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn jsonb_schema_is_valid(schema: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    compiles(id_for!(&schemas[0]), &schemas)
}

#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn json_schema_id_is_valid(id: &str, schemas: pgrx::VariadicArray<pgrx::Json>) -> bool {
    let schemas = values_for!(schemas);
    compiles(id, &schemas)
}

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

#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_validates_json(json: pgrx::Json, schema: pgrx::Json) -> bool {
    let schemas = [schema.0];
    validate(id_for!(&schemas[0]), &schemas, json.0)
}

#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_validates_jsonb(json: pgrx::JsonB, schema: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    validate(id_for!(&schemas[0]), &schemas, json.0)
}

#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_validates_jsonb(json: pgrx::Json, schema: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    validate(id_for!(&schemas[0]), &schemas, json.0)
}

#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_validates_json(json: pgrx::JsonB, schema: pgrx::Json) -> bool {
    let schemas = [schema.0];
    validate(id_for!(&schemas[0]), &schemas, json.0)
}

// jsonschema_validates(doc::json,  id::text, VARIADIC schema::json)
// jsonschema_validates(doc::jsonb, id::text, VARIADIC schema::jsonb)
// jsonschema_validates(doc::json,  id::text, VARIADIC schema::jsonb)
// jsonschema_validates(doc::jsonb, id::text, VARIADIC schema::json)

#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_id_validates_json(
    json: pgrx::Json,
    id: &str,
    schemas: pgrx::VariadicArray<pgrx::Json>,
) -> bool {
    let schemas = values_for!(schemas);
    validate(id, &schemas, json.0)
}

#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_id_validates_jsonb(
    json: pgrx::JsonB,
    id: &str,
    schemas: pgrx::VariadicArray<pgrx::JsonB>,
) -> bool {
    let schemas = values_for!(schemas);
    validate(id, &schemas, json.0)
}

#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_id_validates_jsonb(
    json: pgrx::Json,
    id: &str,
    schemas: pgrx::VariadicArray<pgrx::JsonB>,
) -> bool {
    let schemas = values_for!(schemas);
    validate(id, &schemas, json.0)
}

#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_id_validates_json(
    json: pgrx::JsonB,
    id: &str,
    schemas: pgrx::VariadicArray<pgrx::Json>,
) -> bool {
    let schemas = values_for!(schemas);
    validate(id, &schemas, json.0)
}

fn new_compiler(id: &str, schemas: &[Value]) -> Option<Compiler> {
    let mut compiler = Compiler::new();

    if schemas.is_empty() {
        notice!("No schemas passed to jsonschema_compile");
        return None;
    }

    for (i, s) in schemas.iter().enumerate() {
        let sid = if let Value::String(s) = &s["$id"] {
            s.to_string()
        } else {
            // Use id for the first item and loc{i} for others.
            format!(
                "{id}{}",
                if i == 0 {
                    "".to_string()
                } else {
                    i.to_string()
                }
            )
        };

        if let Err(e) = compiler.add_resource(&sid, s.clone()) {
            notice!("{e}");
            return None;
        }
    }

    Some(compiler)
}

fn compiles(id: &str, schemas: &[Value]) -> bool {
    if let Some(mut c) = new_compiler(id, schemas) {
        let mut schemas = Schemas::new();
        if let Err(e) = c.compile(id, &mut schemas) {
            notice!("{e}");
            return false;
        }
        return true;
    }

    false
}

pub fn validate(id: &str, schemas: &[Value], instance: Value) -> bool {
    if let Some(mut c) = new_compiler(id, schemas) {
        let mut schemas = Schemas::new();
        match c.compile(id, &mut schemas) {
            Err(e) => {
                notice!("{e}");
                return false;
            }
            Ok(index) => {
                if let Err(e) = schemas.validate(&instance, index) {
                    notice!("{e}");
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
    use pgrx::prelude::*;
    use std::error::Error;

    #[pg_test]
    fn test_hello_jsonschema() -> Result<(), Box<dyn Error>> {
        assert!(crate::jsonb_matches_schema(
            pgrx::Json(serde_json::from_str(r#"{"type": "object"}"#)?),
            pgrx::JsonB(serde_json::from_str(r#"{"type": "object"}"#)?),
        ));

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
