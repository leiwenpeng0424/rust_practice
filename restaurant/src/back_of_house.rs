// 模块内部定义的结构体
#[derive(Debug)]
pub struct Breakfirst {
    pub toast: String,    // 公共字段
    season_fruit: String, // 私有字段
}

// 只要是用pub修饰了的枚举结构，那么结构内所有的成员都会是公有的，作用域外部可以访问的。
// 值的注意的是，枚举成员默认就是共有的，因为私有的成员的作用并不大。
#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}

impl Breakfirst {
    // 用于构建结构体实例的方法。
    // 因为Breakfirst拥有一个私有字段，无法在外部作用域中访问，自能通过结构体内部的方法来构建。
    // 所以如果没有summer方法，Breakfirst将无法在外部作用域中构建实例。
    pub fn summer(toast: &str) -> Breakfirst {
        Breakfirst {
            toast: String::from(toast),
            season_fruit: String::from("peaches"),
        }
    }
}

fn fix_incorrect_order() {
    cook_order();
    // 使用super构建从父模块开始的相对路径。
    super::serve_order();
}

fn cook_order() {}
