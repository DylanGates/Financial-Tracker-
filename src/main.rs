use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
enum TransactionType {
    Income,
    Expense,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    id: u32,
    amount: f64,
    category: String,
    transaction_type: TransactionType,
    time_stamp: DateTime<Utc>,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FinanceTracker {
    transactions: Vec<Transaction>,
}

// trait FinanceOperations {
//     fn add_transaction(&mut self, transaction: Transaction);
//     fn total_income(&self) -> f64;
//     fn total_expense(&self) -> f64;
//     fn balance(&self) -> f64;
// }   

impl FinanceTracker {
    fn add_transaction(&mut self, amount: f64, category: String, transaction_type: TransactionType, description: String) {
        let id = self.transactions.len() as u32 + 1;
        let transaction = Transaction::new(
            id,
            amount,
            category,
            transaction_type,
            description
        );
        self.transactions.push(transaction);
    }

    fn total_balance(&self) -> f64 {
        let income: f64 = self.transactions.iter()
            .filter(|t| matches!(t.transaction_type, TransactionType::Income))
            .map(|t| t.amount)
            .sum();

        let expense: f64 = self.transactions.iter()
            .filter(|t| matches!(t.transaction_type, TransactionType::Expense))
            .map(|t| t.amount)
            .sum();

        income - expense
    }

    fn list_transactions(&self) {
        if self.transactions.is_empty() {
            println!("No transactions recorded.");
            return;
        }

        for transaction in &self.transactions {
            println!(
                "ID: {}, Amount: {:.2}, Category: {}, Type: {:?}, Time: {}, Description: {}",
                transaction.id,
                transaction.amount,
                transaction.category,
                transaction.transaction_type,
                transaction.time_stamp,
                transaction.description
            );
        }
    }
}

impl Transaction {
    fn new(id: u32, amount: f64, category: String, transaction_type: TransactionType, description: String) -> Self {
        Transaction {
            id,
            amount,
            category,
            transaction_type,
            time_stamp: Utc::now(),
            description,
        }
    }
}

fn load_transactions_from_file(file_path: &str) -> Vec<Transaction> {
    if !Path::new(file_path).exists() {
        return Vec::new();
    }

    let mut file = File::open(file_path).expect("Unable to open file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read file");

    serde_json::from_str(&data).expect("Unable to parse JSON")
}

fn save_transactions(transactions: &Vec<Transaction>, file_path: &str) {
    let data = serde_json::to_string(transactions).expect("Unable to serialize transactions");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .expect("Unable to open file for writing");
    file.write_all(data.as_bytes()).expect("Unable to write data to file");
}



fn main() {
    let mut tracker = FinanceTracker { transactions: Vec::new() };
    let path = "transactions.json";

    tracker.transactions = load_transactions_from_file(path);

    tracker.add_transaction(100.0, "Salary".to_string(), TransactionType::Income, "Monthly salary".to_string());
    tracker.add_transaction(50.0, "Groceries".to_string(), TransactionType::Expense, "Weekly groceries".to_string());

    println!("##### Personal Finance Tracker #####");
    println!("1. Add Income");
    println!("2. Add Expense");
    println!("3. View Transactions");
    println!("4. View Total Balance");
    println!("5. Exit");
    
    loop {
        println!("Enter your choice: ");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let (amount, category, description) = get_transaction_details();
                tracker.add_transaction(amount, category, TransactionType::Income, description);
                save_transactions(&tracker.transactions, path);
                println!("Income added successfully.");
            }

            "2" => {
                let (amount, category, description) = get_transaction_details();
                tracker.add_transaction(amount, category, TransactionType::Expense, description);
                save_transactions(&tracker.transactions, path);
                println!("Expense added successfully.");
            }

            "3" => {
                tracker.list_transactions();
            }

            "4" => {
                println!("Your total balance is : {:.2}", tracker.total_balance());
            }

            "5" => {
                println!("Exiting...");
                break;
            }

            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}

fn get_transaction_details() -> (f64, String, String) {
    println!("Enter amount: ");
    let mut amount_str = String::new();
    std::io::stdin().read_line(&mut amount_str).expect("Failed to read line");
    let amount: f64 = amount_str.trim().parse().expect("Please enter a valid number");

    println!("Enter category: ");
    let mut category = String::new();
    std::io::stdin().read_line(&mut category).expect("Failed to read line");

    println!("Enter description: ");
    let mut description = String::new();
    std::io::stdin().read_line(&mut description).expect("Failed to read line");

    (amount, category.trim().to_string(), description.trim().to_string())
}