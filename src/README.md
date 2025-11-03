# モジュールが階層化されている場合の `mod` 宣言例

ディレクトリ構成が以下のようになっているとします。

```
src
├── main.rs
├── core.rs
└── core
    ├── lifetime.rs
    └── lifetime
        └── sample1.rs
```

この場合、`main.rs` では

```rust
mod core;
```

だけでOKです。

そして、src/core/mod.rsとsrc/core/lifetime/mod.rsを用意し、それぞれで

// src/core/mod.rs
```rust
pub mod lifetime;
```

// src/core/lifetime/mod.rs
```rust
pub mod sample1;
```

と書きます。

こうすると、main.rsはとてもスッキリしますし、Rustのモジュールシステムの慣習にも合っています。

まとめ：

main.rsでは mod core; だけ
階層ごとにmod.rsを作り、そこでpub mod ...を書く
この方法が推奨される書き方です。