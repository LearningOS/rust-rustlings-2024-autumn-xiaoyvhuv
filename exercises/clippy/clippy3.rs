// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



// #[allow(unused_variables, unused_assignments)]
// fn main() {
//     let my_option: Option<()> = None;
//     if my_option.is_none() {
//         my_option.unwrap();
//     }

//     let my_arr = &[
//         -1, -2, -3
//         -4, -5, -6
//     ];
//     println!("My array! Here it is: {:?}", my_arr);

//     let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
//     println!("This Vec is empty, see? {:?}", my_empty_vec);

//     let mut value_a = 45;
//     let mut value_b = 66;
//     // Let's swap these two!
//     value_a = value_b;
//     value_b = value_a;
//     println!("value a: {}; value b: {}", value_a, value_b);
// }

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 不要在这里调用 unwrap，因为 my_option 已经是 None
        // 这将导致程序 panic
        // my_option.unwrap();
    }

    let my_arr = &[-1, -2, -3, -4, -5, -6]; // 修正数组定义格式
    println!("My array! Here it is: {:?}", my_arr);

    // 更直接地创建一个空向量
    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 使用 Rust 标准库中的 mem::swap 来交换两个变量
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
