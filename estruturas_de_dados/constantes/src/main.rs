const NUMERO_PI : f32 = 3.14159;

fn comprimento_cinrcunferencia(r:f32) -> f32{
    let c = 2f32*r*NUMERO_PI;
    c

}

fn main(){

    println!("A circunferência de raio 2,32 possui o comprimento de {}", comprimento_cinrcunferencia(2.32));

}