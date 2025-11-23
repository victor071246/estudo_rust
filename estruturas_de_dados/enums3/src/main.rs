enum Pagamento{
    Dinheiro,
    CreditoCartao,
    DebitoCartao
}

fn main(){

    let pessoa_pagamento = Pagamento::CreditoCartao;
    match pessoa_pagamento{
        Pagamento::Dinheiro => println!("A pessoa pagou em dinheiro!"),
        Pagamento::CreditoCartao => println!("A pessoa pagou com o cartão de crédito!"),
        _ => {}
    }

}