mod account;
use account::{BankAccount, Currency, Account};

fn main() {

    let mut account = Account::new(1000.0, Currency::USD);

    account.deposit(500.0);
    println!("new balance {:.2}", account.check_balance());

    let withdraw_success = account.withdraw(100.0);
    if withdraw_success{
        println!("Withdraw success, new balance: {:.2}", account.check_balance());
    } else {
        println!("Withdraw error (insufficient funds) actual balance: {:.2}", account.check_balance());

    }

    account.check_balance();


}
