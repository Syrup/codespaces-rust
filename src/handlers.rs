use crate::models::{NewTodo, Todo, TodoQuery, UpdateTodo};
use actix_web::{web, Error, HttpResponse};
use sqlx::SqlitePool;

pub async fn get_todos(
    data: web::Data<SqlitePool>,
    query: web::Query<TodoQuery>,
) -> Result<HttpResponse, Error> {
    let mut sql = String::from("SELECT id, title, description, done FROM todos");
    let mut conditions = Vec::new();
    let mut params = Vec::new();

    if let Some(title) = &query.title {
        conditions.push("title LIKE ?");
        params.push(format!("%{}%", title));
    }
    if let Some(description) = &query.description {
        conditions.push("description LIKE ?");
        params.push(format!("%{}%", description));
    }
    if let Some(done) = query.done {
        conditions.push("done = ?");
        params.push(done.to_string());
    }

    if !conditions.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&conditions.join(" AND "));
    }

    let mut query = sqlx::query_as::<_, Todo>(&sql);
    for param in params {
        query = query.bind(param);
    }

    let todos = query
        .fetch_all(data.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(todos))
}

pub async fn add_todo(
    pool: web::Data<SqlitePool>,
    todo: web::Json<NewTodo>,
) -> Result<HttpResponse, Error> {
    let result = sqlx::query!(
        "INSERT INTO todos (title, description, done) VALUES (?, ?, ?)",
        todo.title,
        todo.description,
        todo.done
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Created().json(Todo {
        id: result.last_insert_rowid() as i32,
        title: todo.title.clone(),
        description: todo.description.clone(),
        done: todo.done,
    }))
}

pub async fn update_todo(
    pool: web::Data<SqlitePool>,
    path: web::Path<i32>,
    todo: web::Json<UpdateTodo>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    // Get the current todo
    let current_todo = sqlx::query!("SELECT * FROM todos WHERE id = ?", id)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let Some(current_todo) = current_todo else {
        return Ok(HttpResponse::NotFound().finish());
    };

    // Update with new values or keep existing ones
    let title = todo.title.as_ref().unwrap_or(&current_todo.title);
    let description = todo
        .description
        .as_ref()
        .unwrap_or(&current_todo.description);
    let done = todo.done.unwrap_or(current_todo.done);

    let result = sqlx::query!(
        "UPDATE todos SET title = ?, description = ?, done = ? WHERE id = ?",
        title,
        description,
        done,
        id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if result.rows_affected() == 0 {
        Ok(HttpResponse::NotFound().finish())
    } else {
        Ok(HttpResponse::Ok().json(Todo {
            id,
            title: title.to_string(),
            description: description.to_string(),
            done,
        }))
    }
}

pub async fn delete_todo(
    pool: web::Data<SqlitePool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let result = sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if result.rows_affected() == 0 {
        Ok(HttpResponse::NotFound().finish())
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

pub async fn get_todo(
    pool: web::Data<SqlitePool>,
    path: web::Path<i32>,
    query: web::Query<TodoQuery>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let mut queries = String::from("SELECT id, title, description, done FROM todos where ");

    match &query.title {
        Some(title) => {
            queries.push_str(&format!("title = {} ", title));
        }
        None => match &query.description {
            Some(description) => {
                queries.push_str(&format!("description = {} ", description));
            }
            None => match &query.done {
                Some(done) => {
                    queries.push_str(&format!("done = {} ", done));
                }
                None => {
                    queries.push_str(&format!("id = {} ", id));
                }
            },
        },
    }

    let row = sqlx::query!(
        "SELECT id, title, description, done FROM todos where id = ?",
        id
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e));

    let row = match row {
        Ok(row) => row,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    Ok(HttpResponse::Ok().json(Todo {
        id: row.id as i32,
        title: row.title,
        description: row.description,
        done: row.done,
    }))
}

pub fn config(pool: web::Data<SqlitePool>) -> impl FnOnce(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.app_data(pool)
            .service(web::resource("/todos").route(web::get().to(get_todos)))
            .service(web::resource("/todos/").route(web::get().to(get_todos)))
            .service(web::resource("/todos/add").route(web::post().to(add_todo)))
            .service(
                web::resource("/todos/{id}")
                    .route(web::put().to(update_todo))
                    .route(web::delete().to(delete_todo))
                    .route(web::get().to(get_todo)),
            )
            .service(
                web::resource("/").route(
                    web::get().to(|| async { HttpResponse::Ok().body("Hello, Actix Web!") }),
                ),
            );
    }
}
