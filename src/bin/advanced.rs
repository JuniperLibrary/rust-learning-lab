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