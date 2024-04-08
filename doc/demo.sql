\timing off


BEGIN;

CREATE EXTENSION IF NOT EXISTS jsonschema;

SELECT '
\ir schemas/address.schema.json
' AS address_schema \gset

\echo :address_schema


ROLLBACK;