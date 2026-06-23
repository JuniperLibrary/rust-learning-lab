use clap::Parser;
use std::fs;
use std::thread;
use std::time::Instant;

// 自定义错误类型
#[derive(Debug, thiserror::Error)]
enum WordCounterError {
    #[error("I/O 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("文件是空的: {0}")]
    EmptyFile(String),
}

// 1. 【实用性 & 零成本抽象】使用 clap 宏自动生成强大的命令行解析器
#[derive(Parser, Debug)]
#[command(author, version, about = "一个极速的并发文件词频统计器")]
struct Args {
    /// 需要统计的文件路径列表
    #[arg(required = true)]
    files: Vec<String>,
}

// 2. 【内存安全】返回 Result 强制处理错误，避免运行时崩溃
fn count_words(path: &str) -> Result<usize, WordCounterError> {
    let content = fs::read_to_string(path)?;
    let count = content.split_whitespace().count();
    if count == 0 {
        return Err(WordCounterError::EmptyFile(path.to_string()));
    }
    Ok(count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 解析命令行传入的参数
    let args = Args::parse();

    let start = Instant::now();
    let mut handles = vec![];   // 用于存放线程的句柄

    // 遍历所有文件路径
    for file in args.files {
        let handle = thread::spawn(move || {
            let count = count_words(&file);
            (file, count)   // ← 没有分号
        });
        handles.push(handle);
    }

    // 收集所有子线程的执行结果
    let mut total_words = 0;
    for handle in handles {
        match handle.join() {
            Ok((filename,result)) => {
                match result {
                    Ok(count) => {
                        println!("[{}] 单词数:{}",filename,count);
                        total_words += count;
                    }
                    Err(e) => eprintln!("[{}] 读取失败 {}",filename, e),
                }
            }
            Err(_) => eprintln!("线程崩溃了"),
        }
    }

    println!("\n总单词数: {}", total_words);
    println!("耗时: {:?}", start.elapsed());


    Ok(())
}