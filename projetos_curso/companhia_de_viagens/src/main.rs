mod viagem;
use viagem::{Dados_Passageiros, Dados_Voos, adicionar_passageiro, adicionar_voo, exibir_passageiros, exibir_voos};

// Crie um programa principal
fn main() {
    let mut dados_passageiros: Vec<Dados_Passageiros> = Vec::new();
    let mut dados_voos: Vec<Dados_Voos> = Vec::new();

// Adicione alguns passageiros

    adicionar_passageiro(&mut dados_passageiros, String::from("João"),String::from("123ABC"), 18);
    adicionar_passageiro(&mut dados_passageiros, String::from("Maria"), String::from("456DEF"), 58);
    adicionar_passageiro(&mut dados_passageiros, String::from("Pedro"), String::from("789GI"), 23);

// Adicionar alguns vôos

    adicionar_voo(&mut dados_voos, String::from("Vôo101"), String::from("São Paulo"), String::from("Rio de Janeiro"), String::from("30/06/2020"), String::from("09:00\n"));

    adicionar_voo(&mut dados_voos, String::from("Vôo102"), String::from("Rio de Janeiro"), String::from("São Paulo"), String::from("01/07/2020"), String::from("09:00"));

// Exibir os passageiros

    exibir_passageiros(&dados_passageiros);

// Exibir vôos

    exibir_voos(&dados_voos);
}
