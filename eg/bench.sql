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
-- And without validation (control):
--
--     psql -f eg/bench.sql --iterations 200_000
--
-- Borrowed from
-- https://github.com/supabase/pg_jsonschema?tab=readme-ov-file#benchmark

\set QUIET on
\unset ECHO
\pset pager off

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

\if :{?extension}
CREATE EXTENSION :extension;
\endif

CREATE TABLE bench_json(
    meta JSON
\if :{?extension}
        CHECK (json_matches_schema(:'schema'::json, meta))
\endif
);

CREATE TABLE bench_jsonb(
    meta JSON
\if :{?extension}
        CHECK (json_matches_schema(:'schema'::json, meta))
\endif
);

\timing on

\if :{?extension}
\else
    \set extension without
\endif

\echo
\echo ######################################################################
\echo # Test :extension JSON validation for :iterations iterations
\echo ######################################################################

INSERT INTO bench_json(meta)
SELECT json_build_object( 'a', i, 'b', i::text )
  FROM generate_series(1, :iterations) t(i);

\echo
\echo ######################################################################
\echo # Test :extension JSONB validation for :iterations iterations
\echo ######################################################################

INSERT INTO bench_jsonb(meta)
SELECT json_build_object( 'a', i, 'b', i::text )
  FROM generate_series(1, :iterations) t(i);

\timing off
ROLLBACK;
