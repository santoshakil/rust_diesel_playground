pub mod schema;

use diesel::prelude::*;

use crate::schema::users;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
struct NewUser<'a> {
    name: &'a str,
    age: i32,
}

fn main() {
    let mut connection =
        SqliteConnection::establish("mydb.sqlite3").expect("Error connecting to database");

    diesel::insert_into(users::table)
        .values(&vec![
            NewUser {
                name: "Alice",
                age: 42,
            },
            NewUser {
                name: "Bob",
                age: 69,
            },
        ])
        .execute(&mut connection)
        .expect("Error inserting users");

    let users = users::table
        .load::<NewUser>(&mut connection)
        .expect("Error loading users");
}
