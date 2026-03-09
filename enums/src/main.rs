use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(dir) => {
            let status = Command::new("ls")
                .arg(&dir)
                .status()
                .expect("Failed to execute ls");
            if !status.success() {
                eprintln!("Error: could not list '{}'", dir);
            }
        }

        FileOperation::Display(file) => {
            let status = Command::new("cat")
                .arg(&file)
                .status()
                .expect("Failed to execute cat");
            if !status.success() {
                eprintln!("Error: could not display '{}'", file);
            }
        }

        FileOperation::Create(file, content) => {
            let cmd = format!("echo '{}' > {}", content, file);
            let output = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .output()
                .expect("Failed to execute command");
            if output.status.success() {
                println!("File '{}' created successfully.", file);
            } else {
                eprintln!("Failed to create file '{}'.", file);
            }
        }

        FileOperation::Remove(file) => {
            let status = Command::new("rm")
                .arg(&file)
                .status()
                .expect("Failed to execute rm");
            if status.success() {
                println!("File '{}' removed successfully.", file);
            } else {
                eprintln!("Error: could not remove '{}'.", file);
            }
        }

        FileOperation::Pwd => {
            Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");
        }
    }
}

fn prompt(label: &str) -> String {
    print!("{}", label);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn print_menu() {
    println!("\nFile Operations Menu:");
    println!("1. List files in a directory");
    println!("2. Display file contents");
    println!("3. Create a new file");
    println!("4. Remove a file");
    println!("5. Print working directory");
    println!("0. Exit");
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        print_menu();
        let choice = prompt("\nEnter your choice (0-5): ");

        let operation = match choice.as_str() {
            "0" => {
                println!("Goodbye!");
                break;
            }
            "1" => {
                let dir = prompt("Enter directory path: ");
                FileOperation::List(dir)
            }
            "2" => {
                let file = prompt("Enter file path: ");
                FileOperation::Display(file)
            }
            "3" => {
                let file = prompt("Enter file path: ");
                let content = prompt("Enter content: ");
                FileOperation::Create(file, content)
            }
            "4" => {
                let file = prompt("Enter file path: ");
                FileOperation::Remove(file)
            }
            "5" => FileOperation::Pwd,
            _ => {
                println!("Invalid option. Please enter a number between 0 and 5.");
                continue;
            }
        };

        perform_operation(operation);
    }
}