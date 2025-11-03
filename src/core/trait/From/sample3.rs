// ジェネリクス関数での柔軟性
fn create_and_process<T>(value: i32) -> T 
where 
    T: From<i32>  // i32からTに変換可能な型
{
    value.into()  // ← 型推論でTに変換される
}

fn main() {
    let a: MyStructA = create_and_process(42);  // MyStructAとして取得
    let b: MyStructB = create_and_process(42);  // MyStructBとして取得（実装されていれば）
}