fn main() {

    {
        let minha_string = String::from("Oi meu nome é João");
        println!("{}", minha_string);
        println!("{}", minha_string.replace("João", "Henrique"));
    }

    {
        let minha_string = String::from("Fui hoje\n ao mercado\n comprar arroz");

        for i in minha_string.lines(){
            println!("( {} )", i)
        }
    }

    {
        let minha_string = String::from("Minha+sogra+é+muito+feliz");
        let token: Vec<&str> = minha_string.split("+").collect();
        println!("{:?}", token);
        println!("{}", token[4]);
    }

    {
        let minha_string = String::from("   Meu nome é João  ");
        println!("{}", minha_string);
        println!("{}", minha_string.trim());
    }

    {
        let minha_string = String::from("D");
        match minha_string.chars().nth(6){
            Some(entry) => println!("Sucesso ! O caractere da 6ª posição é {}", entry),
            None => println!("Erro, não existe o 6º caractere")
        }
    }

}