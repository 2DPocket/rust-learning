//1. 型推論との相性
fn main() {
    // ❌ これは型推論できない場合がある
    let value = 42.from(); // ← 何の型に変換するかわからない
    
    // ✅ intoは左辺の型から推論できる
    let a: MyStructA = 42.into(); // ← MyStructAに変換することが明確
}
