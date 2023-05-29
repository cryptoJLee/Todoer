use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};

use crate::{
    json_serialization::{to_do_item::TodoItem, to_do_items::TodoItems},
    database::establish_connection,
    schema::to_do,
    jwt::JwToken,
    models::item::item::Item,
};

pub async fn delete(to_do_item: web::Json<TodoItem>, _token: JwToken) -> HttpResponse {
    let connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();
    let _= diesel::delete(&items[0]).execute(&connection);
    HttpResponse::Ok().json(TodoItems::get_state())
}
