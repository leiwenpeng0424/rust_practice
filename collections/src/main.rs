// Vector, String, HashMap
fn main() {
    println!("Hello, world!");
    // 创建一个空的Vector.
    // 创建一个空的vector来存储i32类型的值.
    let _v: Vec<i32> = Vec::new();
    // Rust提供了vec!宏来创建带有初始值的Vector
    let mut v = vec![1, 2, 3];

    v.push(4);

    println!("{:?}", v);

    // 同struct类似，vector 一旦离开作用域，就会被销毁，包括里面的内容。
    // 但是，当我们开始使用的时候，情况就会有些复杂。考虑下面这种情况。
    let mut _y: i32 = 0;
    let _x = &v[1];

    v.push(5);

    match v.get(5) {
        // 使用get来访问一个vector中的成员。
        Some(item) => {
            _y = *item;
        }
        None => {
            println!("There is no third element");
            _y = 0;
        }
    }

    println!("{:?}", _y);

    // 遍历vector中的元素
    // for...in
    for item in &v {
        println!("{:?}", item);
    }
    // 通过遍历可变vector，修改所有成员的值。
    for item in &mut v {
        // 通过*号，解引用item，来获取到item真实的值。
        *item *= 2;
    }

    println!("{:?}", v);

    // 使用枚举来在vector中存储多种类型。
    // 如果需要在vector中储存不同类型值时，可以通过定义一个枚举来实现。
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(0.32),
        SpreadsheetCell::Text(String::from("Hello World")),
    ];

    println!("{:?}", row);

    let first = &row[0];
    println!("{:?}", first);

    // 字符串
    // Rust和核心语言中，只有一种字符串类型`str`，字符串slice。
    // 称为String的类型是由rust官方标准库提供的，没有写进核心语言部分，它是可曾长、可变、有所有权的utf-8编码的字符串类型。
    // 当我们讨论String时，应该同时包括String和&str。两者都在被广泛使用，都是使用utf-8编码格式。

    // 创建
    let mut s = String::new();
    s.push_str("Hello");
    s.push_str("World");

    let s1 = "Hello World".to_string();
    let s2 = String::from(" From Lei Wen Peng");

    // 更新
    // 加号拼接。拼接完成之后，s1将会失效。
    // 此处 + 号虽然接受的是一个 &str 作为加号右边的参数, 但是我们这边是使用的 &String, 也可以操作,
    // 是因为这里会对 &String 做一个强制转换.
    let s3 = s1 + &s2;
    // 解引用强制多态
    println!("{:?}", s3);

    // 使用format!宏来输出String。
    // 与println!宏拥有类似的参数。
    let s4 = format!("{}, {}", s2, s3);
    println!("{:?}", s4);

    // 字符串无法被索引，为什么？
    // 在rust内部，String是一个Vec<u8>的封装。索引并不能确保能拿到指定下标的内容，
    // vec的成员可能是由两个字节表示一个书面文字，索引是有可能只拿到某一个文字
    // 表示的第一个字节。

    // 字面量长度似乎是12，但是实际Rust给出的长度是24.
    println!("{:?}", String::from("Здравствуйте").len());

    // 字符串 slice
    // 索引字符串通常不是一个好点子，因为字符串索引应该返回的类型是不明确的：字节值、字符、字型族或者字符串 slice。

    // HashMap
    use std::collections::HashMap;

    let mut hash: HashMap<u8, String> = HashMap::new();
    let my_name = String::from("Lei Wen Peng");

    hash.insert(9, my_name);

    // 类似于Vector，所有的成员都是同质的，所有的key必须是同一个类型，所有的value必须也是同一个类型
    // hash.insert("name".to_string(), "12");
    println!("{:?}", hash);

    let teams = vec![String::from("blue"), String::from("red")];
    let scores = vec![10, 20];

    let mut scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();

    /*
        对于实现了Copy traits的数据类型，例如i32，值可以直接拷贝进HashMap。
        而对于String这类拥有ownership的数据类型来说，将值移动到HashMap之后，
        HashMap会接管这类数据的所有权。
    */
    let s = String::from("mime");
    scores.insert(&s, &12);
    println!("{:?}", scores);
    println!("{:?}", s);
}
