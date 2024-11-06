// src/api.rs

use crate::components::todo::types::Todo;
use reqwest::Client;
use std::error::Error;

fn get_api_base_url() -> &'static str {
    env!("API_BASE_URL")
}

pub async fn fetch_todos() -> Result<Vec<Todo>, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/todos", get_api_base_url());
    let response = client.get(&url).send().await?.json::<Vec<Todo>>().await?;
    Ok(response)
}

pub async fn post_todo(title: &str) -> Result<Todo, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/todos", get_api_base_url());
    let new_todo = Todo {
        title: title.to_string(),
        completed: false,
    };
    let response = client
        .post(&url)
        .json(&new_todo)
        .send()
        .await?
        .json::<Todo>()
        .await?;
    Ok(response)
}

pub async fn delete_todo(id: String) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/todos/{}", get_api_base_url(), id);
    client.delete(&url).send().await?.error_for_status()?;
    Ok(())
}
