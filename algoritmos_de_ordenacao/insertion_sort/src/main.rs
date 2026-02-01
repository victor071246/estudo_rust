
/*
    
    (3, 4, 2, 34, -1, 5, 7)
    (3, 2, 4, 34, -1, 5, 7)
    (2, 3, 4, 34, -1, 5, 7)
    (2, 3, 4, 34, -1, 5, 7)
    (2, 3, 4, -1, 34, 5, 7)
    (2, 3, -1, 4, 34, 5, 7)
    (2, -1, 3, 4, 34, 5, 7)
    (-1, 2, 3, 4, 34, 5, 7)
    (-1, 2, 3, 4, 5, 34, 7)
    (-1, 2, 3, 4, 5, 7, 34)


 */

fn insertion_sort(vetor: &mut [i32]) {
    let n = vetor.len();

    for i in 1..n{
        let key = vetor[i];
        let mut j = i;

        // Desloca para a direita todos os elementos maiores que a key
        while j > 0 && vetor[j - 1] > key {
            vetor[j] = vetor[j -1];
            j -= 1
        }

        // Insere a key na posição correta
        vetor[j] = key;

    }
}

fn main() {
    let mut data = vec![5, 2, 4, 6, 1, 3];
    println!("Antes: {:?}", data);
    insertion_sort(&mut data);
    println!("\nDepois: {:?}", data);

}
