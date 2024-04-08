\set QUIET
\pset pager off
SET client_min_messages TO WARNING;
\set ECHO queries

\unset QUIET

CREATE EXTENSION jsonschema;
\echo
\prompt xxx

SET jsonschema.default_draft TO 'V2020';
\echo
\prompt xxx

CREATE TEMPORARY TABLE json_schemas(
    schema JSON
);
\echo
\prompt xxx

\set ECHO ALL
\copy json_schemas FROM PROGRAM 'jq -c . eg/*.schema.json';
\set ECHO queries
\echo
\prompt xxx

SELECT jsonschema_is_valid(
    'https://example.com/user-profile.schema.json',
    VARIADIC ARRAY(SELECT schema from json_schemas)
);
\prompt xxx

SELECT jsonschema_is_valid(
    'https://example.com/address.schema.json',
    VARIADIC ARRAY(SELECT schema from json_schemas)
);
\prompt xxx

CREATE OR REPLACE FUNCTION validate_user(
    data json
) RETURNS BOOLEAN LANGUAGE SQL STABLE AS $$
    SELECT jsonschema_validates(
        data, 'https://example.com/user-profile.schema.json',
        VARIADIC ARRAY(SELECT schema from json_schemas)
    );
$$;
\echo
\prompt xxx

CREATE TEMPORARY TABLE json_users (
    id SERIAL PRIMARY KEY,
    body JSON CHECK (validate_user(body))
);
\echo
\prompt xxx

INSERT INTO json_users (body) VALUES(json_build_object(
    'username', 'theory',
    'email', 'theory@example.com'
));
\echo
\prompt xxx

SELECT body FROM json_users WHERE body->>'username' = 'theory';
\prompt xxx

INSERT INTO json_users (body) VALUES(json_build_object(
    'username', 'naomi',
    'email', 'nagata@rocinante.ship',
    'address', json_build_object(
        'locality', 'Series',
        'region', 'The Belt',
        'countryName', 'Sol System'
    )
));
\echo
\prompt xxx

SELECT body FROM json_users WHERE body->>'username' = 'naomi';
\prompt xxx

INSERT INTO json_users (body) VALUES(json_build_object(
    'username', 42,
    'email', 'hhgttg@example.com'
));
\echo
\prompt xxx

\set ECHO none
\set QUIET
DROP FUNCTION validate_user(json) CASCADE;
DROP EXTENSION jsonschema;
