fn main() {
    struct MyStructA {
        value: i32,
    }
    
    struct MyStructB {
        value: i32,
    }

    impl From<i32> for MyStructA {
        fn from(item: i32) -> Self {
            MyStructA { value: item }
        }
    }
    
    impl From<i32> for MyStructB {
        fn from(item: i32) -> Self {
            MyStructB { value: item }
        }
    }

    // === 基本的な使用方法 ===
    let a1: MyStructA = MyStructA::from(42);  // From::from使用
    let a2: MyStructA = 42.into();            // Into::into使用
    
    println!("a1.value: {}", a1.value);
    println!("a2.value: {}", a2.value);
    
    // === 型推論の違い ===
    // ✅ intoは左辺から型を推論
    let a3: MyStructA = 100.into();
    let b1: MyStructB = 100.into();  // 同じコードで異なる型に変換
    
    // ✅ fromは明示的に型を指定
    let a4 = MyStructA::from(200);
    let b2 = MyStructB::from(200);
    
    // === 関数の引数での使用 ===
    fn take_struct_a(s: MyStructA) {
        println!("MyStructA: {}", s.value);
    }
    
    fn take_struct_b(s: MyStructB) {
        println!("MyStructB: {}", s.value);
    }
    
    // ✅ intoは関数の引数の型から推論される
    take_struct_a(300.into());  // i32 → MyStructA
    take_struct_b(400.into());  // i32 → MyStructB
    
    // ✅ fromも使える（ただし冗長）
    take_struct_a(MyStructA::from(500));
    take_struct_b(MyStructB::from(600));
    
    // === ジェネリック関数での使用 ===
    fn convert_and_print<T>(value: i32) 
    where 
        T: From<i32> + std::fmt::Debug,
    {
        let converted: T = value.into();  // 型推論でTに変換
        println!("Converted: {:?}", converted);
    }
    
    convert_and_print::<MyStructA>(700);  // MyStructAに変換
    convert_and_print::<MyStructB>(800);  // MyStructBに変換
}