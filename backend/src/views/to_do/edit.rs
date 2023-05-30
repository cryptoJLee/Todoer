use crate::database::DB;
use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};
use crate::jwt::JwToken;

use crate::{
    json_serialization::{to_do_item::TodoItem, to_do_items::TodoItems},
    schema::to_do
};

pub async fn edit(to_do_item: web::Json<TodoItem>, token: JwToken, db: DB) -> HttpResponse {
    let results = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title))
        .filter(to_do::columns::user_id.eq(&token.user_id));
    let _= diesel::update(results)
        .set(to_do::columns::status.eq("DONE"))
        .execute(&db.connection);
    return HttpResponse::Ok().json(TodoItems::get_state(token.user_id));
}
