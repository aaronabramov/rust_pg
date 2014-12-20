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
}
