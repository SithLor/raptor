//this function check if a binary is installed
fn check_binary(binary: &str) -> bool {
    let output = std::process::Command::new("which")
        .arg(binary)
        .output()
        .expect("Failed to execute command");

    output.status.success()
}

// this function check if arg for cli "y","yes","1" or "n","no","0"
fn cli_check_answer(answer: &str) -> bool {
    match answer {
        "y" | "yes" | "1" => true,
        "n" | "no" | "0" => false,
        _ => {
            println!("Invalid answer");
            println!("y, yes, 1 for true");
            println!("n, no, 0 for false");
            false
        }
    }
}
fn cli_length(data: Vec<String>)  {
    if data.len() < 2 {
        panic!("Please provide an action");
    }
}
fn cli_is_valid_action(data:Vec<String>,actions_list:Vec<&str>)  {
    if !actions_list.contains(&data[1].as_str()) {
        panic!("Invalid action");
    }
}

fn cli_paser(data: Vec<String>) {
    fn uninstall(ansewr: Option<bool>) {
        // this automates the uninstall process
        match ansewr {
            Some(true) => {
                println!("Uninstalling...");
            }
            Some(false) => {
                println!("Uninstall cancelled, no is not a valid answer");
            }
            //this if the user did not provide an answer after "backhoe uninstall"
            _ => {
                println!("Are you sure you want to uninstall? (y/n)");
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                if input.trim() == "y" {
                    println!("Uninstalling...");
                } else {
                    println!("Uninstall cancelled");
                }
            }
        }

        // check for
        // check the use want to uninstall ask for confirmation
    }

    let actions_list = vec!["setup", "uninstall"];
    
    cli_length(data.clone());

    cli_is_valid_action(data.clone(),actions_list);

    match data[1].as_str() {
        "setup" => {
            println!("Setting up...");
        }
        "uninstall" => {
            // this make the code clean by using a function not check the data in the function
            match Some(cli_check_answer(data[1].as_str())) {
                Some(true) => {
                    uninstall(Some(true));
                }
                Some(false) => {
                    uninstall(Some(false));
                }
                _ => {
                    uninstall(None);
                }
            }
        }
        _ => {
            println!("Invalid action");
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parsed_args = cli_paser(args);

    println!("Hello, world!");
}
