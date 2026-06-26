fn main() {
    // ===== 整数类型 =====
    let a: i32 = 42;
    let b: u32 = 42;
    let c: i64 = 1_000_000;
    let d: usize = 100;

    println!("整数: a={}, b={}, c={}, d={}", a, b, c, d);
    println!("i32 范围: {} 到 {}", i32::MIN, i32::MAX);
    println!("u32 范围: {} 到 {}", u32::MIN, u32::MAX);

    // ===== 浮点类型 =====
    let f1: f32 = 3.14;
    let f2: f64 = std::f64::consts::PI;

    println!("\n浮点: f1={}, f2={}", f1, f2);
    println!("f32 精度: 约 6-7 位有效数字");
    println!("f64 精度: 约 15-16 位有效数字");

    // ===== 布尔类型 =====
    let flag: bool = true;
    let no_flag = false;

    println!("\n布尔: flag={}, no_flag={}", flag, no_flag);

    // ===== 字符类型 =====
    let c1: char = 'A';
    let c2: char = '中';
    let c3: char = '🦀';

    println!("\n字符: c1={}, c2={}, c3={}", c1, c2, c3);
    println!("char 大小: {} 字节", std::mem::size_of::<char>());

    // ===== 类型转换 =====
    let x: i32 = 42;
    let y: f64 = x as f64;
    let z: i32 = y as i32;

    println!("\n类型转换: x={} -> y={} -> z={}", x, y, z);

    // ===== 字面量后缀 =====
    let p = 42u32;
    let q = 3.14f64;
    let r = 1_000i64;

    println!("\n字面量后缀: p={}, q={}, r={}", p, q, r);

    // ===== 溢出处理 =====
    let a: u8 = 255;
    // let c = a.wrapping_add(1);
    let c = a +1;
    let d = a.checked_add(1);

    println!("\n溢出处理: a={}, a.wrapping_add(1)={}, a.checked_add(1)={:?}", a, c, d);

    // ===== 类型大小对比 =====
    println!("\n类型大小对比:");
    println!("bool: {} 字节", std::mem::size_of::<bool>());
    println!("char: {} 字节", std::mem::size_of::<char>());
    println!("i32: {} 字节", std::mem::size_of::<i32>());
    println!("u32: {} 字节", std::mem::size_of::<u32>());
    println!("i64: {} 字节", std::mem::size_of::<i64>());
    println!("usize: {} 字节", std::mem::size_of::<usize>());
    println!("f32: {} 字节", std::mem::size_of::<f32>());
    println!("f64: {} 字节", std::mem::size_of::<f64>());

    let unsigned: u32 = 42;
    let signed: i32 = 42;
    println!("u32 42 的二进制: {:032b}", unsigned);
    println!("i32 42 的二进制: {:032b}", signed);

    let signed: i32 = -1;
    let unsigned: u32 = signed as u32;
    println!("i32 -1  的二进制: {:032b}", signed);
    println!("u32 -1 as u32 的值: {}", unsigned);
    println!("u32 -1 as u32 的二进制: {:032b}", unsigned);


    println!("\nusize == u64? {}", std::mem::size_of::<usize>() == std::mem::size_of::<u64>());
    println!("Vec::len() 返回什么类型: {}", std::any::type_name_of_val(&vec![1, 2, 3].len()));

    let v = vec![1, 2, 3];
    let index: usize = 1;
    println!("v[{}] = {}", index, v[index]);
    // 下面这句注释掉，你可以解开注释看看编译报错：
    // let bad_index: u32 = 1;
    // println!("v[{}] = {}", bad_index, v[bad_index]);

}
