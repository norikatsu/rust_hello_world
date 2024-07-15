// 通常は沢山のmoduleを利用する
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    hello_world::run()
}
