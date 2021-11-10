//
//@dev 实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为
//Option<u32>，溢出时返回None，可以上传代码片段，或者代码的链接;
//

fn my_sum(arr: &[u32]) -> Option<u32> {
    let mut num: u32 = 0;
    for element in arr.iter() {
        num += element;
    }
    if (num > 2^32) || (num < 0) {
        return None;
    }
    return Some(num);
}

fn main() {
    println!("Hello, world!");
}