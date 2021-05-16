use super::models::{NewTodo, Todo};
use super::schema::todos::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputTodo {
  pub name: String,
  pub status: String,
}

fn get_all_todos(pool: web::Data<Pool>) -> Result<Vec<Todo>, diesel::result::Error> {
  let conn = pool.get().unwrap();
  let items = todos.load::<Todo>(&conn)?;
  Ok(items)
}

pub async fn get_todos(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
  Ok(
    web::block(move || get_all_todos(db))
      .await
      .map(|todo| HttpResponse::Ok().json(todo))
      .map_err(|_| HttpResponse::InternalServerError())?,
  )
}

fn db_get_todo_by_id(pool: web::Data<Pool>, todo_id: i32) -> Result<Todo, diesel::result::Error> {
  let conn = pool.get().unwrap();
  todos.find(todo_id).get_result::<Todo>(&conn)
}

pub async fn get_todo_by_id(
  db: web::Data<Pool>,
  todo_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
  Ok(
    web::block(move || db_get_todo_by_id(db, todo_id.into_inner()))
      .await
      .map(|todo| HttpResponse::Ok().json(todo))
      .map_err(|_| HttpResponse::InternalServerError())?,
  )
}

fn add_single_todo(
  db: web::Data<Pool>,
  item: web::Json<InputTodo>,
) -> Result<Todo, diesel::result::Error> {
  let conn = db.get().unwrap();
  let new_todo = NewTodo {
    name: &item.name,
    status: &item.status,
  };
  let res = insert_into(todos).values(&new_todo).get_result(&conn)?;
  Ok(res)
}

pub async fn add_todo(
  db: web::Data<Pool>,
  item: web::Json<InputTodo>,
) -> Result<HttpResponse, Error> {
  Ok(
    web::block(move || add_single_todo(db, item))
      .await
      .map(|todo| HttpResponse::Created().json(todo))
      .map_err(|_| HttpResponse::InternalServerError())?,
  )
}

fn delete_single_todo(db: web::Data<Pool>, todo_id: i32) -> Result<usize, diesel::result::Error> {
  let conn = db.get().unwrap();
  let count = delete(todos.find(todo_id)).execute(&conn)?;
  Ok(count)
}

pub async fn delete_todo(
  db: web::Data<Pool>,
  todo_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
  Ok(
    web::block(move || delete_single_todo(db, todo_id.into_inner()))
      .await
      .map(|todo| HttpResponse::Ok().json(todo))
      .map_err(|_| HttpResponse::InternalServerError())?,
  )
}
