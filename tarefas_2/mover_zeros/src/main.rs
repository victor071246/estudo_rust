fn mover_zeros(nums: &mut Vec<i32>){
    let mut indice_nao_zero = 0;

    // Iterar através do array
    for i in 0..nums.len(){


        // [0, 1, 2, 0]
        // [1, 0, 2, 0] => indice = 1
        // [1, 2, 0, 0] => indice = 2

        // Se o elemento não for zero
        if nums[i] != 0{
            // Trocar o elemento não zero com o elemento no indice indice_nao_zero
            nums.swap(i, indice_nao_zero);
            // Avançar o índice indice_nao_zero
            indice_nao_zero += 1;
        }



    }
}

fn main(){

    let mut nums = vec![0, 1, 0, 3, 12, 5, 6, 0, 0, 1, 2, 3];

    // chamar a função para mover os zeros para o final
    mover_zeros(&mut  nums);

    println!("{:?}", nums);

}