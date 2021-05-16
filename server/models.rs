use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Todo {
  pub id: i32,
  pub name: String,
  pub status: String,
}

#[derive(Insertable, Debug)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
  pub name: &'a str,
  pub status: &'a str,
}
