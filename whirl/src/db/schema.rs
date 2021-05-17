table! {
    serial_numbers (user_name) {
        serial_number -> Text,
        user_name -> Text,
        serial_status -> Integer,
    }
}

table! {
    user_properties (user_name) {
        user_name -> Text,
        property_id -> Integer,
        property_flags -> Integer,
        property_access -> Integer,
        property_string_value -> Integer,
        property_binary_value -> Nullable<Text>,
    }
}

table! {
    user_registration (user_name) {
        user_name_lower -> Text,
        user_name -> Text,
        serial_number -> Text,
        password -> Text,
        client_version -> Text,
        account_status -> Integer,
        registration_date -> Text,
        times_on -> Integer,
        total_minutes -> Integer,
        user_privileges -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(serial_numbers, user_properties, user_registration,);
