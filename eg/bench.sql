-- This script runs a simple benchmark of a CHECK constraint using either
-- the jsonschema or the pg_jsonschema extension. To test jsonschema for
-- 200_000 iterations:
--
--     psql -f eg/bench.sql --set extension=jsonschema --iterations 200_000
--
-- And with pg_jsonschema:
--
--     psql -f eg/bench.sql --set extension=pg_jsonschema --iterations 200_000
--
-- Borrowed from
-- https://github.com/supabase/pg_jsonschema?tab=readme-ov-file#benchmark

\set QUIET on
\unset ECHO
\pset pager off

\if :{?extension}
\else
   \set extension jsonschema
\endif

\if :{?iterations}
\else
   \set iterations 200_000
\endif

BEGIN;

SELECT '{
    "type": "object",
    "properties": {
        "a": {"type": "number"},
        "b": {"type": "string"}
    }
}' AS schema \gset

CREATE EXTENSION :extension;

CREATE TABLE bench_json(
    meta JSON CHECK (json_matches_schema(:'schema'::json, meta))
);

CREATE TABLE bench_jsonb(
    meta JSON CHECK (json_matches_schema(:'schema'::json, meta))
);

\timing on

\echo
\echo ######################################################################
\echo # Test :extension JSON validation for :iterations iterations
\echo ######################################################################

INSERT INTO bench_json(meta)
SELECT json_build_object( 'a', i, 'b', i::text )
  FROM generate_series(1, :iterations) t(i);

\echo
\echo
\echo ######################################################################
\echo # Test :extension JSONB validation for :iterations iterations
\echo ######################################################################

INSERT INTO bench_jsonb(meta)
SELECT json_build_object( 'a', i, 'b', i::text )
  FROM generate_series(1, :iterations) t(i);

\echo
\echo
\timing off
ROLLBACK;
