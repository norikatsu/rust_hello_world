// integration test 結合テスト
// 基本 「バイナリクレート(main.rs)」に対しては 統合テストは書けない
// 「ライブラリクレート(lib.rs)」専用

use hello_world::math::add_func;

#[test]
fn test_add_integration() {
    assert_eq!(add_func(1, 2), 3);
    assert_eq!(add_func(4, 2), 6);
}
