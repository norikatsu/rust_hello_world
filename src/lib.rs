// for lib crate  ライブラリクレートは lib.rs をトップとする

// 通常は沢山のmoduleを利用する
use std::error::Error;

pub mod math; // 基本  pub にする必要はない・・・ が 統合テスト側で math を参照するなら pub がないと見えない
pub use math::add_func;

pub fn run() -> Result<(), Box<dyn Error>> {
    // CLIだったらコマンドパーススタートの関数、APIならAPIスタートの関数など

    let a = 10;
    let b = 30;
    let c = add_func(a, b);

    println!("Add = {}", c);
    Ok(())
}
