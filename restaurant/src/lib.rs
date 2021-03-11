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

// 声明 back_of_house 模块，其内容将位于 src/back_of_house.rs
mod back_of_house;
mod front_of_house;

use crate::back_of_house::{Appetizer, Breakfirst};

fn serve_order() {}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();

    let _order1 = Appetizer::Soup;
    let _order2 = Appetizer::Salad;

    let mut meal = Breakfirst::summer("Rye");
    meal.toast = String::from("Wheat");

    // println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");

    // **使用use关键字将名称引入作用域**
    /*
        再作用域中增加use和路径类似于在文件系统中增加软连接(symbolic link).
        同样，use关键字也会进行私有性的检查，同其他路径一样.

        使用crate关键字开头，从crate根模块
    */
    // use crate::front_of_house::hosting;
    // hosting::add_to_waitlist();

    /*
        你也可以使用相对路径来引入一个项到作用域
    */
    // use front_of_house::serving;
    // serving::take_order();

    /*
        为了避免当前作用域内出现本地作用域变量、方法名称与引入项名称相同的困惑出现。
        我们一般习惯于引入函数或者方法的父模块，这样做的好处也方便我们对方法的定义经行溯源。
    */

    /*
        使用 as 来对引用项起别名。
        // use std::fmt;
        // use std::io;
        // fn fn1() -> fmt::Result {
        //     // code
        // }
        // fn fn2<T>() -> io::Result<T> {
        //     // code
        // }

        use std::fmt::Result as FmtResult;
        use std::io::Result as IoResult;
        fn function1() -> FmtResult {
            // --snip--
        }
        fn function2() -> IoResult<()> {
            // --snip--
        }
    */

    // 以上两种方法都是惯用的避免引入项名称冲突的方法。遇到时，选其一方式解决即可。

    // **使用 pub use 重导出名称**
    // **使用外部包**
    /*
        在前面的猜数字游戏里面，我们使用了一个外部的crate包，rand，用于产生一个随机的数字
        在Cargo.toml文件的[dependencies]下定义外部crate -> rand，后面跟上版本号。
        这样Cargo在执行构建指令的时候，就会自动去crates.io网站上下载crate包。

        [dependencies]
        rand = "0.5.5"

        引用外部的crate，也同样使用use关键字。

        use rand::Rng;

        fn main() {
            let secret_number = rand::thread_rng().gen_range(1, 101);
        }

        除了外部的crate之外，Rust也提供官方的标准库std::*，同样使用use来引入。
        标准库以std开头。
        use std::colleactions::HashMap;
        let map = HashMap::new();

        **使用嵌套路径来消除大量的use行**
        想从一个命名空间导入多个项，可以使用嵌套路径，而避免使用多个use。

        use std::cmp::Ordering;
        use std::io;

        嵌套引入，只需要保留路径的相同部分，双冒号之后用大括号来引入不同的路径。
        use std::{cmp::Ordering, io};

        在较大的工程中使用嵌套路径的话，对于代码的简洁性时有提升的。

        use std::io;
        use std::io::Write;

        嵌套引入，使用self来表示主路径了。
        use std::io::{self, Write};

        **通过glob运算符将所有的公开定义引入作用域**
        use std::collections::*;
        上面的语句会将std::collections中定义的所有公有项引入作用域。
        通常用在测试模块中。
    */
}
