// Define a Trait called Account with deposit, withdraw, and balance methods
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
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
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!(
            "Deposited ${:.2} into account {}. New balance: ${:.2}",
            amount, self.account_number, self.balance
        );
    }

    // Method to withdraw money
    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
            println!(
                "Withdrew ${:.2} from account {}. New balance: ${:.2}",
                amount, self.account_number, self.balance
            );
        } else {
            println!(
                "Insufficient balance in account {}. Withdrawal failed.",
                self.account_number
            );
        }
    }

    // Method to get the current balance
    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Create two BankAccount instances
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

    // Perform operations on account1
    println!("Account 1: {}'s Initial Balance: ${:.2}", account1.holder_name, account1.balance());
    account1.deposit(200.0); // Deposit money
    println!("Account 1 Balance: ${:.2}", account1.balance());

    // Perform operations on account2
    println!("\nAccount 2: {}'s Initial Balance: ${:.2}", account2.holder_name, account2.balance());
    account2.withdraw(150.0); // Withdraw money
    println!("Account 2 Balance: ${:.2}", account2.balance());

    // Final account balances
    println!(
        "\nFinal Balances:\n{}'s Account: ${:.2}\n{}'s Account: ${:.2}",
        account1.holder_name,
        account1.balance(),
        account2.holder_name,
        account2.balance()
    );
}
