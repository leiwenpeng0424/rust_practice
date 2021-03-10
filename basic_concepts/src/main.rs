mod control_flow;

fn main() {
    // rust声明的变量都是不可修改的，无法经行重新的赋值。
    let a = 12;
    // 编译器会在编译的过程中报出大多数的错误，确保运行时不会出现低级的错误。

    println!("the value of a is {}", a);

    // 默认采取immutable的变量可以坐做到规避一些非常难追查的bug，
    // 如果你想将immutable的变量转换成mutable的话，编译器也会针对你的代码给出确定信息，
    // 避免变量被错误地使用。

    let a = 13;
    println!("the value of a is {}", a);

    // 但是，可变的变量是非常有用的一种特性
    // rust 提供了 mut 关键字来修饰一个可变的变量
    let mut a = 14;

    println!("the value of a is {}", a);

    a = 15;

    println!("the value of a is {}", a);

    // ------- Constants and Variables ---------
    // 常量的声明在 rust 中是无法使用 mut 修饰的。
    // 常量会一直存在于声明时的作用域中，持续整个rust应用的生命周期。
    // 常量适用于一些需要hardcode的地方，并且将常量声明在同一处，方便后面统一修改。
    const MAX_AGE: i8 = 120;

    // -------- Shadowing --------
    // 在同一个作用域中，你可以使用let来声明多个同名的变量，
    // 下面的let声明会隐藏上面的let声明，会创建新的值，但是复用名称。
    // 但是新的值依然是不可变的，这个就是使用 shadowing 和 mut 的关键区别。
    // 另一个区别就是 shadowing 是创建了新的变量，我们可以改变这个新的变量值的类型。
    let x = 1;
    let x = x + 1;
    let x = x * x;
    println!("the value of x is {}", x);

    let x: String = String::from(x.to_string());
    println!("the value of x is {}", x);

    println!("--------------------------------------");

    // ------- 数据类型 --------
    // rust中，每一个变量都会一个指定的类型，这样我们才能知道该如何处理这些数据。
    // rust 是一门，静态类型的语言

    // 下面是两种数据类型的子集，标量(scalar)和复合(compound)
    // rust编译器需要在编译阶段知道所有变量的类型，根据值以及使用方式
    // 编译器通常可以推断出变量的类型。
    // 但是更多的情况下，我们需要自己指定变量的类型。

    // 标量 (scalar)
    // 代表一个单独的值。rust 有四中基本的标量：整型，浮点型，布尔和字符串。

    // 整型
    //  有符号位整型 (负数) i8/i16/i32/i64/i128/isize
    //  无符号位整型 (正数) u8/u16/u32/u64/u128/usize

    //  每一个有符号整型都可以存储一个 -(2^(n-1)) 到 (2^(n-1))-1,
    //  i8可以表示的范围就是 -2^7 到 (2^7) - 1
    // 	u8可以表示的范围就是 0 到 (2^7) - 1

    //  isize和usize指的是根据运行程序的计算机的架构来决定，32位或者是64位。

    // 除了byte之外的所有数字字面量都可以使用类型后缀。
    let _y = 98_222; // 十进制 可以使用短下划线作为分割，方便阅读。
    println!("{}", _y);
    let _y = 0xff; // 十六进制
    let _y = 0o77; // 八进制
    let _y = 0b1111_0000; // 二进制 Binary
    let _y = b'1'; // 单字节字符 Byte

    println!("{}", _y);

    // 整型溢出
    // 如果使用了类型 u8 ，表示的数字范围就是0 - 255，如果你设置了256，
    // 就会造成整型溢出，开发模式下，编译器会抛出 panic 错误，
    // 构建模式下，会执行二进制补码的包装。256 会变成 0，257 会转换成 1。

    // 浮点型
    // 浮点包含两种，f32/f64，默认是f64速度差不多，并且精度更高。
    // 浮点数采用 IEEE-754 标准。f32是单精度，f64是双精度。

    let _z = 16.2;
    let _z = _z + 20.1;
    let _z = _z - 10.22;
    let _z = _z * 0.5;
    let _z = _z / 32.21;
    // let _z = _z % 10;
    println!("{}", _z);

    // 布尔型
    // 布尔型只有两个值，true/false

    let _is_ture = true;
    let _is_false: bool = false;

    // 字符类型
    // Rust的char类型是语言中最原声的字母类型。
    // Char类型的大小是四个字节，
    // Rust 中，汉语，日语等字符，emoji以及零长度的空白字符都是有效的 char 值。
    let _a = 'z';
    let _a = '🦸';
    let _a = '你';

    // 复合类型
    // 复合类型是由多个值组成的类型。
    // 元组(Tuple)和数组(Array)

    // 元组是具有固定长度的，内部可以存储不同类型的值，但是一旦声明制定了长度，就不可改变。
    // 并且内部每一个位置都有一个固定的类型。
    let _b: (i32, i32, i32) = (1, 1, 1);
    // 获取tuple中的值可以采用结构的方法
    let (_a, _d, _c) = _b;
    println!("{} -> {} -> {}", _a, _d, _c);
    // 使用 . 操作符根据位置来取值，位置从 0 开始计算
    let _f = _b.0;
    let _e = _b.1;

    // 数组
    // 数组与元组不同，内部的元素必须要具备相同的数据类型。
    // 相同的是，数组也是声明的时候固定长度，切无法动态地修改。
    let _f = [1, 2, 3, 4];
    // rust 还存在一个 Vector的类型，Vector 就是增强版的 Array，可变长，内部可以存放不同类型的元素。
    // 数组声明的一个技巧, let list: [Type;Count] = [...],
    // 可以用在类型注释中，同样可以用在声明的值， let list = [3; 5];
    let _f: [i32; 5] = [1, 2, 3, 4, 5];
    // 访问数组通过 list[index]的形式

    println!("{}", output_number());
    expression_example();
    let x = expressions_example_2();
    println!("{:?}", x);
    println!("---------------------");
    println!("{}", control_flow::control_flow::produce_num_five());
    control_flow::control_flow::if_else(3);
    control_flow::control_flow::let_if(true);
    // control_flow::control_flow::loop_code();
    control_flow::control_flow::while_code();
    control_flow::control_flow::for_code();
    println!("{:?}", control_flow::control_flow::convert_c_to_h(25.5));
    println!("{:?}", control_flow::control_flow::fibonacci(10));
}

