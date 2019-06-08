use std::thread::sleep;
use core::borrow::Borrow;

// 把红绿灯看成一个状态机，状态转换过程如下：
//+-----------+      +------------+      +---------+
//|   Green   +----->+   Yellow   +----->+   Red   |
//+-----+-----+      +------------+      +----+----+
//      ^                                     |
//      |                                     |
//      +-------------------------------------+

#[derive(Debug)]
enum TrafficLightState {
    Red { waiting_time: std::time::Duration },
    Yellow { waiting_time: std::time::Duration },
    Green { waiting_time: std::time::Duration },
}

struct TrafficLight {
    state: TrafficLightState,
}

fn change_light(state: &TrafficLightState) {
    match state {
        TrafficLightState::Green { waiting_time } => sleep(*waiting_time),
        TrafficLightState::Red { waiting_time } => sleep(*waiting_time),
        TrafficLightState::Yellow { waiting_time } => sleep(*waiting_time)
    }
}

fn main() {
    let mut state_machine = TrafficLight{
        state: TrafficLightState::Green { waiting_time: std::time::Duration::new(60, 0) }
    };
    println!("{:?}", state_machine.state);
    change_light(&state_machine.state);
    state_machine.state = TrafficLightState::Yellow { waiting_time: std::time::Duration::new(10, 0) };
    println!("{:?}", state_machine.state);
    change_light(&state_machine.state);
    state_machine.state = TrafficLightState::Red { waiting_time: std::time::Duration::new(60, 0) };
    println!("{:?}", state_machine.state);
    change_light(&state_machine.state);
    state_machine.state = TrafficLightState::Green { waiting_time: std::time::Duration::new(60, 0) };
    println!("{:?}", state_machine.state);
    change_light(&state_machine.state);
}
