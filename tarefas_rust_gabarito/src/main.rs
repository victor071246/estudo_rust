fn count(num: i32){

    for n in 1..(num + 1){
        println!("{}", n);
    }
}

fn count_down(num: i32){
    let mut n = num;
    while n > 0 {
        println!("{}", n);
        n -= 1;
    }
}

fn main() {
    println!("Counting up 1 to 10!");
    count(10);
    println!("Counting down 10 to 1!");
    count_down(10);
}
