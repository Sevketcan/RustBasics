// Define a Trait called Account with deposit, withdraw, and balance methods
trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

// Define a struct BankAccount with account_number, holder_name, and balance fields
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

// Implement the Account Trait for BankAccount
impl Account for BankAccount {
    // Method to deposit money
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Deposit amount must be greater than zero.".to_string())
        } else {
            self.balance += amount;
            println!(
                "Deposited ${:.2} into account {}. New balance: ${:.2}",
                amount, self.account_number, self.balance
            );
            Ok(())
        }
    }

    // Method to withdraw money
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Withdrawal amount must be greater than zero.".to_string())
        } else if amount > self.balance {
            Err("Insufficient balance for withdrawal.".to_string())
        } else {
            self.balance -= amount;
            println!(
                "Withdrew ${:.2} from account {}. New balance: ${:.2}",
                amount, self.account_number, self.balance
            );
            Ok(())
        }
    }

    // Method to get the current balance
    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Step 1: Create two BankAccount instances
    let mut account1 = BankAccount {
        account_number: 101,
        holder_name: String::from("Alice"),
        balance: 500.0,
    };

    let mut account2 = BankAccount {
        account_number: 102,
        holder_name: String::from("Bob"),
        balance: 300.0,
    };

    // Step 2: Perform deposit and handle any errors using a match statement
    println!("Attempting to deposit into Alice's account...");
    match account1.deposit(200.0) {
        Ok(_) => println!("Deposit successful!"),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nAttempting to deposit a negative amount into Alice's account...");
    match account1.deposit(-50.0) {
        Ok(_) => println!("Deposit successful!"),
        Err(e) => println!("Error: {}", e),
    }

    // Step 3: Perform withdrawal and handle any errors using a match statement
    println!("\nAttempting to withdraw from Bob's account...");
    match account2.withdraw(150.0) {
        Ok(_) => println!("Withdrawal successful!"),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nAttempting to withdraw more than the balance from Bob's account...");
    match account2.withdraw(500.0) {
        Ok(_) => println!("Withdrawal successful!"),
        Err(e) => println!("Error: {}", e),
    }

    // Step 4: Print final balances
    println!(
        "\nFinal Balances:\n{}'s Account: ${:.2}\n{}'s Account: ${:.2}",
        account1.holder_name,
        account1.balance(),
        account2.holder_name,
        account2.balance()
    );
}
