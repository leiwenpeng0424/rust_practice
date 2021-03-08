pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    // 结构中定义的方法可以接受额外的参数
    pub fn is_contain(&self, rect: &Rectangle) -> bool {
        (self.width >= rect.width) && (self.height >= rect.height)
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    // 在impl块中，还有另一个有用的功能，允许在impl中声明不以self作为参数的方法，
    // 称为关联函数。
    // String::from(); from就是一个关联函数
    // 关联函数 使用`::`来调用。`::`操作符主要用来获取结构体中的关联函数或者命名空间中的成员。
    pub fn square(s: u32) -> Rectangle {
        Rectangle {
            width: s,
            height: s,
        }
    }
}
// 每个结构体允许拥有多个实现块。
impl Rectangle {
    // add code here
}

// 借用结构体实例，而不是获取它的所有权。
// pub fn square_area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }
