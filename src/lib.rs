use boon::{CompileError, Compiler, Schemas};
use pgrx::prelude::*;
use pgrx::{Json, JsonB, VariadicArray};
use serde_json::Value;

pgrx::pg_module_magic!();

// id_for returns the schema `$id` for $x, falling back on "file:///schema.json" if $x
// has no `$id`.
macro_rules! id_for {
    ($x:expr) => {
        if let Value::String(s) = &$x["$id"] {
            s
        } else {
            "file:///schema.json"
        }
    };
}

// run_compiles runs compiles for the schema verification functions.
macro_rules! run_compiles {
    ($x:expr, $y:expr) => {
        match compiles($x, $y) {
            Err(e) => {
                info!("{e}");
                false
            }
            Ok(()) => true,
        }
    };
}

// run_validate runs validate for the verification functions.
macro_rules! run_validate {
    ($x:expr, $y:expr, $z:expr) => {
        match validate($x, $y, $z) {
            Err(e) => error!("{e}"),
            Ok(ok) => ok,
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

// Schema validation functions.

// jsonschema_is_valid(schema::json)
// jsonschema_is_valid(schema::jsonb)
// jsonschema_is_valid(id::text, VARIADIC schema::json)
// jsonschema_is_valid(id::text, VARIADIC schema::jsonb)

/// json_schema_is_valid validates `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn json_schema_is_valid(schema: Json) -> bool {
    let schemas = [schema.0];
    run_compiles!(id_for!(&schemas[0]), &schemas)
}

/// jsonb_schema_is_valid validates `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn jsonb_schema_is_valid(schema: JsonB) -> bool {
    let schemas = [schema.0];
    run_compiles!(id_for!(&schemas[0]), &schemas)
}

/// json_schema_id_is_valid validates the schema with the `$id` `id` from the
/// `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn json_schema_id_is_valid(id: &str, schemas: VariadicArray<Json>) -> bool {
    let schemas = values_for!(schemas);
    run_compiles!(id, &schemas)
}

/// jsonb_schema_id_is_valid validates the schema with the `$id` `id` from the
/// `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn jsonb_schema_id_is_valid(id: &str, schemas: VariadicArray<JsonB>) -> bool {
    let schemas = values_for!(schemas);
    run_compiles!(id, &schemas)
}

// Document validation functions.

// jsonschema_validates(doc::json,  schema::json)
// jsonschema_validates(doc::jsonb, schema::jsonb)
// jsonschema_validates(doc::json,  schema::jsonb)
// jsonschema_validates(doc::jsonb, schema::json)

/// json_schema_validates_json validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_validates_json(json: Json, schema: Json) -> bool {
    let schemas = [schema.0];
    run_validate!(id_for!(&schemas[0]), &schemas, json.0)
}

/// jsonb_schema_validates_jsonb validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_validates_jsonb(json: JsonB, schema: JsonB) -> bool {
    let schemas = [schema.0];
    run_validate!(id_for!(&schemas[0]), &schemas, json.0)
}

/// json_schema_validates_jsonb validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_validates_jsonb(json: Json, schema: JsonB) -> bool {
    let schemas = [schema.0];
    run_validate!(id_for!(&schemas[0]), &schemas, json.0)
}

/// jsonb_schema_validates_json validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_validates_json(json: JsonB, schema: Json) -> bool {
    let schemas = [schema.0];
    run_validate!(id_for!(&schemas[0]), &schemas, json.0)
}

// Multi-file document validation functions.

// jsonschema_validates(doc::json,  id::text, VARIADIC schema::json)
// jsonschema_validates(doc::jsonb, id::text, VARIADIC schema::jsonb)
// jsonschema_validates(doc::json,  id::text, VARIADIC schema::jsonb)
// jsonschema_validates(doc::jsonb, id::text, VARIADIC schema::json)

/// json_schema_id_validates_json validates `json` against the schema with the
/// `$id` `id` in `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_id_validates_json(json: Json, id: &str, schemas: VariadicArray<Json>) -> bool {
    let schemas = values_for!(schemas);
    run_validate!(id, &schemas, json.0)
}

