-- Valid
SELECT jsonschema_validates('{"x": "y"}'::json, '{"type": "object"}'::jsonb);
SELECT jsonschema_validates('{"x": "y"}'::jsonb, '{"type": "object"}'::json);

-- Invalid json
SELECT jsonschema_validates('["x", "y"]'::json, '{"type": "object"}'::jsonb);
SELECT jsonschema_validates('["x", "y"]'::jsonb, '{"type": "object"}'::json);

-- Invalid schema
SELECT jsonschema_validates('{"x": "y"}'::json, '{"type": "nonesuch"}'::jsonb);
SELECT jsonschema_validates('{"x": "y"}'::jsonb, '{"type": "nonesuch"}'::json);

-- NULL schema
SELECT jsonschema_validates('{"x": "y"}'::json, NULL::jsonb);
SELECT jsonschema_validates('{"x": "y"}'::jsonb, NULL::json);

-- NULL JSON
SELECT jsonschema_validates(NULL::json, '{"type": "object"}'::jsonb);
SELECT jsonschema_validates(NULL::jsonb, '{"type": "object"}'::json);

-- Load schemas
\set addr_schema `cat eg/address.schema.json`
\set user_schema `cat eg/user-profile.schema.json`
SELECT :'addr_schema'::json ->> '$id' AS addr_schema_id \gset
SELECT :'user_schema'::json ->> '$id' AS user_schema_id \gset

-- Load objects
SELECT '{
  "postOfficeBox": "123",
  "streetAddress": "456 Main St",
  "locality": "Big City",
  "region": "State",
  "postalCode": "12345",
  "countryName": "Country"
}' AS address \gset

SELECT '{
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
}' AS user \gset

-- Valid address
SELECT jsonschema_validates(:'address'::json, :'addr_schema_id', :'addr_schema'::json);
SELECT jsonschema_validates(:'address'::json, :'addr_schema_id', :'addr_schema'::jsonb);
SELECT jsonschema_validates(:'address'::jsonb, :'addr_schema_id', :'addr_schema'::jsonb);
SELECT jsonschema_validates(:'address'::jsonb, :'addr_schema_id', :'addr_schema'::json);

-- Valid user
SELECT jsonschema_validates(:'user'::json, :'user_schema_id', :'addr_schema'::json, :'user_schema'::json );
SELECT jsonschema_validates(:'user'::json, :'user_schema_id', :'addr_schema'::jsonb, :'user_schema'::jsonb );
SELECT jsonschema_validates(:'user'::jsonb, :'user_schema_id', :'addr_schema'::jsonb, :'user_schema'::jsonb );
SELECT jsonschema_validates(:'user'::jsonb, :'user_schema_id', :'addr_schema'::json, :'user_schema'::json );

-- Variadic
SELECT jsonschema_validates(:'user'::json, :'user_schema_id', VARIADIC ARRAY[:'addr_schema', :'user_schema']::json[] );
SELECT jsonschema_validates(:'user'::json, :'user_schema_id', VARIADIC ARRAY[:'addr_schema', :'user_schema']::jsonb[] );
SELECT jsonschema_validates(:'user'::jsonb, :'user_schema_id', VARIADIC ARRAY[:'addr_schema', :'user_schema']::jsonb[] );
SELECT jsonschema_validates(:'user'::jsonb, :'user_schema_id', VARIADIC ARRAY[:'addr_schema', :'user_schema']::json[] );

-- Invalid User
\set invalid_user '{"username": "hello"}'
SELECT jsonschema_validates(:'invalid_user'::json, :'user_schema_id', :'addr_schema'::json, :'user_schema'::json );
SELECT jsonschema_validates(:'invalid_user'::json, :'user_schema_id', :'addr_schema'::jsonb, :'user_schema'::jsonb );
SELECT jsonschema_validates(:'invalid_user'::jsonb, :'user_schema_id', :'addr_schema'::jsonb, :'user_schema'::jsonb );
SELECT jsonschema_validates(:'invalid_user'::jsonb, :'user_schema_id', :'addr_schema'::json, :'user_schema'::json );

-- Invalid schema
\set invalid_schema '{"type": "nonesuch", "$id": "file:///lol.json"}'
SELECT jsonschema_validates(:'user'::json, :'user_schema_id', :'invalid_schema'::json, :'user_schema'::json );
SELECT jsonschema_validates(:'user'::json, :'user_schema_id', :'invalid_schema'::jsonb, :'user_schema'::jsonb );
SELECT jsonschema_validates(:'user'::jsonb, :'user_schema_id', :'invalid_schema'::jsonb, :'user_schema'::jsonb );
SELECT jsonschema_validates(:'user'::jsonb, :'user_schema_id', :'invalid_schema'::json, :'user_schema'::json );

-- NULL schema
SELECT jsonschema_validates(:'address'::json, :'addr_schema_id', NULL::json);
SELECT jsonschema_validates(:'address'::json, :'addr_schema_id', NULL::jsonb);
SELECT jsonschema_validates(:'address'::jsonb, :'addr_schema_id', NULL::jsonb);
SELECT jsonschema_validates(:'address'::jsonb, :'addr_schema_id', NULL::json);

-- NULL JSON
SELECT jsonschema_validates(NULL::json, :'addr_schema_id', :'addr_schema'::json);
SELECT jsonschema_validates(NULL::json, :'addr_schema_id', :'addr_schema'::jsonb);
SELECT jsonschema_validates(NULL::jsonb, :'addr_schema_id', :'addr_schema'::jsonb);
SELECT jsonschema_validates(NULL::jsonb, :'addr_schema_id', :'addr_schema'::json);
