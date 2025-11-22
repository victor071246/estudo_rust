fn main(){
    
    let tupla = (12, "valores", 3.14, (1, 2, 3));
    let (a, b, c, d) = tupla;

    println!("O valor de a é {}", a);
    println!("O valor de b é {}", b);
    println!("O valor de c é {}", c);
    println!("O valor de c é {:?}", d);


    }