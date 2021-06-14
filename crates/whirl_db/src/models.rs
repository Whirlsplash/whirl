// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! Much of the documentation that you will see within this module is quoted
//! from
//! [this](http://dev.worlds.net/private/GammaDocs/WorldServer.html#RoomServer)
//! section from the Gamma Documents.

// use crate::db::schema::*;

// --------------
// | Queryables |
// --------------

#[derive(Queryable, Debug)]
pub struct SerialNumber {
  pub serial_number: String,
  pub user_name:     String,
  pub serial_status: i32,
}

#[derive(Queryable, Debug)]
pub struct UserRegistration {
  pub user_name_lower:   String,
  pub user_name:         String,
  pub serial_number:     String,
  pub password:          String,
  pub client_version:    String,
  pub account_status:    i32,
  pub registration_date: String,
  pub times_on:          i32,
  pub total_minutes:     i32,
  pub user_privileges:   i32,
}

#[derive(Queryable, Debug)]
pub struct UserProperty {
  pub user_name:             String,
  pub property_id:           i32,
  pub property_flags:        i32,
  pub property_access:       i32,
  pub property_string_value: String,
  pub property_binary_value: String,
}

// ---------------
// | Insertables |
// ---------------

// --------------
// | Updatables |
// --------------
