CREATE TABLE serial_numbers (
    serial_number TEXT NOT NULL UNIQUE,
    user_name TEXT NOT NULL PRIMARY KEY,
    serial_status INTEGER NOT NULL
)
