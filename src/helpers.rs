use crate::schema::lists::dsl::*;
use crate::schema::tasks::dsl::*;
use crate::{get_db, models::*};
use diesel::prelude::*;

pub fn create_list_record(list_name: &String) -> Result<usize, diesel::result::Error> {
    let mut conn = get_db();

    let new_task = NewList {
        title: list_name.to_string(),
    };

    diesel::insert_into(lists)
        .values(new_task)
        .execute(&mut conn)
}

pub fn get_list_at(n: i32) -> Result<List, diesel::result::Error> {
    let mut conn = get_db();

    lists.select(List::as_select()).find(n).first(&mut conn)
}

pub fn get_list_count() -> Result<i64, diesel::result::Error> {
    let mut conn = get_db();
    lists.count().get_result(&mut conn)
}

pub fn ensure_at_least_one_list_exists() {
    let list_count = get_list_count().unwrap();

    if list_count == 0 {
        panic!("Cannot use this command without any lists! Please use the `create-list` command first!");
    }
}

pub fn ensure_list_exists(list_name: &String) {
    let mut conn = get_db();

    match lists
        .select(List::as_select())
        .filter(title.eq(list_name))
        .first(&mut conn)
    {
        Ok(_) => {}

        Err(err) => {
            panic!("List does not exist! More details: {}", err);
        }
    }
}

pub fn get_list_items(list_name: &String) -> Result<Vec<Task>, diesel::result::Error> {
    let mut conn = get_db();

    ensure_list_exists(list_name);
    let list: List = lists
        .filter(title.eq(list_name))
        .select(List::as_select())
        .first(&mut conn)
        .expect("Error while loading lists!");

    tasks
        .filter(list_id.eq(list.id))
        .select(Task::as_select())
        .load(&mut conn)
}

pub fn display_list_items(list_name: &String) {
    let content = get_list_items(list_name).unwrap();

    if content.len() == 0 {
        println!("List {} has no tasks assigned to it!", list_name);
        return;
    }

    println!("Display tasks for {}", list_name);
    for task in content {
        println!("{}: {}", task.id, task.body);
    }
}

pub fn create_task_record(
    task_body: &String,
    parent_id: &Option<i32>,
) -> Result<usize, diesel::result::Error> {
    let mut conn = get_db();

    let id_unwrapped = parent_id.unwrap();
    let new_task = NewTask {
        body: task_body.to_string(),
        completed: false,
        list_id: id_unwrapped,
    };

    diesel::insert_into(tasks)
        .values(new_task)
        .execute(&mut conn)
}

pub fn toggle_task_completion(task_id: &Option<i32>) -> Result<usize, diesel::result::Error> {
    let mut conn = get_db();
    let id_unwrapped: i32 = task_id.unwrap();

    diesel::update(tasks.find(id_unwrapped))
        .set(completed.eq(true))
        .execute(&mut conn)
}
