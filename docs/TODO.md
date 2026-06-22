# 🦀 Rust 学习路线图

## ✅ 已完成 — 基础已掌握

- [x] 变量绑定与可变性（`let` / `let mut` / Shadowing）
- [x] 基础数据结构（`String`、`Vec<T>`）
- [x] 表达式：Rust 中"一切皆表达式"（if/else、代码块、match）
- [x] 格式化输出（`println!`、`{:?}`、格式化语法）
- [x] 所有权（Ownership）：规则、移动语义、Copy vs Clone、Drop/RAII
- [x] 借用与引用（Borrowing）：`&T` vs `&mut T`、借用规则、NLL、切片
- [x] 结构体与方法（Struct & impl）：三种结构体、derive、Builder、Newtype
- [x] 枚举与模式匹配（Enum & Match）：Option、Result、解构、if let、while let
- [x] 特征（Trait）：定义、实现、`impl` vs `dyn`、super trait、关联类型
- [x] 泛型（Generics）：类型参数、Trait 约束、where 子句
- [x] 迭代器与闭包（Iterator & Closure）：闭包捕获、链式调用、消费器

---

## 🟡 下一步学习计划

### 阶段一：补齐"地基"的最后一块拼图（控制流与错误处理）

| 知识点 | 难度 | 预估时间 | 说明 |
|--------|------|----------|------|
| 枚举与模式匹配加强 | ★ | 2h | 加强 Result / Option 组合子、matches! 宏 |
| 错误处理（Error Handling） | ★★ | 4h | Rust 没有 try/catch，用 `Result<T, E>` |
| `?` 操作符 | ★★ | 1h | 优雅传播错误，理解 `From<E>` 约束 |
| 自定义错误类型 | ★★★ | 3h | `thiserror` / `Display` 实现 |

```rust
// 目标效果：写出这样的错误处理
fn read_file(path: &str) -> Result<String, MyError> {
    let content = std::fs::read_to_string(path)?;  // ? 自动转换错误类型
    Ok(content)
}
```

### 阶段二：驯服"借用检查器"（进阶内存管理）

| 知识点 | 难度 | 预估时间 | 说明 |
|--------|------|----------|------|
| 生命周期（Lifetimes） | ★★★ | 6h | Rust 最难概念之一，核心是"引用关系"而非"存活时间" |
| `Box<T>` | ★★ | 2h | 堆分配，递归类型，trait 对象 |
| `Rc<T>` / `Weak<T>` | ★★ | 2h | 引用计数，允许多个所有者 |
| `RefCell<T>` | ★★★ | 3h | 内部可变性，运行时借用检查 |
| `Arc<T>` + `Mutex<T>` | ★★★ | 3h | 线程安全的引用计数和互斥锁 |

```rust
// 目标效果：理解生命周期标注的含义
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### 阶段三：构建大型项目（工程化与模块化）

| 知识点 | 难度 | 预估时间 | 说明 |
|--------|------|----------|------|
| 模块系统（Module System） | ★★ | 3h | `mod`、`use`、`pub`、路径、文件组织 |
| Cargo 进阶 | ★ | 1h | `Cargo.toml` 详解、依赖管理 |
| Workspace（工作空间） | ★★ | 2h | 多包项目管理 |
| 测试（Testing） | ★★ | 3h | `#[test]`、单元测试、集成测试、文档测试 |
| 文档（Documentation） | ★ | 1h | 文档注释 `///`、`cargo doc` |

### 阶段四：释放 Rust 的终极威力（并发与异步）

| 知识点 | 难度 | 预估时间 | 说明 |
|--------|------|----------|------|
| `std::thread` | ★★ | 2h | 线程创建、`join`、`move` 闭包 |
| 消息传递（Channels `mpsc`） | ★★★ | 3h | 多生产者单消费者，类似 Go 的 channel |
| 共享状态（`Mutex<T>` + `Arc<T>`） | ★★★ | 3h | 互斥锁、死锁预防 |
| `Send` / `Sync` | ★★★ | 1h | 理解这两个 marker trait 的含义 |
| async / await 入门 | ★★★ | 4h | `Future` trait、`.await`、异步运行时（Tokio） |
| Tokio 实战 | ★★★ | 6h | 异步 TCP/HTTP、任务调度、select! |

```rust
// 目标效果：理解无畏并发的核心概念
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));
```

### 阶段五：Rust 的"黑魔法"（高阶特性）

| 知识点 | 难度 | 预估时间 | 说明 |
|--------|------|----------|------|
| 宏（Macros） | ★★★ | 5h | `macro_rules!` 声明宏、重复模式 |
| 过程宏（Procedural Macros） | ★★★ | 4h | 自定义 `#[derive(...)]`、属性宏、函数宏 |
| unsafe Rust | ★★★ | 3h | 裸指针、FFI、何时需要 unsafe |
| FFI（外部函数接口） | ★★★ | 3h | 调用 C 库、`#[no_mangle]`、`extern` |
| 常用 crate 深入 | ★★ | 4h | serde、tokio、clap、rayon |

## 🚀 实战项目 Ideas

| 项目 | 所需阶段 | 可学知识点 |
|------|---------|-----------|
| RPN 计算器 | 阶段一 ~ 二 | 枚举、match、Vec 栈操作 |
| 命令行猜数字 | 阶段一 | 输入输出、随机数、循环 |
| **文件词频统计器**（已有脚手架） | 阶段一 ~ 三 | 文件 IO、HashMap、迭代器、clap |
| 简易 HTTP 服务器 | 阶段四 | tokio、TCP、HTTP 协议 |
| Todo CLI 应用 | 阶段二 ~ 三 | clap、文件持久化、错误处理、测试 |
| Mini Grep 搜索工具 | 阶段三 ~ 四 | 迭代器、并发、正则 |

## 🧠 学习建议

1. **每天写代码**：哪怕 20 分钟也比一次学 5 小时效果好
2. **先跑通再深究**：看不懂生命周期没关系，先让 borrow checker 通过再说
3. **善用编译器**：Rust 编译器的错误信息是最好的老师——仔细读！
4. **把"为什么编译不通过"当游戏**：Rust 编译器像在跟你玩 puzzle
5. **遇到不懂的 → 看标准库源码**：`Vec`、`Option`、`Result` 的源码是顶级教学材料
6. **每学完一章 → 写一个对应的小程序**：不要只看书
7. **阶段一到阶段三的顺序不要乱跳**（基础不牢地动山摇）
8. **阶段四和阶段五可以穿插学**：但至少先完成阶段一和二

---

> 进度更新：2026-06-22 创建 | 按 Qwen AI 协助整理的 5 阶段路线图重构