/// jsonb_schema_id_validates_jsonb validates `json` against the schema with
/// the `$id` `id` in `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_id_validates_jsonb(json: JsonB, id: &str, schemas: VariadicArray<JsonB>) -> bool {
    let schemas = values_for!(schemas);
    run_validate!(id, &schemas, json.0)
}

/// json_schema_id_validates_jsonb validates `json` against the schema with
/// the `$id` `id` in `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_id_validates_jsonb(json: Json, id: &str, schemas: VariadicArray<JsonB>) -> bool {
    let schemas = values_for!(schemas);
    run_validate!(id, &schemas, json.0)
}

/// jsonb_schema_id_validates_json validates `json` against the schema with
/// the `$id` `id` in `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_id_validates_json(json: JsonB, id: &str, schemas: VariadicArray<Json>) -> bool {
    let schemas = values_for!(schemas);
    run_validate!(id, &schemas, json.0)
}

// pg_jsonschema-compatible functions.
#[pg_extern(immutable, strict)]
fn json_matches_schema(schema: Json, instance: Json) -> bool {
    let schemas = [schema.0];
    run_validate!(id_for!(&schemas[0]), &schemas, instance.0)
}

#[pg_extern(immutable, strict)]
fn jsonb_matches_schema(schema: Json, instance: JsonB) -> bool {
    let schemas = [schema.0];
    run_validate!(id_for!(&schemas[0]), &schemas, instance.0)
}

/// Supported draft versions.
#[non_exhaustive]
#[derive(PostgresGucEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Draft {
    /// Draft for `http://json-schema.org/draft-04/schema`
    V4,
    /// Draft for `http://json-schema.org/draft-06/schema`
    V6,
    /// Draft for `http://json-schema.org/draft-07/schema`
    V7,
    /// Draft for `https://json-schema.org/draft/2019-09/schema`
    V2019_09,
    /// Draft for `https://json-schema.org/draft/2020-12/schema`
    V2020_12,
}

// Convert Draft into boon::Draft.
#[allow(clippy::from_over_into)]
impl Into<boon::Draft> for Draft {
    fn into(self) -> boon::Draft {
        match self {
            Draft::V4 => boon::Draft::V4,
            Draft::V6 => boon::Draft::V6,
            Draft::V7 => boon::Draft::V7,
            Draft::V2019_09 => boon::Draft::V2019_09,
            Draft::V2020_12 => boon::Draft::V2020_12,
        }
    }
}

// GUC fetches the jsonschema.default_draft GUC value.
static GUC: pgrx::GucSetting<Draft> = pgrx::GucSetting::<Draft>::new(Draft::V2020_12);

// initialize the jsonschema.default_draft GUC.
fn init_guc() {
    // Register the GUC jsonschema.default_draft, with values defined by the
    // Draft enum.
    pgrx::GucRegistry::define_enum_guc(
        "jsonschema.default_draft",
        "Default JSON Schema draft",
        r#"JSON Schema draft to use for schemas that don't define the draft in their "$schema" property."#,
        &GUC,
        pgrx::GucContext::Userset,
        pgrx::GucFlags::default(),
    );
}

// Executes when Postgres loads the extension shared object library, which it
// does the first time it's used (and in the session where its loaded by
// `CREATE EXTENSION`).
#[pg_guard]
extern "C" fn _PG_init() {
    init_guc();
}

/// new_compiler creates and returns a new `boon::Compiler` loaded with
/// `schemas`. Each schema in `schemas` is named for its `$id` field or, if it
/// has none, `id` is used for the first schema, and `"{id}{i}"` for
/// subsequent schemas.
fn new_compiler(id: &str, schemas: &[Value]) -> Result<Compiler, CompileError> {
    let mut compiler = Compiler::new();
    compiler.set_default_draft(GUC.get().into());

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

        compiler.add_resource(&sid, s.to_owned())?;
    }

    Ok(compiler)
}

/// compiles compiles the schema named `id` in `schemas`, returning `Ok(())`
/// on success and an error on failure.
fn compiles(id: &str, schemas: &[Value]) -> Result<(), CompileError> {
    let mut c = new_compiler(id, schemas)?;
    let mut schemas = Schemas::new();
    c.compile(id, &mut schemas)?;

    Ok(())
}

