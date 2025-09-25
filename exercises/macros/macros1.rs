// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.



// 定义一个名为 my_macro 的宏
macro_rules! my_macro {
    // 宏没有参数，只有一个分支
    () => {
        // 宏展开时会插入这条打印语句
        println!("Check out my macro!");
    };
}

fn main() {
    // 调用宏时必须加 !，否则会报错
    my_macro!(); // 展开为 println!("Check out my macro!");
}
