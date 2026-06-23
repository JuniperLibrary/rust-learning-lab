fn main() {

    /*
        第一关：泛型 (Generics) —— 制造“万能模具”
            假设你要写一个函数，用来比较两个数字的大小，返回较大的那个。
            如果没有泛型，你得写三个函数：max_i32（比整数）、max_f64（比小数）、max_i64（比长整数）……这就太累了。

           泛型（用 <T> 表示）让你写一次代码，适用于所有类型。
    */
    // 2. 用整数列表测试
    let number_list = vec![34, 50, 25, 100, 65];
    // 编译器会自动推断出这里的 T 是 i32
    let result = get_largest(&number_list);
    println!("最大的数字是: {}", result);

    // 3. 用字符列表测试
    let char_list = vec!['y', 'm', 'a', 'q'];
    // 编译器会自动推断出这里的 T 是 char (字符也支持比较大小)
    let result = get_largest(&char_list);
    println!("最大的字符是: {}", result);

    /*
        第二关：迭代器与闭包 (Iterators & Closures) —— 优雅的“流水线”
            这是 Rust 中最迷人、最常用的语法，体现了 Rust 函数式编程的一面。
            假设有一个需求：“把一个数字列表里的偶数挑出来，每个乘以 2，然后求总和。”
            传统的写法是用 for 循环：
    */
    let numbers = vec![34, 50, 25, 100, 65];
    let mut sum = 0;
    for n in &numbers {
        if n % 2 == 0 {
            sum += n;
        }
    }

    /*
        这在 Rust 里能跑，但不够“Rust”。Rust 程序员更喜欢用迭代器链，一行代码搞定：
    */
    let numbers2 = vec![1, 2, 3, 4, 5, 6];

    // 魔法开始了！
    // 1. iter()：把列表变成迭代器（流水线）
    // 2. filter(|&x| ...)：闭包。|x| 是参数，x % 2 == 0 是返回值。只保留偶数。
    // 3. map(|x| ...)：闭包。把挑出来的偶数乘以 2。
    // 4. sum()：把流水线上的结果加起来。
    let sum: i32 = numbers2.iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 2)
        .sum();

    println!("最终的总和是: {}", sum); // 输出应该是 24 (2*2 + 4*2 + 6*2)

    /*
        什么是闭包 Closure?
            注意 |x| x % 2 这个奇怪的语法。
            他是一个匿名函数 类似于java的lambda，或 JS的箭头函数 (x) => x * 2
            竖线 || 里面是参数，后面是代码体。极其简洁，专门配合迭代器使用
    */

}

// 1. 定义一个“万能”函数
// <T> 声明了一个泛型类型 T（就像代数里的 x）。
// T: PartialOrd 叫做“特征约束”（Trait Bound）。
// 意思是：“T 可以是任何类型，但它必须实现了 PartialOrd（支持大小比较）这个特征！”
fn get_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}