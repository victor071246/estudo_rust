mod shapes;

use shapes::{Shape, Circle, Rectangle};

fn main() {

    let c = Circle{ radius: 5.0 };
    let r = Rectangle{ width: 10.0, height: 20.0};
    println!("A área do retângulo é: {}", c.area());
    println!("O perímetro do retângulo é: {}", c.perimeter());
    c.draw();

    println!();
    println!("A área do retângulo é: {}", r.area());
    println!("O perímetro do retângulo é: {}", r.perimeter());
    r.draw();
}
