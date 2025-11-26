use core::num;

fn rotacionar_array(nums: &mut [i32; 7], k: usize) {
    let n = nums.len();

    if n == 0 {
        println!("Array vazio, não há nada para rotacionar");
        return; // Array vazio, não há nada para rotacionar
    }

    let k = k % n; // Lidar com casos que k é maior que n
    nums.reverse(); // Inverte todo o array
    nums[0..k].reverse(); // Inverter os primeiros k elementos
    nums[k..].reverse(); // Inverter os elementos restantes
}

fn main(){

    let mut array = [1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    println!("Array original: {:?}", array);
    rotacionar_array(&mut array, k);
    println!("Array rotacionado: {:?}", array);


}