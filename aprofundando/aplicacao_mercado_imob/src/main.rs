mod imobiliaria;
use imobiliaria::Imobiliaria;

fn main() {
    let mut imobiliaria = Imobiliaria{
        nome: String::from("Imobiliária ABC"),
        endereco: String::from("Rua dos bobos, 0"),
        imoveis: Vec::new()
    };

    imobiliaria.novo_imovel(String::from("Rua dos bobos, 1"), 200_000.0, 3, 2, 136.0);
    imobiliaria.listar_imoveis();
}
