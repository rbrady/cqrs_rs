extern crate chrono;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Task {
    pub id: Option<u32>,
    pub title: String,
    pub description: String,
    pub assigner: String,
    pub due_date: NaiveDate,
}
