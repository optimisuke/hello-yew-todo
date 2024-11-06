// src/api.rs

use crate::{components::todo::types::Todo, env::API_BASE_URL};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use std::error::Error;

fn get_api_base_url() -> &'static str {
    API_BASE_URL
}

pub async fn fetch_todos() -> Result<Vec<Todo>, Box<dyn Error>> {
    let url = format!("{}/todos", get_api_base_url());

    let response = Request::get(&url).send().await?;

    // ステータスコードが成功でない場合、エラーメッセージを返す
    if !response.ok() {
        return Err(format!("Failed to fetch todos: Status {}", response.status()).into());
    }

    let todos = response
        .json::<Vec<Todo>>()
        .await
        .map_err(|err| format!("Failed to parse todos JSON: {}", err))?;

    Ok(todos)
}

#[derive(Debug, Deserialize, Serialize)]
struct NewTodo {
    pub title: String,
    pub completed: bool,
}

pub async fn post_todo(title: &str) -> Result<Todo, Box<dyn Error>> {
    let url = format!("{}/todos", get_api_base_url());
    let new_todo = NewTodo {
        title: title.to_string(),
        completed: false,
    };

    let response = Request::post(&url).json(&new_todo)?.send().await?;

    // ステータスコードが成功でない場合、エラーメッセージを返す
    if !response.ok() {
        return Err(format!("Failed to post todo: Status {}", response.status()).into());
    }

    let todo = response
        .json::<Todo>()
        .await
        .map_err(|err| format!("Failed to parse posted todo JSON: {}", err))?;

    Ok(todo)
}

pub async fn delete_todo(id: String) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/todos/{}", get_api_base_url(), id);

    let response = Request::delete(&url).send().await?;

    // ステータスコードが成功でない場合、エラーメッセージを返す
    if !response.ok() {
        return Err(format!("Failed to delete todo: Status {}", response.status()).into());
    }

    Ok(())
}
