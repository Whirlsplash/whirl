// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! Much of the documentation that you will see within this module is quoted
//! from http://dev.worlds.net/private/GammaDocs/WorldServer.html#RoomServer.

/// The SerialNumbers table contains a record for every valid serial number. It
/// is initialized with a set of serial numbers by a WorldServer administrator.
/// A user will register by providing a valid serial number that matches an
/// unused table entry. The serial number must be distributed with the client
/// software or in some other way, because it will be required for all
/// registrations from the client. The serial number record contains the
/// following information:
///
///
/// The SerialNumbers table will be initialized with a set of serial numbers by
/// a WorldServer administrator. The serialStatus column should be initialized
/// to SERIAL_FREE at this time (this will be done for you when you create
/// serial numbers). A user will then register via the client by providing a
/// valid serial number. The UserServer will set serialStatus to SERIAL_USED
/// upon successful user registration with a given serial number, and enter
/// their username in the userName field.
///
/// The included program SerialGen can generate a list of serial numbers based
/// on a seed and tagged by prefix. The program runs in the C-shell and produces
/// three output files: an SQL script that you can use to directly modify the
/// SerialNumbers table, a master list as a text table for administration
/// purposes, and a separate text table for use in production of the serial
/// numbers to be given to client end users. See
/// [Generating Serial Numbers](http://dev.worlds.net/private/GammaDocs/WorldServer.html#SerialGen).
///
///
/// The values defined for serialStatus are:
///
/// 0 = SERIAL_FREE
///
/// 1 = SERIAL_USED
#[derive(Debug)]
pub struct SerialNumbers {
  /// The user's serial number
  pub serial_number: String,

  /// Username with case intact.
  pub user_name: String,

  /// One of {SERIAL_FREE, SERIAL_USED}
  pub serial_status: i64,
}

/// The UserRegistration table contains a record for every registered user. The
/// record holds the following information:
///
/// The WorldServer will set an authenticated user's privilege level to the
/// value given in this field when the user logs on. This privilege level is
/// communicated to all RoomServers the user connects to. To take effect, the
/// value must be changed while the user is logged off, otherwise it will only
/// become effective at the next login.
///
///
/// AccountStatus allowed values:
///
/// The values defined for accountStatus include:
///
/// 0 = ACCOUNT_INACTIVE
///
/// 1 = ACCOUNT_ACTIVE
///
/// The WorldServer will set accountStatus to ACCOUNT_ACTIVE upon successful
/// user registration. The WorldServer administrator may deactivate an account
/// by setting accountStatus to ACCOUNT_INACTIVE.
///
///
/// userPrivileges allowed values:
///
/// The values defined for userPrivileges include:
///
/// 0 = No privileges
///
/// 1 = PRIV_BUILD - the user may dynamically register rooms
///
/// 2 = PRIV_BROADCAST - the user may broadcast text
///
/// 4 = PRIV_PROPERTY - the user may retrieve and set all properties of any
/// object
///
/// 3 = PRIV_BUILD and PRIV_BROADCAST
///
/// 5 = PRIV_BUILD and PRIV_PROPERTY
///
/// 6 = PRIV_BROADCAST and PRIV_PROPERTY
///
/// 7 = PRIV_BUILD and PRIV_BROADCAST and PRIV_PROPERTY
///
/// The WorldServer will set an authenticated user's privilege level to the
/// value given in this field when the user logs on. This privilege level is
/// communicated to all RoomServers the user connects to. To take effect, the
/// value must be changed while the user is logged off, otherwise it will only
/// become effective at the next login.
#[derive(Debug)]
pub struct UserRegistration {
  /// The user name in all lower case.
  pub user_name_lower: String,

  /// The user name with case intact.
  pub user_name: String,

  /// The user's serial number.
  pub serial_number: String,

  /// The user's password.
  pub password: String,

  /// The user's client software version.
  pub client_version: String,

  /// One of {ACCOUNT_ACTIVE, ACCOUNT_INACTIVE}.
  pub account_status: i64,

  /// The date and time the user registered.
  pub registration_date: String,

  /// The number of times the user has logged on since registration.
  pub times_on: i64,

  /// The number of minutes the user has been logged on since registration.
  pub total_minutes: i64,

  pub user_privileges: i64,
}

