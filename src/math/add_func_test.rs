// test
use crate::math::add_func::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // 外部のスコープから（mod testsに）名前をインポートする便利なイディオム。
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add_func(1, 2), 3);
    }

    // #[test]
    // fn test_bad_add() {
    //     assert_eq!(Add_func(1, 2), 4);
    // }
}
