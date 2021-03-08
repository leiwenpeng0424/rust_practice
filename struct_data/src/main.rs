mod calc_square_area;

// Struct，或者Structure，是一个自定义数据类型，允许你命名和包装多个相关的值，从而形成一个有意义的组合。
// struct类似于元组，可以拥有不同数据类型的成员，但是struct需要为每个成员提供一个访问的名字
fn main() {
    println!("Hello, world!");

    // struct
    #[derive(Debug)]
    struct User {
        name: String,
        gender: String,
        sign_in_count: u64,
        active: bool,
    }

    // 定义了 struct 之后我们就可以使用它来创建结构体的实例.
    let mut me = User {
        name: String::from("ray"),
        gender: String::from("men"),
        sign_in_count: 1,
        active: false,
    };

    // 想要修改实例的值, 整个实例必须是可变的.
    me.name = String::from("lei wenpeng");

    println!("{:#?}", me);

    // 使用结构体更新语法.从其他结构体生成新的实例.
    let you = User {
        name: String::from("roy"),
        gender: String::from("men"),
        ..me
    };

    println!("{:?}", you);

    // 使用没有命名字段的元组结构来创建不同的类型.
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)] // 通过添加trait注释来处理届固体的格式化输出
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _white = Color(255, 255, 255);
    let _center_point = Point(0, 0, 0);
    // 以 struct 关键字开头, 后跟结构体名, 以及元组中的类型.
    // 虽然Color和Point都表示拥有三个成员,且成员都是i32类型的值.
    // 但是Rust编译器会认为这是两个不同的类型

    // 计算矩形的面积
    let rect = calc_square_area::Rectangle {
        width: 15,
        height: 15,
    };

    let area = rect.area();
    // calc_square_area::square_area(&rect);
    println!("{:#?}", area);
    println!(
        "{:#?}",
        rect.is_contain(&calc_square_area::Rectangle {
            width: 12,
            height: 16
        })
    );
    println!("{:#?}", rect.is_square());
    let square = calc_square_area::Rectangle::square(12);
    println!("{:#?}", square.area());
}
