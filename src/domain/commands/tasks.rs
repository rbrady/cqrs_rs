extern crate chrono;
use chrono::NaiveDate;

pub struct CreateTaskCommand {
    pub title: String,
    pub description: String,
    pub assigner: String,
    pub due_date: NaiveDate,
}