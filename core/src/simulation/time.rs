extern crate anymap;

use self::anymap::AnyMap;

pub struct Time {
    pub current_time: f64,
    pub end_time: f64,
    pub tick: f64
}

impl Time {
    pub fn new(configuration: &AnyMap) -> Time {
        let end_time = config!(configuration, TimeEndTime);
        let tick = config!(configuration, TimeTick);

        Time{ current_time: 0.0_f64, end_time: end_time as f64, tick: tick }
    }

    pub fn is_passed(&self) -> bool {
        self.current_time >= self.end_time
    }

    pub fn next_tick(&mut self) {
        self.current_time += self.tick;
    }
}
