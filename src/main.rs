/*use clap::Parser;
use std::fs;
use std::io;
use std::thread;
use std::time::Instant;

// 1. 【实用性 & 零成本抽象】使用 clap 宏自动生成强大的命令行解析器
#[derive(Parser, Debug)]
#[command(author, version, about = "一个极速的并发文件词频统计器")]
struct Args {
    /// 需要统计的文件路径列表
    #[arg(required = true)]
    files: Vec<String>,
}

// 2. 【内存安全】返回 Result 强制处理错误，避免运行时崩溃
fn count_words(path: &str) -> io::Result<usize> {
    // read_to_string 读取文件到内存，返回一个 String 类型（它是这块内存的唯一所有者）
    let content = fs::read_to_string(path)?;

    // 【零成本抽象】迭代器链：看起来像高级语言，编译后和底层 C 循环一样快
    let count = content.split_whitespace().count();

    Ok(count)
} // 函数结束，content 离开作用域，其占用的堆内存被自动、立即释放（无 GC 介入）

fn main() {
    // 解析命令行传入的参数
    let args = Args::parse();

    let start = Instant::now(); // 记录开始时间
    let mut handles = vec![];   // 用于存放线程的句柄

    // 遍历所有文件路径
    for file in args.files {
        // 3. 【无畏并发 & 所有权转移】核心魔法在这里！
        // `move` 关键字将 `file` 变量的所有权，从主线程 *转移* 给了新创建的子线程。
        let handle = thread::spawn(move || {
            // 在子线程中安全地使用 file
            match count_words(&file) {
                Ok(count) => (file, Ok(count)),
                Err(e) => (file, Err(e)),
            }
        });
        handles.push(handle);
    }

    // 收集所有子线程的执行结果
    let mut total_words = 0;
    for handle in handles {
        // 等待线程结束并获取返回值
        let (filename, result) = handle.join().unwrap();
        match result {
            Ok(count) => {
                println!("[{}] 单词数: {}", filename, count);
                total_words += count;
            }
            Err(e) => eprintln!("[{}] 读取失败: {}", filename, e),
        }
    }

    println!("\n总单词数: {}", total_words);
    println!("耗时: {:?}", start.elapsed());
}*/
fn main() {
    /*
    * 第一关 变量与默认的安全感
    * 在很多语言中，变量默认是可以随便改的。但在 rust 中，变量默认是不可变的 Immutable
    */
    // 1、默认不可变的变量
    let name = "Rust";
    // name = "Java";

    // 2、可变变量 必须显式的加上 mut mutable
    let mut age =10;
    println!("最初的年龄： {}", age);
    age =11;
    println!("现在的年龄：{}",age);

    /*
     第二关：Rust 的灵魂 —— 所有权 (Ownership)
    */
    // String是一个复杂类型，它的数据存在 堆 heap 内存上
    let s1 = String::from("你好，世界");
    // 注意这里 把s1 赋值 给 s2
    let s2 = s1;
    // println!("s1 is:{}",s1);
    println!("s2 is {}",s2);

    /*
        第三关：借用（Borrowing） 只看不摸 VS 拿去改
            既然赋值会把所有权交出去（太绝情了），那如果我只是想看一眼数据，不想拿走它呢？
            这就需要用到引用（Reference），在 Rust 中叫做 “借用”。
    */
    let s3 = String::from("你好 ");
    // 1 、 不可变借用 & ：就像把书借给别人看，但是别人不能乱画、乱写
    let len = calculate_length(&s3);

    println!("字符串 '{}' 的长度是 {}", s3, len); // ✅ s3 依然可以使用，因为所有权还在 s3 手里
    // 2 、 可变借用（&mut）：把东西借给别人 修改
    let mut message = String::from("hello");
    change_message(&mut message);
    println!("修改后: {}", message);

    /*
        第四关：枚举与模式匹配 (没有 Null 的世界)
                Rust 没有 null（空值）。为了表示“一个值可能不存在”或者“操作可能失败”，Rust 使用了枚举 (Enum)。
                你刚才代码里的 Result 和这里要讲的 Option 都是枚举。
    */
    // 定义一个枚举，表示一种状态
    enum Coin {
        Yuan(u32), // 携带一个 u32 类型的数据（面值）
        Jiao,
        Fen,
    }
    let my_coin = Coin::Yuan(100);

    // 使用 match 进行模式匹配（极其强大，类似其他语言的 switch，但必须穷举所有情况）
    match my_coin {
        Coin::Yuan(value) => println!("这是 {} 元", value),
        Coin::Jiao => println!("这是一角"),
        Coin::Fen => println!("这是一分"),
    }

    // Rust 处理“可能没有值”的标准做法：Option 枚举
    // Option 的定义大致是：enum Option<T> { Some(T), None }
    let some_number = Some(5); // 有值
    let absent_number: Option<i32> = None; // 没值

    // 你必须用 match 明确处理“有值”和“没值”的情况，编译器不让你逃避！
    match absent_number {
        Some(val) => println!("数字是 {}", val),
        None => println!("这里什么都没有！"),
    }

    // 第一步：创建一个可变的 String 列表 (Vec)
    // let mut：表示这个变量是可以被修改的（比如往里面添加东西）
    // vec![] 是 Rust 提供的一个快捷宏，用来快速创建一个列表
    let mut my_fruits = vec![String::from("橘子"), String::from("苹果"), String::from("橙子")];

    // 打印初始状态。 {:?} 是专门用来打印列表、结构体等复杂数据的占位符
    println!("--- 去超市前 ---");
    println!("篮子里有: {:?}", my_fruits);

    // 第二步：调用我们写的函数
    // 注意这里的 `&mut`：我们把 my_fruits 的“可变借用”交给了 add_fruits 函数。
    // 意思是：“篮子借给你，你可以往里面放东西，但放完记得把篮子还给我。”
    add_fruits(&mut my_fruits);
    // 第三步：在 main 函数里打印最终的列表
    println!("\n--- 从超市回来后 ---");
    println!("篮子里有: {:?}", my_fruits); // ✅ 这里依然可以使用 my_fruits，因为所有权一直都在 main 手里！


    // let mut my_fruits2: Vec<String> = vec![String::from("橘子")];
    //
    // // 1. 我们先拿一个“不可变借用”，相当于我们拿眼睛“盯着”篮子里的第一个水果
    // let first_fruit = &my_fruits2[0];
    //
    // // 2. 然后，我们试图把篮子借给别人去“修改”（添加新水果）
    // add_fruits(&mut my_fruits2);
    //
    // // 3. 最后，我们试图打印一开始盯着的那个水果
    // println!("我一直盯着的水果是: {}", first_fruit);

}
// 这是一个独立的函数。
// 注意参数 `list: &mut Vec<String>`：意思是“借给我一个装 String 的列表，并且允许我修改它”。
fn add_fruits(list: &mut Vec<String>) {
    // 使用 .push() 方法往列表末尾添加新元素
    // String::from() 用来创建一个真正的、分配在堆内存上的字符串
    list.push(String::from("梨"));
    list.push(String::from("香蕉"));

    // 函数执行到这里就结束了。
    // 因为 list 只是“借用”，所以它离开作用域时，什么都不会被销毁，数据依然保留在内存中。
}

// 参数类型是 &mut String，表示借用并且可以修改
fn change_message(s: &mut String) {
    s.push_str(", world!");
}

// 参数类型是 &String 表示只借用 不拿走所有权
fn calculate_length(s: &String) -> usize {
    s.len()
} // s3  离开作用域，但因为他只是借用，所以什么都不会发生