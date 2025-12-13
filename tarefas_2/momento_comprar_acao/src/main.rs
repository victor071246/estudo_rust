fn lucro_maximo(prices: Vec<i32>) -> i32{

    if prices.is_empty(){
        return 0;
    }

    let mut preco_minimo = prices[0];
    let mut lucro_maximo = 0;

    for preco in prices.iter().skip(1){
        let lucro_atual = preco - preco_minimo;
        lucro_maximo = lucro_maximo.max(lucro_atual);
        preco_minimo = preco_minimo.min(*preco);
    }

    lucro_maximo
}

fn main(){

    let precos1 = vec![7, 1, 5, 3, 6, 4];
    let lucro1 = lucro_maximo(precos1.clone());
    println!("Exemplo 1: Lucro Máximo = {}", lucro1);

    println!();

    let precos2 = vec![7, 5, 4, 3, 2, 1];
    let lucro2 = lucro_maximo(precos2.clone());
    println!("Exemplo 2: Lucro Máximo = {}", lucro2);


}