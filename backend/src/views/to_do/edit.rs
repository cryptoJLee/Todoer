use actix_web::{web, HttpResponse};
use crate::jwt::JwToken;

use crate::{
    json_serialization::{to_do_item::TodoItem, to_do_items::TodoItems},
    processes::process_input,
    state::read_file,
    to_do::{enums::TaskStatus, to_do_factory},
};

pub async fn edit(to_do_item: web::Json<TodoItem>, token: JwToken) -> HttpResponse {
    println!("here is the message in the token: {}", token.message);
    let state = read_file("./state.json");
    let status: TaskStatus;
    match &state.get(&to_do_item.title) {
        Some(result) => {
            status = TaskStatus::from_string(result.as_str().unwrap().to_string());
        }
        None => {
            return HttpResponse::NotFound().json(format!("{} not in state", &to_do_item.title))
        }
    }
    let existing_item = to_do_factory(to_do_item.title.as_str(), status.clone());
    if &status.stringify()
        == &TaskStatus::from_string(to_do_item.status.as_str().to_string()).stringify()
    {
        return HttpResponse::Ok().json(TodoItems::get_state());
    }
    process_input(existing_item, "edit".to_owned(), &state);
    return HttpResponse::Ok().json(TodoItems::get_state());
}
