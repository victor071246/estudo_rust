fn dobro(num: i32) -> i32{
     2*num;
}

fn maior(a: i32, b: i32) -> i32 {
    if a >= b{
        return a;
    }
    else{
        return b;
    }
}

fn alguma_fn(par_a: i32, par_b: i128) -> f32{
    println!("Essa funçãodevolve umvalor flutuante");
    10.1
}

fn main(){
    println!("O maior número entre 4 e 5 é {}", maior(4, 5));
}
