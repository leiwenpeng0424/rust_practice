// rust的模块化
// --------------
// Packages,       cargo的一个功能，允许你构建、测试和分享crate。
// Crates,         一个模块的树形结构，它形成了库或二进制项目。
// Modules和Use,   允许你控制作用域和路径的私有性。
// Path，          一个命名例如结构体、函数或者模块等项的方式。

// 通过 cargo new project-name 可以创建一个 rust 项目。
/*
    src
    |--main.rs
    target
    Cargo.lock
    Cargo.toml
*/
/*
    查看Cargo.toml文件，其中并没有提到任何关于src/main.rs的地方，但是cargo run依然可以找到
    该文件，作为程序的入口文件来运行。

    是因为，cargo遵循一个约定，src/main.rs就是一个与包同名的二进制crate的crate根。
    同样的，cargo也知道如果包内包含src/lib.rs，则包带有其同名的库crate，且src/lib.rs是crate的根。
    crate的根文件将由Cargo传递给rustc来实际构建库1，或者二进制项目。

    如果项目中同时有src/main.rs和src/lib.rs，则它有两个crate，一个库，一个二进制项，且名字都与
    包相同。通过将文件放置在src/bin目录下面，一个包可以拥有多个二进制crate：src/bin下的文件都会编译成
    独立的二进制crate。

    一个crate会将一个作用域内的相关功能分组到一起，使得该功能可以很方便地在多个项目之间共享。
*/

/*
	**定义模块来控制作用域和私有性**
*/

fn main() {
    println!("Hello, world!");
}