// 函数，通过fn关键字定义，函数名称后跟着圆括号，然后跟着一对大括号，标识着函数体的开始与结尾。
// 定义完成之后，可以在main函数中调用。
fn output_number() -> i32 {
    23
}

// 函数参数，函数参数是一种特殊变量，是函数签名的一部分，
// 当函数拥有参数时，可以为这些参数提供具体的值。
// 多参数使用都好分割
fn output_number_2(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

// 语句与表达式
// Rust是一门基于表达式的语言，
// 在其他的语言中，表达式和语句可能并没有什么区别，
// 但是在Rust中理解表达式(expressions)和语句(statements)的区别是很重要的。
// ---------------------
// 语句，是执行一些操作，但是不返回值的指令。
// 表达式，需要计算并参生一个值。
fn expression_example() {
    let _x = 6; // 语句，只声明了一个变量，不返回变量。
                // let _y = _x = 6; // 该语句再Rust中回报错，但是在C，Ruby，JavaScript中都会成功，_y和_x都是6。

    // 表达式会计算值。

    let x = 5;

    let y = {
        let x = 3;
        x + 1 // 表达式返回 x+1 的值，赋值给 y。
    };

    println!("{:?}", y);
    println!("{:?}", x);
}
// 在Rust中，大部分编写的代码都将会是表达式，
// 上面编写的语句 let _x = 6; 等式右边的六其实就是一个表达式，返回6，并赋值给右边声明的变量 _x。
// 所以，表达式可以是语句的一部分。
// 函数调用是一个表达式，宏调用是一个表达式，我们用于创建新作用域的大括号{}，也是一个表达式。
// 注意上面函数体中的
/*
    let y = {
        let x = 3;
        x + 1 // 如果最后结尾加了分号，那它就不是表达式，而是一个语句。
        // 这与与大多数语言的写法不同。加了分号就表示一个语句，不加就表示一个表达式。
    }
*/

// 具有返回值的函数
fn expressions_example_2() -> i8 {
    5
}
