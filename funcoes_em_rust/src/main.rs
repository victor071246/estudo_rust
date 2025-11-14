fn dobro(num: i32) -> i32{
    return 2*num;
}

fn maior(a: i32, b: i32) -> i32 {
    if a >= b{
        return a;
    }
    else{
        return b;
    }
}

fn main(){
    println!("O maior número entre 4 e 5 é {}", maior(4, 5));
}
