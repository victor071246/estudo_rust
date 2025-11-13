use std::io;

fn convert_to_int(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unrwap();
    x
}

fn main(){
    
}