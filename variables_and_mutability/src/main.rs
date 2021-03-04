fn main() {
	// rust声明的变量都是不可修改的，无法经行重新的赋值。
	let a = 12;
	// 编译器会在编译的过程中报出大多数的错误，确保运行时不会出现低级的错误。

	let a = 13;
	let mut a = 14;

	// 默认采取immutable的变量可以坐做到规避一些非常难追查的bug，
	// 如果你想将immutable的变量转换成mutable的话，编译器也会针对你的代码给出确定信息，
	// 避免变量被错误地使用。

    println!("Hello, world!");
}
