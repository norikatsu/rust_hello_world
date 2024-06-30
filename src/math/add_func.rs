// math func


pub fn add_func(x: i32, y: i32) -> i32 {
   

    if x == 0 {
        println!(" x == 0");
    }

    let z = x + y;




    if z > 5 {
        println!("Large than 5 : {}", z);
    } else {
        println!("Small than 5 : {}", z);
    }

    z
}


