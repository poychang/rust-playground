# Rust Playground

Rust 官方網站 [www.rust-lang.org](https://www.rust-lang.org/)

安裝 Rust 環境及基本使用方式，請參考[新手入門](https://www.rust-lang.org/zh-TW/learn/get-started)

可以在 [crates.io](https://crates.io/) 找到各式各樣的 Rust 套件

## 更新 Rust

Rust 語言與編譯器有一個為其六週的發佈循環，並且每兩到三年 Rust 團隊會產出一個新的 Rust 版號（edition），例如 Rust 2015 、 Rust 2018 與 Rust 2021。

若要更新 Rust 環境，請使用 `rustup update` 指令。

## Rust 建置指令碼

在專案的資料夾中，建立 `build.rs` 檔案，這樣 Cargo 就會先編譯並執行此檔案的建置指令碼，然後再去建置整個專案。

例如：

```rust
fn main() {
    // 以下程式碼告訴 Cargo ，一旦指定的檔案 `src/hello.c` 發生了改變，就重新運行當前的建構指令碼
    println!("cargo:rerun-if-changed=src/hello.c");
    // 使用 `cc` 來建構一個 C 檔案，然後進行靜態連結
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
}
```

關於建置指令碼的使用場景：

- 建置所需的相依函示庫
- 在作業系統中尋找指定的函示庫
- 根據某個說明描述檔案生成一個 Rust 模組
- 執行一些平台相關的組態

## cargo 基本指令

詳請參考官方 [The Cargo Book](https://doc.rust-lang.org/cargo/index.html) 文件

- `cargo --version` 查看版本
- `cargo new [PROJECT_NAME]` 建立新專案
  - 若要建立函示庫專案，請使用 `cargo new --lib [LIBRARY_PROJECT_NAME]`
- `cargo build` 能建置您的專案
- `cargo run` 能執行您的專案
- `cargo test` 能測試您的專案
- `cargo doc` 能為您的專案產生技術文件
- `cargo publish` 能將函式庫發佈到 crates.io
- `cargo clean` 移除專案的 target 資料夾
- `cargo update` 更新專案的所有相依套件

## Module System

Rust 的文件會出現 `project`、`package`、`crate`、`module` 這四種名詞作為模組系統的說明，彼此的關係如下：

- `package` 等同於 `project`，使用 `cargo new` 指令就是建立一個新的 `project`
- 一個 `package` 或 `project` 可以包含多個 `binary crates`，以及 0 個或者 1 個 `library binary`
- 一個 `crate` 可以包含多個 `module`

`library binary` 指的是 `lib.rs`，而 `binary crates` 則是編譯後放在 `bin` 資料夾下的函示庫檔案。

## 學習資源

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust 程式設計語言](https://rust-lang.tw/book-tw/)（為 The Rust Programming Language 的中譯版本）
