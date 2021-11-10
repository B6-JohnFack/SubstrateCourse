//
//@dev 为枚举交通信号灯实现一个trait, trait里包含一个返回时间的方法，不同的灯持续的时间不同，可以上
//传代码片段，或者代码的链接
//

pub trait Time {
    fn get_time(&self);
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl Time for TrafficLight {
    fn get_time(&self) {
        match *self {
            TrafficLight::Red => println!("Light color: Red, time: 60s"),
            TrafficLight::Green => println!("Light color: Green, time: 30s"),
            TrafficLight::Yellow => println!("Light color: Red, time: 2s"),
        }
    }
}