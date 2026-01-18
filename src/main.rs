use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};
use std::fs;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Deserialize, Serialize)]
struct TodoItem {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(&self)?;

        fs::write("db.json", content)?;
        Ok(())
    }

    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        match fs::read_to_string("db.json") {
            Ok(content) => {
                let list: TodoList = serde_json::from_str(&content)?;
                Ok(list)
            }
            Err(_) => Ok(TodoList::new())
        }
    }

    fn remove(&mut self, id: usize) -> bool {
        if let Some(index) = self.items.iter().position(|i| i.id == id) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let check = if self.completed { "[x]" } else { " [ ] " };
        write!(f, "{} {} - {}", check, self.id, self.title)
    }
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Rust로 만든 간단한 Todo 리스트 CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        title: String,
    },

    List,

    Complete {
        id: usize
    },

    Delete {
        id: usize,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();

    let mut todo_list = TodoList::load()?;

    match &cli.command {
        Commands::Add { title } => {
            let new_id = todo_list.items.len() + 1;
            let new_item = TodoItem {
                id: new_id,
                title: title.clone(),
                completed: false,
            };
            todo_list.items.push(new_item);

            todo_list.save()?;
            println!("할 일이 추가되었습니다: [{}] {}", new_id, title);
        }
        Commands::List => {
            let items = &todo_list.items;

            if items.is_empty() {
                println!("  (비어 있음)")
            } else {
                for item in items {
                    println!("  {}", item);
                }
            }
        }
        Commands::Complete { id } => {
            if let Some(item) = todo_list.items.iter().position(
                |i| i.id == *id) {
                todo_list.items[item].completed = true;
                todo_list.save()?;
                println!("할 일이 완료되었습니다");
            } else {
                println!("ID {}번 할 일을 찾을 수 없습니다.", id)
            }
        }
        Commands::Delete { id } => {
            if todo_list.remove(*id) {
                todo_list.save()?;
                println!("삭제되었습니다: ID {}", id)
            } else {
                println!("ID {}번 할 일을 찾을 수 없습니다.", id)
            }
        }
    }

    Ok(())
}
