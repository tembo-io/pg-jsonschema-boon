use boon::{Compiler, Draft, Schemas};
use pgrx::prelude::*;
use serde_json::Value;

pgrx::pg_module_magic!();

#[pg_extern]
fn jsonschema_validate(jsonb: pgrx::JsonB, schemas: Vec<pgrx::JsonB>) -> bool {
    let schema_url = "http://tmp/schema.json";
    let instance: Value =
        serde_json::from_value(jsonb.0).expect("failed to parse json response from SPI");

    // Create the compiler. Default to the latest draft; but add a GUC.
    let mut compiler = Compiler::new();
    compiler.set_default_draft(Draft::V2020_12);

    // Load all of the schemas.
    for s in schemas.into_iter() {
        let schema: Value =
            serde_json::from_value(s.0).expect("failed to parse json response from SPI");
        compiler
            .add_resource(schema_url, schema)
            .expect("failed to parse json response from SPI");
    }

    let mut schemas = Schemas::new();
    let sch_index = compiler
        .compile(schema_url, &mut schemas)
        .expect("Cannot compile schemas");
    let result = schemas.validate(&instance, sch_index);
    result.is_ok()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;
    use std::error::Error;

    #[pg_test]
    fn test_hello_jsonschema() -> Result<(), Box<dyn Error>> {
        assert!(crate::jsonschema_validate(
            pgrx::JsonB(serde_json::from_str(r#"{"type": "object"}"#)?),
            vec![pgrx::JsonB(serde_json::from_str(r#"{"type": "object"}"#)?)],
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
