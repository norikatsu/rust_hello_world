mod add_func; // ここで add_func.rs ファイルをプライベートで読み込み
pub use add_func::add_func; // 上記の内部関数を pub 公開

//for Test
#[cfg(test)]
mod add_func_test; // ここで add_func.rs ファイルをプライベートで読み込み
