use diesel::connection::SimpleConnection;

use crate::helpers::get_connection;

mod app;
mod helpers;

fn main() {
    println!("Hello, world!");
    for i in 0..20 {
        println!("{}", i);
        let mut conn = get_connection().unwrap();
        let r = conn.batch_execute(
            "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        );
        SELECT * FROM users;",
        );
        println!("{:?}", r);
    }
}
