pub mod cmd {
    use cron::Schedule;
    use chrono::Local;
    use std::str::FromStr;
    use tauri::command;

    #[command]
    pub fn next_trigger_time(expression: &str) -> Vec<String> {
        let schedule = Schedule::from_str(expression);
        let mut result: Vec<String> = Vec::new();
        match schedule {
            Ok(sc) => {
                for datetime in sc.upcoming(Local).take(5) {
                    result.push(datetime.to_string());
                }
                result
            },
            Err(_e) => {
                println!("-> {}", _e);
                result
            }
        }
    }
}