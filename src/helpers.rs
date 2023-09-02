use crate::schema::lists::dsl::*;
use crate::schema::tasks::dsl::*;
use crate::{get_db, models::*};
use diesel::prelude::*;

pub fn get_list_items(list_name: &String) -> Vec<Task> {
    let mut conn = get_db();
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
        .expect("Error while loading tasks!")
}

pub fn display_list_items(list_name: &String, content: &Vec<Task>) {
    if content.len() == 0 {
        println!("List {} has no tasks assigned to it!", list_name);
        return;
    }

    println!("Display tasks for {}", list_name);
    for task in content {
        println!("{}: {}", task.id, task.body);
    }
}
