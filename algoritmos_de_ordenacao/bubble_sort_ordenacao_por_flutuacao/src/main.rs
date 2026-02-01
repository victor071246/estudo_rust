fn swap_array(lista:&mut [i32;7], i: usize, j: usize){
    let temp: i32 = lista[i];
    lista[i] = lista[j];
    lista[j] = temp;
}

// Bubble Sort
// o bubble sort coloca todos os menores elementos mais a esquerda comparando sempre o menor elemento a direita se é maior que seu elemento da esquerda, se for, os substitui

    //[10, 5, 66, 7, -2]
    //[10, 5, 66, -2, 7]
    //[10, 5, -2, 66, 7]
    //[10, -2, 5, 66, 7]
    //[-2, 10, 5, 66, 7]
    //[-2, 10, 5, 7, 66]
    //[-2, 5, 10, 7, 66]
    //[-2, 5, 7, 10, 66]
fn main() {
    let mut array: [i32;7] = [10, 23, 4, 5, 66, 7, -3];
    println!("{:?}", array);

    for i in 0..array.len(){
        for j in ((i+1)..array.len()).rev(){
            println!("i = {}, j = {}", i, j);
            println!("j-1 = {}, j = {}", array[j-1], array[j]);
            if array[j -1] > array[j]{
                // função-a-fazer
                swap_array(&mut array, j-1, j);
            }

            println!("{:?}", array);
        }
    }
}
