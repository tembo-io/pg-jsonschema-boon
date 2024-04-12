-- Valid schema
SELECT jsonschema_is_valid('{"type": "object"}'::json);
SELECT jsonschema_is_valid('{"type": "object"}'::jsonb);

-- Invalid schema
SELECT jsonschema_is_valid('["not a schema"]'::json);
SELECT jsonschema_is_valid('["not a schema"]'::jsonb);

-- NULLs
SELECT jsonschema_is_valid(NULL::json);
SELECT jsonschema_is_valid(NULL::jsonb);

-- Load schemas
\set addr_schema `cat eg/address.schema.json`
\set user_schema `cat eg/user-profile.schema.json`
SELECT :'addr_schema'::json ->> '$id' AS addr_schema_id \gset
SELECT :'user_schema'::json ->> '$id' AS user_schema_id \gset

-- Single named schema
SELECT jsonschema_is_valid(:'addr_schema_id', :'addr_schema'::json);
SELECT jsonschema_is_valid(:'addr_schema_id', :'addr_schema'::jsonb);

-- Multiple schema
SELECT jsonschema_is_valid(:'user_schema_id', :'addr_schema'::json, :'user_schema'::json);
SELECT jsonschema_is_valid(:'user_schema_id', :'addr_schema'::jsonb, :'user_schema'::jsonb);

-- Variadic array
SELECT jsonschema_is_valid(:'user_schema_id', VARIADIC ARRAY[:'addr_schema', :'user_schema']::json[]);
SELECT jsonschema_is_valid(:'user_schema_id', VARIADIC ARRAY[:'addr_schema', :'user_schema']::jsonb[]);

-- Invalid
SELECT jsonschema_is_valid('file:///foo', :'addr_schema'::json);
SELECT jsonschema_is_valid('file:///bar', :'addr_schema'::jsonb);

-- Default ID
SELECT jsonschema_is_valid('nonesuch', '{"type": "object"}'::json);
SELECT jsonschema_is_valid('nonesuch', '{"type": "object"}'::jsonb);

-- No such ID
SELECT jsonschema_is_valid('file:///nonesuch', :'addr_schema'::json);
SELECT jsonschema_is_valid('file:///nonesuch', :'addr_schema'::jsonb);
