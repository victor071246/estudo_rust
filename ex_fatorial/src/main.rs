
use std::io;

    fn convert_to_int(data_input: & String) -> i32 {
        let x = data_input.trim().parse::<i32>().unwrap();
        x
    }

    fn main(){

        let mut entrada_fatorial = String::new();
        println!(" Insira o  valor do fatorial: ");
        io::stdin().read_line(&mut entrada_fatorial).expect("Erro ao ler entrada_fatorial");
        let mut acumulador = 0;
        let mut fatorial = convert_to_int(&entrada_fatorial);

       while  fatorial > 1 {
            acumulador += fatorial * (fatorial - 1);
            fatorial -= 1;
       }
       println!("O valor de acumulador é: {}", acumulador);


    //    5 | 0 + 5 * 4 | 4 | 20 + 4 * 3 | 3 | 32 + 3 * 2 | 2 | 38 + 2 * 1 | 40;
    //                 

    }