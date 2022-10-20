use serde::{Deserialize, Serialize};
use std::process::Command;
use std::{env, fs::File, io::BufReader, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub tool: String,
    pub args: Vec<String>,
    pub comment_before: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Props {
    pub tasks: Vec<Task>,
}

impl Task {
    pub fn run(path: &str) -> std::io::Result<()> {
        let current_dir;
        if path.is_empty() {
            current_dir = env::current_dir()?;
        } else {
            current_dir = Path::new(path).to_path_buf();
        }

        let mut props_path = current_dir.clone();
        props_path.push("grasp.json");
        // Open the file in read-only mode with buffer.
        let file = match File::open(props_path) {
            Ok(file) => file,
            Err(e) => {
                println!("Error: Cant find 'grasp.json'");
                return Err(e.into());
            }
        };

        let reader = BufReader::new(file);

        let props: Props =
            serde_json::from_reader(reader).expect("Can't parse properties file 'grasp.json'.");

        for task in props.tasks {
            println!("{}", task.comment_before);

            Command::new(&task.tool).args(&task.args).status().expect(
                &format!("Faild execute process: {} {:?}", &task.tool, &task.args).to_string(),
            );
        }

        Ok(())
    }
}
