-- Make sure the DSO is loaded.
SELECT jsonschema_is_valid('{"type": "object"}'::json);

-- Should default to 2020
SELECT current_setting('jsonschema.default_draft');
SHOW jsonschema.default_draft;

-- Set to 2019
SELECT set_config('jsonschema.default_draft', 'V2019', false);
SELECT current_setting('jsonschema.default_draft');
SHOW jsonschema.default_draft;

-- Set to v7
SELECT set_config('jsonschema.default_draft', 'V7', false);
SELECT current_setting('jsonschema.default_draft');
SHOW jsonschema.default_draft;

-- Set to v6
SELECT set_config('jsonschema.default_draft', 'V6', false);
SELECT current_setting('jsonschema.default_draft');
SHOW jsonschema.default_draft;

-- Set to v4
SELECT set_config('jsonschema.default_draft', 'V4', false);
SELECT current_setting('jsonschema.default_draft');
SHOW jsonschema.default_draft;

-- Set to invalid value
SELECT set_config('jsonschema.default_draft', 'Nope', false);
SELECT current_setting('jsonschema.default_draft');
SHOW jsonschema.default_draft;
