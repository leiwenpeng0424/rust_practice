/*
    定义一个模块以mod开始，指定模块名称，最后用花括号来包裹模块的主体。
    模块内我们也可以定义其他的模块，以同样的格式定义。
    模块内可以保存一些定义的其他项，比如结构体、枚举、常量、特性或者函数。

    前面提到了，src/lib.rs和src/main.rs被叫做crate根。之所以这样叫，它们是因为
    这两个文件的内容都分别在crate模块结构的根组成了一个名为crate的模块，该结构称为
    模块树。

    crate
    |----front_of_house
        |----hosting
        |   |----add_to_waitlist
        |   |----seat_at_table
        |----serving
            |----take_order
            |----server_order
            |----take_payment

    这样一种表现形式类似于文件系统中的目录。
*/

/*
    **路径用于引用模块树中的项**
    我们使用路径的形式来找到crate模块树中的某一个项，
    有两种表示路径的方式
        - 绝对路径 从crate根开始，以crate名或者字面值crate开头。
        - 相对路径 从当前模块开始，以self或者super或当前模块的标识符开头
    绝对路径与相对路径都采用双冒号(::)分割标识符。
*/

/*
    **以super为开头的，从父模块开始的相对路径**
    这么做类似于文件系统中以..开头的语法。
*/

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
        fn fix_incorrect_order() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();
}
