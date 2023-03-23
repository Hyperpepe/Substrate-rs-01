#[allow(dead_code)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait Duration {
    fn duration(&self) -> u32;
}

impl Duration for TrafficLight {
    fn duration(&self) -> u32 {
        match *self {
            TrafficLight::Red => 60,    // 红灯持续 60 秒
            TrafficLight::Yellow => 5,  // 黄灯持续 5 秒
            TrafficLight::Green => 30,  // 绿灯持续 30 秒
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    println!("Red light duration: {} seconds", red.duration());
    println!("Yellow light duration: {} seconds", yellow.duration());
    println!("Green light duration: {} seconds", green.duration());
}

