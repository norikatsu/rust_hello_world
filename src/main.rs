
mod math;
use math::add_func;


fn main() {
    let a = 10;
    let b = 30;
    let c = add_func(a,b);

    println!("Add = {}", c);
}


