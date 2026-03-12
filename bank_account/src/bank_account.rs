#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // ignore negative amounts
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // only allow positive amounts up to the current balance
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    // Bonus challenge: increase balance by rate (e.g. 0.05 for 5%)
    // I chose 5% to be the given interest rate
    pub fn apply_interest(&mut self, rate: f64) {
        if rate > 0.0 {
            self.balance += self.balance * rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        let acct = BankAccount::new(10.0);
        assert!((acct.balance() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_deposit() {
        let mut acct = BankAccount::new(10.0);
        acct.deposit(25.0);
        assert!((acct.balance() - 35.0).abs() < 1e-10);
    }

    #[test]
    fn test_withdraw() {
        let mut acct = BankAccount::new(50.0);
        acct.withdraw(20.0);
        assert!((acct.balance() - 30.0).abs() < 1e-10);
    }

    #[test]
    fn test_balance() {
        let acct = BankAccount::new(30.0);
        assert!((acct.balance() - 30.0).abs() < 1e-10);
    }

    #[test]
    fn test_deposit_negative() {
        let mut acct = BankAccount::new(20.0);
        acct.deposit(-5.0);
        assert!((acct.balance() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_withdraw_negative() {
        let mut acct = BankAccount::new(20.0);
        acct.withdraw(-10.0);
        assert!((acct.balance() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_withdraw_too_much() {
        let mut acct = BankAccount::new(20.0);
        acct.withdraw(50.0);
        assert!((acct.balance() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_interest() {
        let mut acct = BankAccount::new(100.0);
        acct.apply_interest(0.05);
        assert!((acct.balance() - 105.0).abs() < 1e-10);
    }
}