use crate::components::todo::todo_item::TodoItem;
use crate::components::todo::types::Todo;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
    pub todo_items: Vec<Todo>,      // Rc を外して Vec<Todo> に変更
    pub on_delete: Callback<usize>, // 削除用のコールバック
}

#[function_component(TodoList)]
pub fn todo_list(props: &TodoItemProps) -> Html {
    let on_delete = props.on_delete.clone();

    html! {
        <ul class="list-group">
            {for props.todo_items.iter().map(|todo| {
                let on_delete = on_delete.clone();
                let todo_id = todo.id;
                html! {
                    <TodoItem
                        title={todo.title.clone()}
                        completed={todo.completed}
                        on_delete={Callback::from(move |_| on_delete.emit(todo_id))}
                    />
                }
            })}
        </ul>
    }
}
