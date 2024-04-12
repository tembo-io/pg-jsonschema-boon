--  pg_jsonschema-compatible functions

-- Valid JSON
SELECT json_matches_schema('{"type": "object"}', '{"hi": "there"}');
SELECT jsonb_matches_schema('{"type": "object"}', '{"hi": "there"}');

-- Invalid JSON
SELECT json_matches_schema('{"type": "object"}', '["not an object"]');
SELECT jsonb_matches_schema('{"type": "object"}', '["not an object"]');

-- Invalid schema
SELECT json_matches_schema('{"type": "nonesuch"}', '{"x": "y"}');
SELECT jsonb_matches_schema('{"type": "nonesuch"}', '{"x": "y"}');

-- NULL instance
SELECT json_matches_schema('{"type": "object"}', NULL);
SELECT jsonb_matches_schema('{"type": "object"}', NULL);

-- NULL schema
SELECT json_matches_schema(NULL, '{"x": "y"}');
SELECT jsonb_matches_schema(NULL, '{"x": "y"}');
