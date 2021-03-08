pub mod control_flow {
    // add code here

    pub fn produce_num_five() -> i32 {
        5
    }

    // if 表达式
    // if 允许根据条件执行不同的分支代码
    pub fn if_else(num: i32) {
        // 这里的判断条件必须是一个bool值，
        // 返回true或者是false的表达式。
        // 这里就要比JavaScript要严谨一点。
        if num < 3 {
            println!("{:?}", "小于三");
        } else if num == 3 {
            println!("{:?}", "等于三");
        } else {
            println!("{:?}", "大于三");
        }
        // if的条件查找，只会执行第一个匹配到的语句块，后面所有剩下的都不会执行。
        // 甚至都不会进行进行的检查。
        // if else 语句并不适合进行多个分支的判断，如果使用 else if 超过了两个，
        // 那么可能你需要对你的代码进行重构了。
        // 后面会介绍一种更强大的Rust分支结构(branching construct)，叫做 match
    }

    pub fn let_if(condition: bool) {
        let num = if condition { 5 } else { 6 };
        println!("{:?}", num);
    }

    pub fn loop_code() {
        // Rust中提供三种循环
        // loop，while，for
        // 使用loop重复执行代码

        let mut num = 1;

        loop {
            num = num + 1;
            println!("{:?}", num);

            if num == 10000 {
                break;
            }
        }

        // loop 表达式可以返回某个值，当循环满足某个条件
        let res = loop {
            num -= 1;
            if num == 1 {
                break num;
            }
        };

        println!("--> {:?}", res);
    }

    pub fn while_code() {
        let mut num = 5;
        while num > 1 {
            println!("{:?}", num);

            num -= 1;
        }
    }

    pub fn for_code() {
        let a = [10, 20, 30, 40, 50];
        for item in a.iter() {
            println!("{:?}", item);
        }

        let b = 0..4;

        for item in b {
            println!("{:?}", item);
        }
    }

    pub fn convert_c_to_h(c: f32) -> f32 {
        (c * 1.8) + 32.0
    }

    pub fn fibonacci(num: i32) -> i32 {
        if num == 0 || num == 1 {
            num
        } else {
            fibonacci(num - 2) + fibonacci(num - 1)
        }
    }
}
