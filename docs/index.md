# Rust 学习实验室

> 从零开始，动手学 Rust。每个知识点都配有**可运行代码 + 详细笔记**。

## 📚 学习进度

| 模块 | 状态 | 练习代码 | 笔记 |
|------|------|----------|------|
| 变量与可变性 | ✅ 已完成 | `src/main.rs` | [📖 笔记](01-基础/01-变量与可变性.md) |
| 所有权 (Ownership) | ✅ 已完成 | `src/main.rs` | [📖 笔记](01-基础/02-所有权.md) |
| 借用与引用 (Borrowing) | ✅ 已完成 | `src/main.rs` | [📖 笔记](01-基础/03-借用与引用.md) |
| 枚举与模式匹配 (Enum & Match) | ✅ 已完成 | `src/main.rs` | [📖 笔记](01-基础/04-枚举与模式匹配.md) |
| 结构体 (Struct) | ✅ 已完成 | `src/bin/hero.rs` | [📖 笔记](01-基础/05-结构体与方法.md) |
| 方法 (impl) | ✅ 已完成 | `src/bin/hero.rs` | [📖 笔记](01-基础/05-结构体与方法.md) |
| 特征 (Trait) | ✅ 已完成 | `src/bin/hero.rs` | [📖 笔记](01-基础/06-特征.md) |
| 向量 (Vec) | ✅ 已完成 | `src/main.rs` | [📖 笔记](01-基础/05-结构体与方法.md#元组结构体) |

## 📁 目录结构

```
docs/
├── 01-基础/     # 已学基础知识点（每篇都包含了"是什么→怎么用→为什么→常见坑"）
├── 02-进阶/     # 待学进阶知识点
├── 03-实战/     # 实战项目笔记
├── TODO.md      # 详细学习路线图（带优先级/难度）
└── index.md     # 本文件
```

## 🎯 学习原则

- **动手 > 看书**：每个知识点都有对应的可运行代码
- **渐进式**：基础 → 进阶 → 实战，不要跳过
- **用自己的话总结**：笔记不是抄文档，是理解后的提炼
- **优先理解"为什么"**：Rust 的设计决策背后都有深刻的工程考量

## 🔗 推荐学习资源

| 资源 | 说明 |
|------|------|
| [**Rust 程序设计语言**（The Book）](https://doc.rust-lang.org/book/) | 官方入门书，必读 |
| [**Rust By Example**](https://doc.rust-lang.org/stable/rust-by-example/) | 边看边练，配合本仓库很合适 |
| [**Rustlings**](https://github.com/rust-lang/rustlings) | 命令行交互式练习 |
| [**Rustonomicon**](https://doc.rust-lang.org/nomicon/) | 学习 unsafe 后必看 |
| [**Rust 学习之路** (github.com)](https://github.com/ctjhoa/rust-learning) | 精选资源合集 |
