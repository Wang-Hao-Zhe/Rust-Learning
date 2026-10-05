# Rust Learning

跟着 [The Rust Programming Language](https://doc.rust-lang.org/book/)（"the book"）学习 Rust 的代码记录。

## 🛠️ 环境(我的学习环境)

- Rust：`rustc 1.99.0` / `cargo 1.99.0`
- 操作系统: Fedora Linux 44 (KDE Plasma Desktop Edition) x86_64
- 编辑器：VS Code + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- 字体：Sarasa Mono Slab SC（开启连字）

## 🗂️ 项目结构

这是一个 **VS Code 多根工作区**，工作区文件位于 `.vscode/` 下：
```text
learning/
├── .doc/ # 文档脚本
├── .vscode/
│   └── learning.code-workspace
├── chapter1-hello_rust/ # 第一章
├── chapter2-gussing_game/ # 第二章
├── chapter3-concepts/ # 第三章
└── chaptern-xxxxxxxxxx/ # 后续章节
```

每个章节都是**独立的 Cargo 项目**，可以单独 `cargo run`；同时由外层 Git 仓库统一管理。

## 🚀 常用命令

```bash
# 进入某一章(以第二章为例)
cd chapter2-gussing_game

# 运行当前章节
cargo run

# 运行某个 bin（如果有 src/bin/ 下的程序）
cargo run --bin draw

# 编译但不运行
cargo build

# 清理编译产物
cargo clean
```


## 📦 chapter2 的 input 小库

`chapter2-gussing_game` 里有一个自建的 `src/lib.rs` + `src/input.rs`，封装了最基础的读行,在项目中用法如下：

```rust
use chapter2_gussing_game::input::input;

fn main() {
    let s = input();            // 返回去除了换行的 String
    let n: i32 = s.parse().unwrap();
}
```

## 📄 License

[MIT](LICENSE) © 2026 Alex-Wang
