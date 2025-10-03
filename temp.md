├── main.rs        // エントリーポイント
├── db.rs          // 共通のDbPool定義と初期化
├── models.rs      // Todo, Userなどのモデル
└── handlers/
    ├── mod.rs     // pub useでまとめる
    └── todo.rs    // Todo関連のハンドラ群