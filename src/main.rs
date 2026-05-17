use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone)]
enum Category {
    Food,
    Rent,
    Misc,
    Entertainment,
}
impl Category {
    fn from_str(input: &str) -> Category {
        match input.to_lowercase().trim() {
            "food" => Category::Food,
            "rent" => Category::Rent,
            "entertainment" => Category::Entertainment,
            _ => Category::Misc,
        }
    }
}

#[derive(Debug, Clone)]
struct Expense {
    amount: f64,
    description: String,
    category: Category,
}
impl Expense {
    fn to_csv(&self) -> String {
        format!("{}|{}|{:?}", self.amount, self.description, self.category)
    }
}

fn save_to_file(expenses: &Vec<Expense>) {
    let mut file = File::create("expenses.csv").expect("Failed to create/open the save file!");

    let mut data = String::new();
    for exp in expenses {
        data.push_str(&exp.to_csv());
        data.push('\n');
    }

    file.write_all(data.as_bytes())
        .expect("Failed to write data to the disk!");

    println!("Save complete!");
}

fn load_expenses() {
    let filename = "expenses.csv";

    match fs::read_to_string(filename) {
        Ok(content) => {
            println!("Success! File content: {}", content);
        }
        Err(_) => {
            println!("Could not read file (it might not exist). Starting empty.");
        }
    }
}
fn load_from_file() -> Vec<Expense> {
    let mut expenses = Vec::new();
    let path = "expenses.csv";

    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            let parts: Vec<&str> = line.split('|').collect();

            if parts.len() == 3 {
                let amount: f64 = parts[0].parse().unwrap_or(0.0);
                let description = parts[1].to_string();
                let category = Category::from_str(parts[2]);

                expenses.push(Expense {
                    amount,
                    description,
                    category,
                });
            }
        }
    }
    expenses
}

fn input_ask(message: &str) -> String {
    println!("{}", message);
    print!("> ");
    io::stdout().flush().expect("Could not flush stdout");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    input.to_string()
}
fn main() {
    let mut exp_vec = if Path::new("expenses.csv").exists() {
        load_from_file()
    } else {
        Vec::new()
    };

    loop {
        let input = input_ask("Choose action");

        match input.as_str() {
            "exit" => {
                println!("Saving your data...");
                save_to_file(&exp_vec);
                println!("See ya later!");
                break;
            }
            _ => {}
        }
    }
}
