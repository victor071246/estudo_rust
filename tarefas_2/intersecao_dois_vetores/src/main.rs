use std::collections::HashSet;

fn interseccao(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // Converter os vetores para HashSet para busca eficiente
    let conjunto1: HashSet<_> = nums1.into_iter().collect();
    let conjunto2: HashSet<_> = nums2.into_iter().collect();

    // Utilizar o método intersection no HashSet para encontrar elementos em comum
    let resultado: Vec<_> = conjunto1.intersection(&conjunto2).cloned().collect();
    resultado
}

fn main() {

    //Exemplo de uso

    let nums1 = vec![1, 3, 4, 1, 5, 6];
    let nums2 = vec![2, 4, 5];

    // Chamar a função para calcular a intersecção
    let resultado = interseccao(nums1, nums2);

    // Imprimir o resultado
    println!("Intersecção: {:?}", resultado);

}