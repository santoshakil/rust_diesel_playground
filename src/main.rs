pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::schema::users as users_table;
use crate::schema::users::dsl::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct User {
    pub id: Option<i32>,
    pub name: String,
    pub age: i32,
}

impl User {
    pub fn new(u_name: &str, u_age: i32) -> Self {
        Self {
            id: None,
            name: u_name.to_string(),
            age: u_age,
        }
    }
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = &mut SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let new_user = User::new("John", 20);

    diesel::insert_into(users_table::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error inserting new user");

    let all_users: Vec<User> = users
        .filter(age.gt(20))
        .limit(5)
        .load(connection)
        .expect("Error loading users");

    for user in all_users {
        println!(
            "id: {}, name: {}, age: {}",
            user.id.unwrap(),
            user.name,
            user.age
        );
    }
}
