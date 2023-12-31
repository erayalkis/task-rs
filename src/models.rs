use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::lists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct List {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::lists)]
pub struct NewList {
    pub title: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub body: String,
    pub completed: bool,
    pub list_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewTask {
    pub body: String,
    pub completed: bool,
    pub list_id: i32,
}
