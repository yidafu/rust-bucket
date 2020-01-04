fn math<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

fn fn_as_return_value() -> impl Fn(i32) -> i32 {
    let i = 2;
    // 函数返回，引用跟着返回，变量 i 将被销毁
    // 通过 move 将 i 的所有权移到闭包内
    move |j| j * i
}

fn main() {
    let out = 233;
    fn add(i: i32, j: i32) -> i32 {
        i + j
    }
    let closure_add = |i, j| i + j + out;

    assert_eq!(250, closure_add(10, 7));
    assert_eq!(17, add(10, 7));

    let a = 2;
    let b = 3;
    assert_eq!(math(|| a + b), 5);

    assert_eq!(fn_as_return_value()(5), 10);
}
