use actix_web::{web, HttpResponse};

use crate::{json_serialization::{to_do_item::TodoItem, to_do_items::TodoItems}, jwt::JwToken, state::read_file, to_do::{enums::TaskStatus, to_do_factory}, processes::process_input};

pub async fn delete(
  to_do_item: web::Json<TodoItem>,
  token: JwToken
) -> HttpResponse {
  let state = read_file("./state.json");
  let status: TaskStatus;

  match &state.get(&to_do_item.title) {
    Some(result) => {
      status = TaskStatus::from_string(
        result.as_str().unwrap().to_string()
      );
    },
    None => {
      return HttpResponse::NotFound().json(
        format!("{} not in state", &to_do_item.title),
      );
    }
  }

  let existing_item = to_do_factory(to_do_item.title.as_str(), status.clone());
  process_input(existing_item, "delete".to_owned(), &state);
  HttpResponse::Ok().json(TodoItems::get_state())
}