use rusqlite::{params, Connection, Result};
use crate::db::tables::SerialNumbers;

#[repr(i32)] #[derive(Debug, PartialEq)]
enum AccountStatus {
	AccountInactive = 0,
	AccountActive = 1,
}

fn modify_account_status(username: &str, status: AccountStatus) -> Result<()> {
	let connection = Connection::open("worlds.db")?;

	// language=SQLite
	connection.execute(
		"UPDATE user_registration \
		SET account_status = (?1) \
		where username = '(?2)'",
		params![status as i32, username]
	)?;

	Ok(())
}

fn delete_account(username: &str) -> Result<()> {
	let connection = Connection::open("worlds.db")?;

	// Get serial_number from `username`'s row.
	// language=SQLite
	connection.query_row(
		"SELECT * FROM serial_numbers WHERE username = '(?1)'",
		params![username],
		|row| row.get(0)
	);

	let mut row = connection.prepare(
		"SELECT * \
		FROM serial_numbers \
		WHERE username = '(?1)';"
	)?;
	let row_iter = row.query_map(params![username], |row| {
		Ok(SerialNumbers {
			serial_number: row.get(0)?,
			user_name: row.get(1)?,
			serial_status: row.get(2)?,
		})
	})?;

	// Reset serial number so it can be reused.
	// language=SQLite
	connection.execute(
		"UPDATE serial_numbers \
		SET username = 'none', serial_status = 0 \
		WHERE serial_number = '(?1)'",
		params![row_iter.]
	)?;

	Ok(())
}

fn set_account_host() { }

fn modify_account_vip() { }
