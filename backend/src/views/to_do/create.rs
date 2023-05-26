use serde_json::value::Value;
use serde_json::Map;
use actix_web::{HttpRequest, HttpResponse};

use crate::json_serialization::to_do_items::TodoItems;
use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::state::read_file;
use crate::processes::process_input;

pub async fn create(req: HttpRequest) -> HttpResponse {
  let state: Map<String, Value> = read_file("./state.json");

  let title: String = req.match_info().get("title").unwrap().to_string();
  let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);

  process_input(item, "create".to_string(), &state);
  HttpResponse::Ok().json(TodoItems::get_state())
}