use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config{
    pub benchmark: HashMap<String, i64>,
    pub daily_collect_cron_expr: Option<String>,
}
