// 「Wrapper構造体のインスタンスが生きている間、valueが参照しているデータ（&'a T）も有効でなければならない」
// つまり、「Wrapperが生きている＝valueの参照先も生きている」ことをRustが保証します
struct Wrapper<'a, T> {
    value: &'a T,
}

