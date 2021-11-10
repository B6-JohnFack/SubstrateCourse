//
//@dev 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方
//形，需要用到泛型和泛型约東，可以上传代码片段，或者代码的链接。
//
//以矩形（包括正方形）为例
//
fn calculate_area<T: PartialOrd + Copy> (x: u32, y: u32) -> u32 {
    let mut my_area: u32 = 0;
    my_area = x * y;
    my_area
}