enum Currency{
    USD,
    EURM,
    GBP,
    JPY,
    CHF,
    AUD,
    CAD,
    BRL,
}

struct Account{
    balance: f64,
    currency: Currency
}

impl Account {
    fn new(balance: f64, currency: Currency) -> Self {
        Self{ balance, currency}
    }

    fn deposit(&mut self, amount: f64){
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> bool{
        if self.balance >= amount{
            self.balance -= amount;
            return true
        }
        false
    }

    fn check_balance(&self) -> f64{
        self.balance
    }

    fn convert_to(&self, target_currency: Currency) -> f64{
        // Utilize uma API de taxas de câmbio para converter seu saldo para a moeda de destino
        let exchange_rate = 1.0; // exemplo
        self.balance * exchange_rate
    }
}

fn main() {
    let mut account = Account::new(1000.0, Currency::USD);
    println!("Saldo inicial da nossa conta: {}", account.check_balance());

    account.deposit(1000.0);
    println!("Saldo após um depósito de 1000 reais: {}", account.check_balance());

    let withdraw_success = account.withdraw(200.0);
    
    if withdraw_success{
        println!("Saldo após saque: {}", account.check_balance())
    } else {
        println!("Saldo não pode ser efetuado, valor de saque menor que o valor da conta");
    }
}
