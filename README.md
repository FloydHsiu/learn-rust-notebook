# learn-rust-notebook

![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)
![Learning Progress](https://img.shields.io/badge/LeetCode-1%2F2000-blue)

My journey of learning Rust via LeetCode Quest, using AI to transform functional solutions into high-performance, idiomatic code.

## 核心理念

1. **Draft:** 先用自己的直覺寫出第一版（可能很像 C++ 或 Python）。
2. **Refine:** 提供 Prompt 給 LLM（如 Gemini），請它改寫成更 Rustacean 的版本。
3. **Learn:** 紀錄改寫後的差異（如：`match` 的運用、所有權的優化、或是 `Iterator` 的使用），及演算法的盲點與差異。

## 目錄結構

```text
learn-rust-notebook/
├── src/
│   ├── lib.rs
│   └── q0000_sample/
│       ├── mod.rs 
│       ├── solution.rs      # 最終程式碼
│       ├── draft.rs         # 初始版本
│       └── notes.md         # LLM 的建議與心得
├── prompt/
│   └── improve-draft.prompt.md
└── README.md
```

## 進度追蹤表

| # | Title | Difficulty | Key Rust Concept |
| --- | --- | --- | --- |
| 0000 | sample | Easy | cargo test, #\[cfg(test)\] |

## Prompt

在詢問 LLM 時，有些請求或描述會重複使用，可以記錄在其中，也可以隨著練習的過程，優化提供給 LLM 的 Prompt。

## 📚 Learning Resources

* [The Rust Programming Language](https://doc.rust-lang.org/book/) (The Book)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
* [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises to get used to reading and writing Rust code.
