// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 不要在 is_none 时 unwrap，会 panic，clippy 会警告
    // if my_option.is_none() {
    //     my_option.unwrap();
    // }

    // 数组元素要用逗号分隔
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // resize 返回 ()，不能直接赋值。应先创建 Vec，再 resize。
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    // 用 clear() 清空 Vec，符合 clippy 建议
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 用 std::mem::swap 交换变量值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
