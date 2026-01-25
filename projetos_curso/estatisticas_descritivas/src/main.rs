use estatisticas_descritivas::executar_estatisticas_descritivas;
fn main() {
    let numeros: Vec<i64> = vec![1, 1, 2, 3, 4, 4, 4, 5, 6, 0];
    executar_estatisticas_descritivas(numeros);
}
