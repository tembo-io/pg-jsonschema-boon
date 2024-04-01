use boon::{CompileError, Compiler, Schemas};
use pgrx::prelude::*;
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
    match validate(id_for!(&schemas[0]), &schemas, instance.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

#[pg_extern(immutable, strict)]
fn jsonb_matches_schema(schema: pgrx::Json, instance: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    match validate(id_for!(&schemas[0]), &schemas, instance.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
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
    match compiles(id_for!(&schemas[0]), &schemas) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// jsonb_schema_is_valid validates `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn jsonb_schema_is_valid(schema: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    match compiles(id_for!(&schemas[0]), &schemas) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// json_schema_id_is_valid validates the schema with the `$id` `id` from the
/// `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn json_schema_id_is_valid(id: &str, schemas: pgrx::VariadicArray<pgrx::Json>) -> bool {
    let schemas = values_for!(schemas);
    match compiles(id, &schemas) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// jsonb_schema_id_is_valid validates the schema with the `$id` `id` from the
/// `schemas`.
#[pg_extern(immutable, strict, name = "jsonschema_is_valid")]
fn jsonb_schema_id_is_valid(id: &str, schemas: pgrx::VariadicArray<pgrx::JsonB>) -> bool {
    let schemas = values_for!(schemas);
    match compiles(id, &schemas) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
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
    match validate(id_for!(&schemas[0]), &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// jsonb_schema_validates_jsonb validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_validates_jsonb(json: pgrx::JsonB, schema: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    match validate(id_for!(&schemas[0]), &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// json_schema_validates_jsonb validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn json_schema_validates_jsonb(json: pgrx::Json, schema: pgrx::JsonB) -> bool {
    let schemas = [schema.0];
    match validate(id_for!(&schemas[0]), &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// jsonb_schema_validates_json validates `json` against `schema`.
#[pg_extern(immutable, strict, name = "jsonschema_validates")]
fn jsonb_schema_validates_json(json: pgrx::JsonB, schema: pgrx::Json) -> bool {
    let schemas = [schema.0];
    match validate(id_for!(&schemas[0]), &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
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
    match validate(id, &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
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
    match validate(id, &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
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
    match validate(id, &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
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
    match validate(id, &schemas, json.0) {
        Err(e) => error!("{e}"),
        Ok(ok) => ok,
    }
}

/// Supported draft versions
#[non_exhaustive]
#[derive(PostgresGucEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Draft {
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

static GUC: pgrx::GucSetting<Draft> = pgrx::GucSetting::<Draft>::new(Draft::V2020_12);

// initialize GUCs
pub fn init_guc() {
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

#[pg_guard]
pub extern "C" fn _PG_init() {
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

/// compiles compiles the schema named `id` in `schemas`, returning `true` on
/// success and `false` on failure.
fn compiles(id: &str, schemas: &[Value]) -> Result<bool, CompileError> {
    if let Ok(mut c) = new_compiler(id, schemas) {
        let mut schemas = Schemas::new();
        c.compile(id, &mut schemas)?;
        return Ok(true);
    }

    Ok(false)
}

/// validate validates `instance` against schema `id` in `schemas`.
pub fn validate(id: &str, schemas: &[Value], instance: Value) -> Result<bool, CompileError> {
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

    #[derive(Debug, Eq, PartialEq)]
    enum ErrorCaught {
        True,
        False,
    }
    fn load_json(name: &str) -> Value {
        let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let root = Path::new(&root_dir);
        serde_json::from_reader(File::open(root.join("schemas").join(name)).unwrap()).unwrap()
    }

    #[pg_test]
    fn test_compiles() -> Result<(), Box<dyn Error>> {
        let address = load_json("address.schema.json");
        let user = load_json("user-profile.schema.json");

        assert!(compiles(
            "https://example.com/address.schema.json",
            &[address.clone(), user.clone()]
        )
        .unwrap());
        assert!(compiles(
            "https://example.com/user-profile.schema.json",
            &[address.clone(), user.clone()]
        )
        .unwrap());
        assert!(compiles(
            "https://example.com/nonesuch.schema.json",
            &[address.clone(), user.clone()]
        )
        .is_err());

        Ok(())
    }

    #[pg_test]
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

        Ok(())
    }

    #[pg_test]
    fn test_id_for() {
        assert_eq!(id_for!(json!({"$id": "foo"})), "foo");
        assert_eq!(id_for!(json!({"$id": "bar"})), "bar");
        assert_eq!(id_for!(json!({"$id": "yo", "id": "hi"})), "yo");
        assert_eq!(id_for!(json!({"id": "yo"})), "file:///schema.json");
        assert_eq!(id_for!(json!(null)), "file:///schema.json");
    }

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
        let query = format!(
            "SELECT jsonb_matches_schema('{}', '{}')",
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
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
