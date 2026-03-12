mod bank_account;

use bank_account::BankAccount;

fn main() {
    // Demonstrate the BankAccount struct
    let mut account = BankAccount::new(100.0);
    println!("Started with balance: ${}", account.balance());

    account.deposit(50.0);
    println!("After depositing $50: ${}", account.balance());

    account.withdraw(30.0);
    println!("After withdrawing $30: ${}", account.balance());

    account.apply_interest(0.1);
    println!("After applying 10% interest: ${}", account.balance());
}
