fn main() {
    /*
        第一关：结构体 (Struct) —— 捏造你自己的“积木”
    */
    // 2、实例化结构体
    let mut hero1 = Hero{
        name: String::from("亚瑟"),
        hp:100,
        is_alive:true,
    };
    // 访问和修改字段（需要 hero1 是可变的 mut）
    println!("{} 的初始血量是: {}", hero1.name, hero1.hp);
    hero1.hp = 50; // 挨了一刀
    println!("{} 现在的血量是: {}", hero1.name, hero1.hp);

    println!("===========");

    /*
        第二关：方法 (impl) —— 让积木“动”起来
    */
    let mut arthur = Hero::new(String::from("亚瑟"));

    arthur.show_status();           // 调用只读方法
    arthur.take_damage(30);         // 调用修改方法
    arthur.show_status();
    println!("===========");
    /*
        第三关：特征 (Trait) —— Rust 版的“接口”
            Rust 没有继承（没有子类继承父类）。如果你想要多个不同的结构体拥有相同的行为，你需要使用 trait（特征）。
            假设我们的游戏里不仅有英雄，还有怪物，它们都能“发出声音”或者“被攻击”。
    */

    let mut slime = Monster { name: String::from("史莱姆"), hp: 20 };

    slime.take_damage(10);
    if slime.is_dead() {
        println!("怪物已死亡");
    } else {
        println!("怪物还活着");
    }
}


// 1、定义结构体
struct Hero{
    name: String,
    hp: i32,
    is_alive: bool,
}

impl Hero{
    // 1. 【关联函数】（类似其他语言的构造函数）
    // 注意：它没有 self 参数，通过 Hero::new() 调用
    fn new(name: String) -> Self {
        Self { name, hp: 100 ,is_alive:true} // Self 代表 Hero 本身
    }

    // 2. 【不可变方法】（只读数据）
    // &self 是 hero: &Hero 的简写。意思是：“借用这个英雄看看，但不修改他”
    fn show_status(&self) {
        println!("[{}] 当前血量: {}", self.name, self.hp);
    }

    // 3. 【可变方法】（修改数据）
    // &mut self 是 hero: &mut Hero 的简写。意思是：“借用这个英雄，并且我要修改他”
    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
        println!("[{}] 受到 {} 点伤害！剩余血量: {}", self.name, damage, self.hp);
    }
}

// 1. 定义一个 Trait（特征），就像定下了一份“契约”
trait Attackable {
    fn take_damage(&mut self, damage: i32);

    // Trait 里也可以提供默认实现
    fn is_dead(&self) -> bool;
}

struct Monster{
    name: String,
    hp: i32,
}
impl Attackable for Monster {
    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
        println!("怪物 [{}] 惨叫一声，掉血 {}", self.name, damage);
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0
    }
}