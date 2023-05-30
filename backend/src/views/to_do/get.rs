use actix_web::Responder;

use crate::json_serialization::to_do_items::TodoItems;
use crate::jwt::JwToken;

pub async fn get(token: JwToken) -> impl Responder {
  TodoItems::get_state(token.user_id)
}