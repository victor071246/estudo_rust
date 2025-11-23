enum Pagamento{
    Dinheiro(f32),
    CreditoCartao(bool, f32),
    DebitoCartao(bool, f32)
}

fn main(){

    let pessoa_pagamento = Pagamento::CreditoCartao(true, 100f32);
    match pessoa_pagamento{
        Pagamento::Dinheiro(f) => println!("A pessoa pagou em dinheiro {} reais!", f),
        Pagamento::CreditoCartao(true, x) => println!("A pessoa pagou com o cartão de crédito {}!", x),
        Pagamento::CreditoCartao(false, x) => println!("O pagamento com o cartão de crédito não funcionou! O valor {} não foi pago", x),
        _ => {}
    }

}