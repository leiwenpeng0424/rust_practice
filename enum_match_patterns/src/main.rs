use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    // 枚举与匹配模式。
    // 枚举。使用enum关键字声明
    #[derive(Debug)]
    enum IpAddressKind {
        IPV4,
        IPV6,
    }

    #[derive(Debug)]
    struct IpAddress {
        kind: IpAddressKind,
        address: String,
    }

    let _ip4 = IpAddress {
        kind: IpAddressKind::IPV4,
        address: String::from("127.0.0.1"),
    };

    let _ipv6 = IpAddress {
        kind: IpAddressKind::IPV6,
        address: String::from("::1"),
    };

    // 官方提供了针对两种ip地址标准定义
    // std::net::IpAddr

    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    assert_eq!("127.0.0.1".parse(), Ok(localhost_v4));
    assert_eq!("::1".parse(), Ok(localhost_v6));

    // 一个枚举的🌰
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // ipv6是使用128位二进制来表示。
    // 每4个16进制为1组，一共8组。0000:0000:0000:0000:0000:0000:0000:0000
    // ::1

    // Option枚举与其相对空值的优势
    // Option代表了一种使用场景，即一个值要么有值，要么没值。
    // 从类型系统的角度来看就是，
    // Rust需要检查是否处理了所有应该处理了的值的场景。

    // Rust中没有很多其他语言中都有的空值(Null)概念.
    // 空值是一个值,代表着没有值.

    // 在Rust中,并没有空值,但是确实是有可以编码存在或者不存在的方法.

    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);

    // match 控制流运算符
    // match 是Rust中提供的一种非常强大的控制流运算符,
    // 它允许我们将一个值与一系列的模式相比较,
    // 并根据相匹配的模式执行相应的代码.
    // match会以枚举成员作为模式,逐个匹配,直到进入合适的模式中,执行逻辑.

    #[derive(Debug)]
    enum State {
        Alabama,
        NewYork,
    }

    // 一个Coin枚举
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(State),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        // 一个以枚举成员作为模式的match表达式
        // match coin {
        //     Coin::Penny => {
        //         println!("{:?}", "Lucy Penny");
        //         1
        //     }
        //     Coin::Nickel => 5,
        //     Coin::Dime => 10,
        //     Coin::Quarter(state) => {
        //         println!("{:?}", state);
        //         25
        //     }
        // }
        if let Coin::Quarter(state) = coin {
            println!("{:?}", state);
            25
        } else {
            1
        }
    }

    value_in_cents(Coin::Quarter(State::Alabama));

    // Rust中的match是穷尽的，编译器会保证我们已经处理过了所有的值的可能性，
    // 以此来避免在运行时出现的值为Null的情况。

    // 面对极端的match场景，可能会有特别多个可能匹配的值，
    // Rust提供通配符来统一处理余下的分支。
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("{:?}", "one"),
        3 => println!("{:?}", "three"),
        5 => println!("{:?}", "five"),
        _ => (),
    }

    // if-let 简单控制流
    // if-let 语法提供一种不那么冗余的方式结合if和let来处理只匹配一个模式的值,而忽略其他模式的情况.
}
