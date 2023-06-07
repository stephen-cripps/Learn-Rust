struct BankAccount {
    account_holder: String,
    balance: i32,
}

impl BankAccount {
    // Create a new BankAccount instance
    fn new(account_holder: String) -> BankAccount {
        BankAccount {
            account_holder,
            balance: 0,
        }
    }

    // Deposit an amount into the account
    fn deposit(&mut self, amount: i32) {
        self.balance += amount
    }

    // Withdraw an amount from the account if sufficient funds are available
    fn withdraw(&mut self, amount: i32) {
        self.balance -= amount
    }

    // Get the current account balance
    fn get_balance(&self) -> i32 {
        self.balance
    }

    fn get_name(&self) -> &str {
        &self.account_holder[..]
    }
}

fn main() {
    let mut account = BankAccount::new(String::from("John Doe"));

    account.deposit(100);
    println!(
        "{}'s balance after deposit: {}",
        account.get_name(),
        account.get_balance()
    );

    account.withdraw(50);
    println!(
        "{}'s balance after withdrawal: {}",
        account.get_name(),
        account.get_balance()
    );
}
