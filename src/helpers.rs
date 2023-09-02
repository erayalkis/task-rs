use crate::schema::lists::dsl::*;
use crate::{get_db, models::*};
use diesel::prelude::*;

fn get_list_items(list_name: String) {
    let mut conn = get_db();
    let res = lists
        .filter(title.eq(list_name))
        .select(List::as_select())
        .load(&mut conn);
}