// Mock info!() during tests to just go to STDOUT. Would be nice to capture it
// somehow, but the lack of reference to a std::io::Write in pgrx's info!()
// makes it tricky.
#[cfg(test)]
macro_rules! info {
    ($($arg:tt)*) => {{ println!($($arg)*)}};
}

/// validate validates `instance` against schema `id` in `schemas`.
fn validate(id: &str, schemas: &[Value], instance: Value) -> Result<bool, CompileError> {
    match new_compiler(id, schemas) {
        Err(e) => Err(e),
        Ok(mut c) => {
            let mut schemas = Schemas::new();
            match c.compile(id, &mut schemas) {
                Err(e) => Err(e),
                Ok(index) => {
                    if let Err(e) = schemas.validate(&instance, index) {
                        info!("{e}");
                        return Ok(false);
                    }
                    Ok(true)
                }
            }
        }
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;
    use pgrx::pg_sys::panic::CaughtError::PostgresError;
    use pgrx::{spi::SpiError, Json, JsonB};
    use serde_json::json;
    use std::{env, error::Error, fs::File, path::Path};

    // Enum used to record handling expected errors.
    #[derive(Debug, Eq, PartialEq)]
    enum ErrorCaught {
        True,
        False,
    }

    // Load the named JSON file into a serde_json::Value.
    fn load_json(name: &str) -> Value {
        let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let root = Path::new(&root_dir);
        serde_json::from_reader(File::open(root.join("schemas").join(name)).unwrap()).unwrap()
    }

    // Make sure our Draft enum converts properly into boon's.
    #[test]
    fn test_draft() {
        assert_eq!(boon::Draft::V4, Draft::V4.into());
        assert_eq!(boon::Draft::V6, Draft::V6.into());
        assert_eq!(boon::Draft::V7, Draft::V7.into());
        assert_eq!(boon::Draft::V2019_09, Draft::V2019_09.into());
        assert_eq!(boon::Draft::V2020_12, Draft::V2020_12.into());
    }

    #[test]
    fn test_compiles() -> Result<(), Box<dyn Error>> {
        let address = load_json("address.schema.json");
        let user = load_json("user-profile.schema.json");

        assert!(compiles(
            "https://example.com/address.schema.json",
            &[address.clone(), user.clone()]
        )
        .is_ok());
        assert!(compiles(
            "https://example.com/user-profile.schema.json",
            &[address.clone(), user.clone()]
        )
        .is_ok());

        // An unknown schema should fail.
        assert!(compiles(
            "https://example.com/nonesuch.schema.json",
            &[address.clone(), user.clone()]
        )
        .is_err());

        // An invalid schema should fail.
        assert!(compiles("fail.json", &[json!({"$schema": "not a schema"})]).is_err());

        Ok(())
    }

    #[test]
    fn test_new_compiler() -> Result<(), Box<dyn Error>> {
        let address = load_json("address.schema.json");
        let user = load_json("user-profile.schema.json");
        let id = String::from("https://example.com/user-profile.schema.json");
        let c = new_compiler(&id, &[address.clone(), user.clone()]);
        assert!(c.is_ok());

        // Make sure it compiles user and address.
        let mut c = c.unwrap();
        let mut schemas = Schemas::new();
        c.compile(&id, &mut schemas)?;

        let mut schemas = Schemas::new();
        assert!(c
            .compile("https://example.com/address.schema.json", &mut schemas)
            .is_ok());

        // Try some without IDs
        let id = String::from("file:test.json");
        let c = new_compiler(&id, &[json!({"type": "object"}), json!({"type": "array"})]);
        assert!(c.is_ok());

        // It should have used the id.
        let mut c = c.unwrap();
        let mut schemas = Schemas::new();
        assert!(c.compile(&id, &mut schemas).is_ok());

        // And appended "1" to the second schema.
        let mut schemas = Schemas::new();
        assert!(c.compile("file:test.json1", &mut schemas).is_ok());

        // But no more.
        let mut schemas = Schemas::new();
        assert!(c.compile("file:test.json2", &mut schemas).is_err());

        // Test an invalid draft.
        assert!(new_compiler(&id, &[json!({"$schema": "lol"})]).is_err());

        Ok(())
    }

    #[test]
    fn test_id_for() {
        assert_eq!(id_for!(json!({"$id": "foo"})), "foo");
        assert_eq!(id_for!(json!({"$id": "bar"})), "bar");
        assert_eq!(id_for!(json!({"$id": "yo", "id": "hi"})), "yo");
        assert_eq!(id_for!(json!({"id": "yo"})), "file:///schema.json");
        assert_eq!(id_for!(json!(null)), "file:///schema.json");
    }

    #[test]
    fn test_validate() -> Result<(), Box<dyn Error>> {
        let address_schema = load_json("address.schema.json");
        let user_schema = load_json("user-profile.schema.json");
        let address = json!({
          "postOfficeBox": "123",
          "streetAddress": "456 Main St",
          "locality": "Big City",
          "region": "State",
          "postalCode": "12345",
          "countryName": "Country"
        });
        let user = json!({
          "username": "user123",
          "email": "user@example.com",
          "fullName": "John Doe",
          "age": 30,
          "location": "Big City",
          "interests": ["Travel", "Technology"]
        });

        assert!(validate(
            "https://example.com/address.schema.json",
            &[address_schema.clone(), user_schema.clone()],
            address.clone(),
        )
        .unwrap());

        assert!(validate(
            "https://example.com/user-profile.schema.json",
            &[address_schema.clone(), user_schema.clone()],
            user.clone(),
        )
        .unwrap());

        // Test a compile failure.
        assert!(validate(
            "file:test.json",
            &[json!({"$id": "nonesuch"})],
            user.clone(),
        )
        .is_err());

        // Test an unknown schema name.
        assert!(validate(
            "file:unknown.json",
            &[address_schema.clone(), user_schema.clone()],
            user.clone(),
        )
        .is_err());

        // Test an invalid user.
        assert!(!validate(
            "https://example.com/user-profile.schema.json",
            &[address_schema.clone(), user_schema.clone()],
            json!({
              "username": "user123",
              "location": "Big City",
            })
        )
        .unwrap());

        // Test aa user with an invalid address.
        assert!(!validate(
            "https://example.com/address.schema.json",
            &[address_schema.clone(), user_schema.clone()],
            json!({
                "username": "user123",
                "email": "user@example.com",
                "address": {
                  "locality": "Big City",
                  "countryName": "Country"
                },
            })
        )
        .unwrap());

        Ok(())
    }

    #[pg_test]
    fn test_jsonb_matches_schema() {
        assert!(crate::jsonb_matches_schema(
            Json(json!({"type": "object"})),
            JsonB(json!({"hi": "there"})),
        ));
    }

    #[pg_test]
    fn test_json_matches_schema_sql() -> spi::Result<()> {
        // pg_jsonschema-compatible function.
        for func in ["jsonb_matches_schema", "json_matches_schema"].iter() {
            // Valid.
            let query = format!(
                "SELECT {}('{}', '{}')",
                func,
                json!({"type": "object"}),
                json!({"x": "y"})
            );
            let result = Spi::get_one(&query)?;
            assert_eq!(result, Some(true));

            // Invalid json.
            let query = format!(
                "SELECT {}('{}', '{}')",
                func,
                json!({"type": "object"}),
                json!(["x", "y"]),
            );
            let result = Spi::get_one(&query)?;
            assert_eq!(result, Some(false));

            // Invalid schema.
            let query = format!(
                "SELECT {}('{}', '{}')",
                func,
                json!({"type": "nonesuch"}),
                json!({"x": "y"})
            );
            let res: Result<ErrorCaught, SpiError> = PgTryBuilder::new(|| {
                Spi::run(&query)?;
                Ok(ErrorCaught::False)
            })
            .catch_when(PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, |e| {
                if let PostgresError(e) = e {
                    assert_eq!(
                        "file:///schema.json is not valid against metaschema",
                        e.message(),
                    );
                }
                Ok(ErrorCaught::True)
            })
            .catch_others(|e| e.rethrow())
            .execute();
            assert_eq!(res, Ok(ErrorCaught::True));

            // NULL instance.
            let query = format!("SELECT {}(NULL, '{}')", func, json!({"x": "y"}));
            let result: Option<bool> = Spi::get_one(&query)?;
            assert_eq!(result, None);

            // NULL schema.
            let query = format!("SELECT {}('{}', NULL)", func, json!({"type": "object"}));
            let result: Option<bool> = Spi::get_one(&query)?;
            assert_eq!(result, None);
        }

        Ok(())
    }

    #[pg_test]
    fn test_jsonschema_validates_one() -> spi::Result<()> {
        let types = ["json", "jsonb"];
        for schema_type in types {
            for obj_type in types {
                // Valid.
                let query = format!(
                    "SELECT jsonschema_validates('{}'::{}, '{}'::{})",
                    json!({"x": "y"}),
                    obj_type,
                    json!({"type": "object"}),
                    schema_type,
                );
                let result = Spi::get_one(&query)?;
                assert_eq!(result, Some(true));

                // Invalid json.
                let query = format!(
                    "SELECT jsonschema_validates('{}'::{}, '{}'::{})",
                    json!(["x", "y"]),
                    obj_type,
                    json!({"type": "object"}),
                    schema_type,
                );
                let result = Spi::get_one(&query)?;
                assert_eq!(result, Some(false));

                // Invalid schema.
                let query = format!(
                    "SELECT jsonschema_validates('{}'::{}, '{}'::{})",
                    json!({"x": "y"}),
                    obj_type,
                    json!({"type": "nonesuch"}),
                    schema_type,
                );
                let res: Result<ErrorCaught, SpiError> = PgTryBuilder::new(|| {
                    Spi::run(&query)?;
                    Ok(ErrorCaught::False)
                })
                .catch_when(PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, |e| {
                    if let PostgresError(e) = e {
                        assert_eq!(
                            "file:///schema.json is not valid against metaschema",
                            e.message(),
                        );
                    }
                    Ok(ErrorCaught::True)
                })
                .catch_others(|e| e.rethrow())
                .execute();
                assert_eq!(res, Ok(ErrorCaught::True));

                // NULL schema.
                let query = format!(
                    "SELECT jsonschema_validates(NULL, '{}'::{})",
                    json!({"x": "y"}),
                    schema_type,
                );
                let result: Option<bool> = Spi::get_one(&query)?;
                assert_eq!(result, None);

                // NULL instance.
                let query = format!(
                    "SELECT jsonschema_validates('{}'::{}, NULL)",
                    json!({"type": "object"}),
                    obj_type
                );
                let result: Option<bool> = Spi::get_one(&query)?;
                assert_eq!(result, None);
            }
        }

        Ok(())
    }

    #[pg_test]
    fn test_jsonschema_validates_multi() -> spi::Result<()> {
        let address_schema = load_json("address.schema.json");
        let user_schema = load_json("user-profile.schema.json");
        let address_id = address_schema.get("$id").unwrap().as_str().unwrap();
        let user_id = user_schema.get("$id").unwrap().as_str().unwrap();
        let address = json!({
          "postOfficeBox": "123",
          "streetAddress": "456 Main St",
          "locality": "Big City",
          "region": "State",
          "postalCode": "12345",
          "countryName": "Country"
        });
        let user = json!({
          "username": "user123",
          "email": "user@example.com",
          "fullName": "John Doe",
          "age": 30,
          "interests": ["Travel", "Technology"],
          "address": {
            "locality": "Big City",
            "region": "State",
            "countryName": "Country"
          }
        });

        let types = ["json", "jsonb"];
        for schema_type in types {
            for obj_type in types {
                println!(
                    "ID: {}",
                    address_schema.get("$id").unwrap().as_str().unwrap()
                );
                // Valid address.
                let query = format!(
                    "SELECT jsonschema_validates('{obj}'::{ot}, '{id}', '{as}'::{st})",
                    obj = address,
                    ot = obj_type,
                    id = address_id,
                    as = address_schema,
                    st = schema_type,
                );
                let result = Spi::get_one(&query)?;
                assert_eq!(result, Some(true));

                // Valid user.
                let query = format!(
                    "SELECT jsonschema_validates('{obj}'::{ot}, '{id}', '{us}'::{st}, '{as}'::{st})",
                    obj = user,
                    ot = obj_type,
                    id = user_id,
                    us = user_schema,
                    as = address_schema,
                    st = schema_type,
                );
                let result = Spi::get_one(&query)?;
                assert_eq!(result, Some(true));

                // Invalid json.
                let query = format!(
                    "SELECT jsonschema_validates('{obj}'::{ot}, '{id}', '{us}'::{st}, '{as}'::{st})",
                    obj = json!({"username": "hello"}), // no email
                    ot = obj_type,
                    id = user_id,
                    us = user_schema,
                    as = address_schema,
                    st = schema_type,
                );
                let result = Spi::get_one(&query)?;
                assert_eq!(result, Some(false));

                // Invalid schema.
                let query = format!(
                    "SELECT jsonschema_validates('{obj}'::{ot}, '{id}', '{us}'::{st}, '{as}'::{st})",
                    obj = json!({"username": "hello"}), // no email
                    ot = obj_type,
                    id = user_id,
                    us = user_schema,
                    as = json!({"type": "nonesuch", "$id": "file:///lol.json"}),
                    st = schema_type,
                );
                let res: Result<ErrorCaught, SpiError> = PgTryBuilder::new(|| {
                    Spi::run(&query)?;
                    Ok(ErrorCaught::False)
                })
                .catch_when(PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, |e| {
                    if let PostgresError(e) = e {
                        assert_eq!(
                            "file:///lol.json is not valid against metaschema",
                            e.message(),
                        );
                    }
                    Ok(ErrorCaught::True)
                })
                .catch_others(|e| e.rethrow())
                .execute();
                assert_eq!(res, Ok(ErrorCaught::True));

                // NULL schema.
                let query = format!(
                    "SELECT jsonschema_validates('{obj}'::{ot}, '{id}', '{us}'::{st}, NULL::{st})",
                    obj = json!({"username": "hello"}), // no email
                    ot = obj_type,
                    id = user_id,
                    us = user_schema,
                    st = schema_type,
                );
                let res: Result<ErrorCaught, SpiError> = PgTryBuilder::new(|| {
                    Spi::run(&query)?;
                    Ok(ErrorCaught::False)
                })
                .catch_when(PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, |e| {
                    if let PostgresError(e) = e {
                        assert_eq!("array contains NULL", e.message(),);
                    }
                    Ok(ErrorCaught::True)
                })
                .catch_others(|e| e.rethrow())
                .execute();
                assert_eq!(res, Ok(ErrorCaught::True));

                // NULL instance.
                let query = format!(
                    "SELECT jsonschema_validates(NULL::{ot}, '{id}', '{us}'::{st}, '{as}'::{st})",
                    ot = obj_type,
                    id = user_id,
                    us = user_schema,
                    as = json!({"type": "nonesuch", "$id": "file:///lol.json"}),
                    st = schema_type,
                );
                let result: Option<bool> = Spi::get_one(&query)?;
                assert_eq!(result, None);
            }
        }

        Ok(())
    }

    #[pg_test]
    fn test_draft_schema_guc() -> spi::Result<()> {
        let draft = Spi::get_one("SELECT current_setting('jsonschema.default_draft')")?;
        assert_eq!(Some("V2020_12"), draft);
        assert_eq!(Draft::V2020_12, GUC.get());

        Spi::run("SELECT set_config('jsonschema.default_draft', 'V6', false)")?;
        let draft = Spi::get_one("SELECT current_setting('jsonschema.default_draft')")?;
        assert_eq!(Some("V6"), draft);
        assert_eq!(Draft::V6, GUC.get());

        Spi::run("SET jsonschema.default_draft TO 'V4'")?;
        let draft = Spi::get_one("SHOW jsonschema.default_draft")?;
        assert_eq!(Some("V4"), draft);
        assert_eq!(Draft::V4, GUC.get());

        // Make sure it fails for an invalid setting.
        let res: Result<ErrorCaught, SpiError> = PgTryBuilder::new(|| {
            Spi::run("SET jsonschema.default_draft TO 'NONESUCH'")?;
            Ok(ErrorCaught::False)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE, |_| {
            Ok(ErrorCaught::True)
        })
        .catch_others(|e| e.rethrow())
        .execute();
        assert_eq!(res, Ok(ErrorCaught::True));

        // GUC should be unchanged.
        assert_eq!(Draft::V4, GUC.get());

        Ok(())
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
