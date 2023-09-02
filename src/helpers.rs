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

pub fn get_list_from_title(list_title: String) -> Result<List, diesel::result::Error> {
    let mut conn = get_db();

    lists
        .select(List::as_select())
        .filter(title.eq(list_title))
        .first(&mut conn)
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
    // Maybe these queries can be refactored to be better?
    // I'm assuming a query like `SELECT * FROM tasks WHERE list_id = (SELECT * FROM lists WHERE list_name = $) should work, very burnt out rn though so this might just be shit`
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

    println!("--- Displaying tasks for {} ---\n", list_name);

    for task in content {
        if task.completed {
            println!("[X] - {}: {}", task.id, task.body);
        } else {
            println!("[ ] - {}: {}", task.id, task.body);
        }
    }
}

pub fn display_all_items() {
    let mut conn = get_db();
    let lists_vec: Vec<List> = lists.select(List::as_select()).load(&mut conn).unwrap();

    for list in lists_vec {
        display_list_items(&list.title);
        println!(" ");
    }
}

pub fn create_task_record(
    task_body: &String,
    parent_id: i32,
) -> Result<usize, diesel::result::Error> {
    let mut conn = get_db();

    let new_task = NewTask {
        body: task_body.to_string(),
        completed: false,
        list_id: parent_id,
    };

    let list = lists
        .select(List::as_select())
        .find(parent_id)
        .first(&mut conn)
        .unwrap();

    match diesel::insert_into(tasks)
        .values(new_task)
        .execute(&mut conn)
    {
        Ok(res) => {
            display_list_items(&list.title);
            return Ok(res);
        }

        Err(err) => return Err(err),
    }
}

pub fn delete_task_record(task_id: i32) -> Result<usize, diesel::result::Error> {
    let mut conn = get_db();

    let task = tasks
        .select(Task::as_select())
        .find(task_id)
        .first(&mut conn)
        .unwrap();

    let parent_list = lists
        .select(List::as_select())
        .find(task.list_id)
        .first(&mut conn)
        .unwrap();

    match diesel::delete(tasks.find(task_id)).execute(&mut conn) {
        Ok(res) => {
            display_list_items(&parent_list.title);
            return Ok(res);
        }

        Err(err) => {
            return Err(err);
        }
    }
}

pub fn toggle_task_completion(task_id: i32) -> Result<Task, diesel::result::Error> {
    let mut conn = get_db();

    let mut task: Task = tasks
        .select(Task::as_select())
        .find(task_id)
        .first(&mut conn)
        .unwrap();

    let parent_list = lists
        .select(List::as_select())
        .find(task.list_id)
        .first(&mut conn)
        .unwrap();

    // SQLite doesn't support RETURN's on updates, so we have to do it like this instead
    match diesel::update(tasks.find(task_id))
        .set(completed.eq(!task.completed))
        .execute(&mut conn)
    {
        Ok(_) => {
            display_list_items(&parent_list.title);
            task.completed = !task.completed;
            return Ok(task);
        }

        Err(err) => {
            return Err(err);
        }
    }
}