/// The UserProperties table is used to store persistent user properties. These
/// are accessed every time a user logs in, and they may also be used to form
/// the reply for a "finger" operation. The UserProperties table contains the
/// following columns:
///
///
/// The setting of the PropertyFlag determines which column the value of the
/// property is stored in. When the value of the property is a string and is
/// stored in propertyStringValue, the propertyBinaryValue will be NULL. When
/// the value of the property is binary data and is stored
/// in propertyBinaryValue, the propertyStringValue will be NULL. Properties
/// stored in propertyStringValue will be readable using the Select command in
/// SQLplus. Properties stored in propertyBinaryValue will appear encoded in
/// hexadecimal when selected using SQLplus.
///
/// The values in the propertyFlags and propertyAccess as seen when doing a
/// select on these columns are as follows:
///
/// propertyFlags
///
/// 128 = Store in DB, no auto-update, not a finger property, stored in
/// propertyStringValue.
///
/// 144 = Store in DB, no auto-update, not a finger property, stored in
/// propertyBinaryValue.
///
/// 160 = Store in DB, no auto-update, finger property, stored in
/// propertyStringValue.
///
/// 176 = Store in DB, no auto-update, finger property, stored in
/// propertyBinaryValue.
///
/// 192 = Store in DB, auto-update, not a finger property, stored in
/// propertyStringValue.
///
/// 208 = Store in DB, auto-update, not a finger property, stored in
/// propertyBinaryValue.
///
/// 224 = Store in DB, auto-update, finger property, stored in
/// propertyStringValue.
///
/// 240 = Store in DB, auto-update, finger property, stored in
/// propertyBinaryValue.
///
///
/// propertyAccess
///
/// 0 = Public write, public read.
///
/// 1 = Possessor write, public read.
///
/// 2 = Public write, owner read.
///
/// 3 = Possessor write, owner read.
///
///
/// UserProperties can be used to store persistent user data from session to
/// session, including potentially shared-state properties related to users.
/// Properties and their Id's need to be coordinated between the server and the
/// client (or the client's underlying language). Properties are generally
/// meaningless to the server, except for the reserved properties for
/// session tracking etc. It is up to the client to interpret the property
/// values correctly.
///
/// In Gamma, properties are exchanged with the client by using attributes. A
/// full discussion of properties, attributes and using shared objects in Gamma
/// will be included in a future document.
///
/// PropertyId:
///
/// Each property has a PropertyId field that is one byte. Up to 255 PropertyIds
/// may be defined. Zero is reserved and is not a valid PropertyId. Some of
/// these properties are shared between o bjects (especially those relating to
/// session control), but others may be defined on a per-object basis. Currently
/// defined propertyIds include:
///
/// Session Properties:
///
/// 1 = Application Name (string)
///
/// 2 = User Name (string)
///
/// 3 = Protocol Number (string)
///
/// 4 = Error Number (string)
///
/// 6 = Password (string)
///
/// 8 = Update Interval (string)
///
/// 9 = Client Version (string)
///
/// 10 = Serial Number (string)
///
/// 12 = Logon/Logoff Flag (string)
///
/// 13 = Permitted Session Duration (string)
///
/// 14 = Guest User Flag (string)
///
/// 15 = Server Type (string)
///
/// User Properties:
///
/// 5 = Avatar Bitmap (string)
///
/// 7 = Avatar Updates Requested (string)
///
/// 9 = Client Version (string)
///
/// 11 = Email Address (string)
///
/// The client software can require users to register with their email addresses
/// as an optional field. For example, in Worlds Chat, the client sends the
/// email address as VAR_EMAIL. VAR_EMAIL in turn is understood by the server to
/// be a property that is stored in the Properties database, as Property ID 11,
/// with the email itself stored in PropertyStringValue as a string. Since this
/// table is also keyed to the username, you can correlate tables of email
/// addresses as needed using SQL.
///
/// To extract an email address for a given user:
/// ```sql
/// select propertyStringValue
/// from UserProperties
/// where userName='John Doe' and propertyId=11;
/// ```
///
/// To extract a list of all recorded email addresses:
/// ```sql
/// select userName, propertyStringValue
/// from UserProperties
/// where propertyId=11
/// order by userName;
/// ```
///
/// You should note however that many database table queries, particularly in
/// the RoomProperties table, might give you data that is stored mostly in
/// binary form, and that cannot be properly interpreted by standard SQL. What
/// you'll get in this case is often meaningless hexadecimal, which could be
/// unicode text or just raw data.
#[derive(Debug)]
pub struct UserProperties {
  /// The user name with case intact
  pub user_name: String,

  /// The property identifier.
  pub property_id: i64,

  /// Each property has a PropertyFlags field that defines certain aspects of
  /// the property.
  pub property_flags: i64,

  /// Defines access restrictions on the property.
  pub property_access: i64,

  /// The value of the property when it is a string.
  pub property_string_value: String,

  /// The value of the property when it is binary data.
  pub property_binary_value: String,
}
