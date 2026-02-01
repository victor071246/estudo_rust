fn selection_sort(vector: &mut [i32]){
    let n = vector.len();

    for i in 0..n {
        let mut min_idx = i;

        // procura o menor elemento do intervalo [i+1, n]
        for j in (i + 1)..n {
            if vector[j] < vector[min_idx] {
                min_idx = j;
            }
        }

        // troca vector[i] com vector[min_idx] se necessário
        if min_idx != i {
            vector.swap(i, min_idx);
        }
    }
}

fn main() {
    let mut data = vec![64, 25, 12, 22, 11];
    println!("Antes: {:?}", data);
    selection_sort(&mut data);
    println!("\nDepois: {:?}", data);
}


