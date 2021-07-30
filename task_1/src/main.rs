fn main() {
    let red_light = TrafficLight::Red;
    println!("red light is: {}", red_light.time());
    let green_light = TrafficLight::Green;
    println!("green light is: {}", green_light.time());
    let yellow_light = TrafficLight::Yellow;
    println!("yellow light is: {}", yellow_light.time());
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

pub trait StayTime {
    fn time(&self) -> u8;
}

impl StayTime for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 30,
            TrafficLight::Yellow => 5,
        }
    }
}
