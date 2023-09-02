use crate::schema::lists::dsl::*;
use crate::schema::tasks::dsl::*;
use crate::{get_db, models::*};
use diesel::prelude::*;

pub fn get_list_items(list_name: &String) -> Result<Vec<Task>, diesel::result::Error> {
    let mut conn = get_db();

    // TODO: Make this part use the `first` function for Diesel
    let list: Vec<List> = lists
        .filter(title.eq(list_name))
        .limit(1)
        .select(List::as_select())
        .load(&mut conn)
        .expect("Error while loading lists!");

    tasks
        .filter(list_id.eq(list[0].id))
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

pub fn toggle_task_completion(task_id: &Option<i32>) -> Result<usize, diesel::result::Error> {
    let mut conn = get_db();
    let id_unwrapped: i32 = task_id.unwrap();

    diesel::update(tasks.find(id_unwrapped))
        .set(completed.eq(true))
        .execute(&mut conn)
}
