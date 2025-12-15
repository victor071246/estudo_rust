use core::num;
//HashSet
use std::collections::HashSet;

fn contains_duplicates(nums: &[i32]) -> bool{
    let mut seen = HashSet::new();

    for &num in nums{
        if !seen.insert(num){
            // Se o valor já existe dentro do HashSet, ou seja valor duplicado
            return  true;
        }
    }
    // Não foi encontrada duplicatas
    false
}

fn main(){
    let nums1 = vec![1, 2, 3, 4, 5];
    let nums2 = vec![1, 2, 3, 4, 1];

    println!("Array 1 contém duplicatas?: {}", contains_duplicates(&nums1));
    println!("Array 2 contém duplicatas?: {}", contains_duplicates(&nums2));
}