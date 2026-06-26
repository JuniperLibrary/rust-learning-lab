# AGENTS.md — rust-learning-lab

Personal Rust learning playground. Not production code.

## Project structure

```
Cargo.toml          # single crate, edition = "2024", default-run = "s0-data-types"
src/
  main.rs           # fallback binary — just prints available --bin options
  bin/
    阶段零/           # Stage 0: Foundation (basic data types, control flow, etc.)
      01-基础数据类型.rs  # cargo run --bin s0-data-types
      02-复合类型.rs      # (pending)
      03-控制流.rs        # (pending)
      04-字符串深入.rs    # (pending)
      05-类型转换.rs      # (pending)
      06-常用集合.rs      # (pending)
    阶段一/           # Stage 1: Error handling & advanced control flow (pending)
    hero.rs         # struct + trait examples —— cargo run --bin hero
    advanced.rs     # generics + iterators + closures —— cargo run --bin advanced
docs/               # learning notes in Chinese (9 completed topics)
test*.txt           # sample text files for the commented-out word counter
```

Binary names use English identifiers (e.g. `s0-data-types`) for easy CLI entry, while file paths are in Chinese for readability.

## Key commands

```bash
cargo run                     # run default binary (currently s0-data-types)
cargo run --bin s0-data-types # Stage 0: 基础数据类型
cargo run --bin hero          # struct + trait examples
cargo run --bin advanced      # generics + iterators + closures
cargo check                   # quick compile check (faster than build)
```

## Important facts

- **Edition 2024** — requires Rust ≥ 1.85. Current toolchain: 1.91.0.
- **No tests.** No `#[test]` or `#[cfg(test)]` anywhere in the repo.
- **No CI, no Makefile, no formatter/linter config.** The project uses defaults.
- **Dead code warnings are expected** — commented-out code blocks are intentional teaching examples. `cargo check` will emit `#[warn(dead_code)]`.
- **All source files are heavily commented in Chinese** — the .rs files ARE the learning material. Do not strip comments.
- **No workspace** — single package despite git history mentioning workspace renames.
- **Runtime deps:** `clap 4.5` (derive) + `thiserror 2.x`.

## What this repo is for

Progressive Rust learning: variables → ownership → borrowing → enum/match → struct → trait → generics → iterators/closures. Each concept has runnable code + a matching doc note in `docs/01-基础/`.

## Binary naming convention

Binary names use English identifiers for easy CLI entry:
- `s0-data-types` → `src/bin/阶段零/01-基础数据类型.rs`
- `s0-` prefix = Stage 0 (阶段零)
- `s1-` prefix = Stage 1 (阶段一), etc.

## 用户协作规范（首次对话请先阅读此处）

### 沟通风格
- 全程**中文**交流
- 语言简洁直接，不要客套/寒暄/表扬
- 不要一次性给出全部信息，要**步步引导**

### 教学模式（核心 — 不遵守会被纠正）
1. **先动手，后讲原理** — 先让用户运行现有代码、观察现象，再解释 Why
2. **一次只教一个概念** — 不超前引入未学过的知识点
3. **不要直接给答案** — 用提问引导用户自己发现模式
4. **每一步都要用户动手写/改代码**，不能只是阅读
5. 学习新的 crate（如 `thiserror`）时，用 `cargo add` 添加依赖，让用户感受 Rust 的工具链

### 文档输出与提交规范（每次必做）
1. **每一课都要输出**：
   - `docs/` 下的对应笔记文件（如 `docs/01-基础/09-错误处理.md`）
   - 文件格式遵循已有笔记风格：是什么 → 怎么用 → 为什么 → 常见坑
   - 学习过程回顾总结（放在笔记末尾或单独说明）
2. 用 `todowrite` 管理学习进度，一课一个 todo，完成后立即标记
3. **每一课结束后必须提交代码**：用 `git commit` 提交当课的所有改动，commit message 格式为 `docs: 第X课 - <课程主题>`

### 学习路径编排
- 严格按照 TODO.md 的六阶段路线图推进（阶段零 → 阶段五）
- 阶段零 → 阶段一 → 阶段二 → 阶段三（不跳顺序）
- 语⾔基础未学完前（阶段零未完成前），不进入后续进阶内容
