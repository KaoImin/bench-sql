use diesel::pg::PgConnection;
use diesel::Connection;

pub struct DB {
	conn: PgConnection,
}

impl DB {
	pub fn establish(url: &str) -> Self {
		DB {
			conn: PgConnection::establish(url).unwrap(),
		}
	}
}

