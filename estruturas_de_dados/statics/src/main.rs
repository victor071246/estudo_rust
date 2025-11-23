#![allow(static_mut_refs)]

static mut STATIC_VARIABLE: i32 = 15;

fn main(){

    unsafe {
    println!("O valor da STATIC_VARIAVEL é {}", STATIC_VARIABLE);

    }
}