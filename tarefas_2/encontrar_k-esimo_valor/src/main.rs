fn encontrar_kesimo_maior(nums: &mut Vec<i32>, k: usize) -> i32{

    // Ordena de forma decrescente
    nums.sort_by(| a, b| b.cmp(a));

    return nums[k-1];
}

fn main(){

    // Exemplo de array e valor de k
    let nums = vec![3, 2, 1, 5, 6, 4];
    let k = 20;

    // Chama a função para encontrar o k-ésimo maior elemento
    let resultado = encontrar_kesimo_maior(&mut nums.clone(), k);

    // Exibe o resultado
    println!("O {}º maior elemento é {}", k, resultado);
}