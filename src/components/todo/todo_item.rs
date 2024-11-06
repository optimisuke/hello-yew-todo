// use crate::components::todo::types::Todo;
use yew::{function_component, html, Callback, Properties}; // Properties を yew から持ち込む

#[derive(Properties, PartialEq)] // Properties, PartialEq を継承した構造体を作成
pub struct TodoItemProps {
    pub title: String,
    pub completed: bool,
    pub on_toggle: Callback<()>, // チェック状態変更用のコールバック
    pub on_delete: Callback<()>, // 削除ボタン用のコールバック関数
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {
    let on_toggle = props.on_toggle.clone();
    let on_delete = props.on_delete.clone();
    let completed = props.completed;

    html! {
      <li class="list-group-item d-flex justify-content-between align-items-center">
      <div>
        <input
            class="form-check-input me-2"
            type="checkbox"
            checked={completed}
            onclick={Callback::from(move |_| on_toggle.emit(()))} // チェック状態をトグル
        />
          {&props.title}
      </div>
      <button type="button" class="btn btn-danger btn-sm" onclick={Callback::from(move |_| on_delete.emit(()))}>{"削除"}</button>
      </li>
    }
}
