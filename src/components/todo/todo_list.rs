use crate::components::todo::todo_item::TodoItem;
use crate::components::todo::types::Todo;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
    pub todo_items: Vec<Todo>,         // Rc を外して Vec<Todo> に変更
    pub on_toggle: Callback<(String)>, // チェック状態更新用のコールバック
    pub on_delete: Callback<String>,   // 削除用のコールバック
}

#[function_component(TodoList)]
pub fn todo_list(props: &TodoItemProps) -> Html {
    html! {
        <ul class="list-group">
            {for props.todo_items.iter().map(|todo| {
                let on_toggle = props.on_toggle.clone();
                let on_delete = props.on_delete.clone();
                let completed = todo.completed;
                let todo_id1 = todo.id.clone(); // `todo.id` をクロージャの外でクローン
                let todo_id2 = todo.id.clone(); // `todo.id` をクロージャの外でクローン

                html! {
                    <TodoItem
                        title={todo.title.clone()}
                        completed={completed}
                        on_toggle={Callback::from(move |_| on_toggle.emit(todo_id1.clone()))}
                        on_delete={Callback::from(move |_| on_delete.emit(todo_id2.clone()))}
                    />
                }
            })}
        </ul>
    }
}
