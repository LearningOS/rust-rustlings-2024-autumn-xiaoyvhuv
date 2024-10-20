// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


// 定义模块并在其中定义宏，直接使用 pub 关键字
// 使用 #[macro_export] 属性来使宏可以在模块外部使用
#[macro_export]
#[macro_use]  //对于模块 要加这个
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    // 直接调用宏，不需要模块路径
    my_macro!();
}