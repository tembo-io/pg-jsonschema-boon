use boon::{Compiler, Schemas};
use pgrx::prelude::*;
use serde_json::Value;

pgrx::pg_module_magic!();

// pg_jsonschema-compatible functions.
#[pg_extern(immutable, strict)]
fn json_matches_schema(schema: pgrx::Json, instance: pgrx::Json) -> bool {
    let schemas = [schema.0];
    validate(id_for(&schemas), &schemas, instance.0)
}

#[pg_extern(immutable, strict)]
fn jsonb_matches_schema(schema: pgrx::Json, instance: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    validate(id_for(&schemas), &schemas, instance.0)
}

// Schema validation functions.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn jsonb_schemas_are_valid(schemas: pgrx::VariadicArray<pgrx::JsonB>) -> bool {
    let schemas = convert_jsonbs(schemas);
    compiles(id_for(&schemas), &schemas)
}

#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn json_schemas_are_valid(schemas: pgrx::VariadicArray<pgrx::Json>) -> bool {
    let schemas = convert_jsons(schemas);
    compiles(id_for(&schemas), &schemas)
}

#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn jsonb_schema_id_is_valid(id: &str, schemas: pgrx::VariadicArray<pgrx::JsonB>) -> bool {
    let schemas = convert_jsonbs(schemas);
    compiles(id, &schemas)
}

#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn json_schema_id_is_valid(id: &str, schemas: pgrx::VariadicArray<pgrx::Json>) -> bool {
    let schemas = convert_jsons(schemas);
    compiles(id, &schemas)
}

// Document validation functions.
#[pg_extern(immutable, strict, name = "json_is_valid")]
fn json_is_valid(json: pgrx::Json, id: &str, schemas: pgrx::VariadicArray<pgrx::Json>) -> bool {
    let schemas = convert_jsons(schemas);
    validate(id, &schemas, json.0)
}

#[pg_extern(immutable, strict, name = "json_is_valid")]
fn json_is_valid_first(json: pgrx::Json, schemas: pgrx::VariadicArray<pgrx::Json>) -> bool {
    let schemas = convert_jsons(schemas);
    validate(id_for(&schemas), &schemas, json.0)
}

#[pg_extern(immutable, strict, name = "json_is_valid")]
fn jsonb_is_valid(json: pgrx::Json, id: &str, schemas: pgrx::VariadicArray<pgrx::JsonB>) -> bool {
    let schemas = convert_jsonbs(schemas);
    validate(id, &schemas, json.0)
}

#[pg_extern(immutable, strict, name = "json_is_valid")]
fn jsonb_is_valid_first(json: pgrx::Json, schemas: pgrx::VariadicArray<pgrx::JsonB>) -> bool {
    let schemas = convert_jsonbs(schemas);
    validate(id_for(&schemas), &schemas, json.0)
}

/// Converts schemas from `pgrx::Array<pgrx::Json>` and returns the result.
/// Used by the functions that take multiple
fn convert_jsons(schemas: pgrx::VariadicArray<pgrx::Json>) -> Vec<Value> {
    schemas
        .iter()
        .map(|x| match x {
            Some(s) => s.0,
            None => {
                error!("NULL schema");
            }
        })
        .collect::<Vec<_>>()
}

fn convert_jsonbs(schemas: pgrx::VariadicArray<pgrx::JsonB>) -> Vec<Value> {
    schemas
        .iter()
        .map(|x| match x {
            Some(s) => s.0,
            None => {
                error!("NULL schema");
            }
        })
        .collect::<Vec<_>>()
}

fn id_for(schemas: &[Value]) -> &str {
    for s in schemas.iter() {
        if let Value::String(s) = &s["$id"] {
            return s;
        };
    }
    "schema.json"
}

fn new_compiler(id: &str, schemas: &[Value]) -> Option<Compiler> {
    let mut compiler = Compiler::new();

    if schemas.is_empty() {
        // notice!("{e}");
        println!("No schemas passed to jsonschema_compile");
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
            // notice!("{e}");
            println!("{e}");
            return None;
        }
    }

    Some(compiler)
}

fn compiles(id: &str, schemas: &[Value]) -> bool {
    if let Some(mut c) = new_compiler(id, schemas) {
        let mut schemas = Schemas::new();
        if let Err(e) = c.compile(id, &mut schemas) {
            // notice!("{e}");
            println!("{e}");
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
                println!("{e}");
                return false;
            }
            Ok(index) => {
                if let Err(e) = schemas.validate(&instance, index) {
                    println!("{e}");
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
