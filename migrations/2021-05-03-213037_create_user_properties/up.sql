CREATE TABLE user_properties (
  user_name TEXT NOT NULL PRIMARY KEY,
  property_id INTEGER NOT NULL,
  property_flags INTEGER NOT NULL,
  property_access INTEGER NOT NULL,
  property_string_value INTEGER NOT NULL,
  property_binary_value TEXT
)
