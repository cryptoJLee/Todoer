use actix_web::Responder;

use crate::json_serialization::to_do_items::TodoItems;

pub async fn get() -> impl Responder {
  TodoItems::get_state()
}