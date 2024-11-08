use crate::api::{delete_todo, fetch_todos, post_todo};
use api::update_todo;
use components::header::Header;
use components::todo::todo_form::TodoForm;
use components::todo::todo_list::TodoList;
use components::todo::types::Todo;
use yew::prelude::*;

mod api;
mod components;
mod env;

#[function_component(App)]
fn app() -> Html {
    let todo_items = use_state(|| Vec::<Todo>::new());

    // 初回レンダリング時に API からデータを取得
    {
        let todo_items = todo_items.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_todos().await {
                        Ok(todos) => todo_items.set(todos),
                        Err(err) => log::error!("Failed to fetch todos: {}", err),
                    }
                });
                || ()
            },
            (),
        );
    }

    let on_add = {
        let todo_items = todo_items.clone();
        Callback::from(move |title: String| {
            let todo_items = todo_items.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match post_todo(&title).await {
                    Ok(new_todo) => {
                        let mut current_todo_items = (*todo_items).clone();
                        current_todo_items.push(new_todo); // 追加された Todo を直接追加
                        todo_items.set(current_todo_items);
                    }
                    Err(err) => log::error!("Failed to add todo: {}", err),
                }
            });
        })
    };

    // on_toggle コールバックで completed 状態を更新し、API に送信
    let on_toggle = {
        let todos = todo_items.clone();
        Callback::from(move |id: String| {
            let todos = todos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(todo) = todos.iter().cloned().find(|todo| todo.id == id) {
                    let new_completed = !todo.completed;
                    match update_todo(&id, &todo.title, new_completed).await {
                        Ok(updated_todo) => {
                            let mut updated_todos = (*todos).clone();
                            if let Some(todo) = updated_todos.iter_mut().find(|todo| todo.id == id)
                            {
                                todo.completed = updated_todo.completed;
                            }
                            todos.set(updated_todos);
                        }
                        Err(err) => log::error!("Failed to update todo: {}", err),
                    }
                }
            });
        })
    };

    let on_delete = {
        let todo_items = todo_items.clone();
        Callback::from(move |id: String| {
            // id を String として受け取る
            let todo_items = todo_items.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match delete_todo(id.clone()).await {
                    Ok(_) => {
                        todo_items.set(
                            todo_items
                                .iter()
                                .cloned()
                                .filter(|todo| todo.id != id)
                                .collect(),
                        );
                    }
                    Err(err) => log::error!("Failed to delete todo: {}", err),
                }
            });
        })
    };

    html! {
      <>
        <Header />
        <main class="container-fluid mt-2">
          <TodoForm {on_add} />
          <TodoList todo_items={(*todo_items).clone()} on_toggle={on_toggle} on_delete={on_delete} />
        </main>
      </>
    }
}

fn main() {
    yew::start_app::<App>();
    wasm_logger::init(wasm_logger::Config::default());
}
