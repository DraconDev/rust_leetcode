// pub struct RecentCounter {
//     pings: Vec<i32>,
// }

// impl RecentCounter {
//     pub fn new(pings: Vec<i32>) -> Self {
//         Self { pings }
//     }

//     pub fn ping(&self, t: i32) -> i32 {
//         for i in 0..self.pings.len() {
//             if self.pings[i] > t {
//                 return i as i32;
//             }
//         }
//         0
//     }
// }
use std::collections::VecDeque;

pub struct RecentCounter {
    pings: VecDeque<i32>,
}

impl RecentCounter {
    pub fn new() -> Self {
        Self {
            pings: VecDeque::new(),
        }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        self.pings.push_back(t);
        while *self.pings.front().unwrap() < t - 3000 {
            self.pings.pop_front();
        }
        self.pings.len() as _
    }
}
