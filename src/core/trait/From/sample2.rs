// メソッドチェーンでの使いやすさ
fn process_struct(s: MyStructA) -> MyStructA {
    // 何かの処理
    s
}

fn main() {
    // ✅ intoはメソッドチェーンで自然
    let result = 42.into().process_struct();
    
    // ❌ fromはメソッドチェーンで不自然
    let result = MyStructA::from(42).process_struct();
}