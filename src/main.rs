extern crate postgres;

use postgres::{Connection, SslMode};

fn main() {
    let dsn = "postgresql://dabramov@localhost/esl_development";
    let conn = match Connection::connect(dsn, &SslMode::None) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Connection error: {}", e);
            return;
        }
    };

    let stmt = match conn.prepare("select * from users") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Preparing query failed: {}", e);
            return;
        }
    };

    let mut rows = stmt.query(&[]).ok().expect("failed");
    for row in rows {
        let id: i32 = row.get("id");
        let display_name: String = row.get("display_name");
        println!("id={}, display_name={}", id, display_name);
    }
}
