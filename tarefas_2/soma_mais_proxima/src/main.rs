// Consturir uma funçao que recebe 2 parametros de entrada, um vetor e um número alvo, pegar 3 elementos do vetor ou a soma desses 3 elementos tem que ser o mais proximo de cinco. A função deve encontrar três numeros no vetor cuja soma seja a mais proxima possivel do numero alvo e retornar essa soma

use std::i32;

fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {

    let mut nums = nums;
    nums.sort();

    let mut closest_sum = i32::MAX;
    let mut min_diff = i32::MAX;

    for i in 0..nums.len()-2{
        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let current_sum = nums[i] + nums[left] + nums[right];
            let current_diff = (target - current_sum).abs();

            if current_diff < min_diff {
             closest_sum = current_sum;
             min_diff = current_diff;
            }

            if current_sum < target {
                left += 1;
            }
            else {
                right -= 1;
            }

        }
    }

    closest_sum
}

fn main(){
    let nums = vec![-1, 2, 1, -4];
    let target = 1;

    let result = three_sum_closest(nums, target);

    println!("A soma mais próxima do alvo é: {}", result)
}