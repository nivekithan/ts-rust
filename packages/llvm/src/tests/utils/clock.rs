pub struct Clock {
    time: Vec<usize>,
    limiter: Vec<usize>,

    finished: bool,
}

impl Clock {
    pub fn new(limiter: Vec<usize>) -> Self {
        let time: Vec<usize> = limiter.iter().map(|_| return 0).collect();

        return Clock {
            time: time,
            limiter,
            finished: false,
        };
    }

    pub fn increase(&mut self) {
        let pos = self.time.len() - 1;

        self.increase_pos(pos);
    }

    pub fn get_cur_time(&self) -> Option<&Vec<usize>> {
        if self.finished {
            return None;
        }

        return Some(&self.time);
    }

    fn increase_pos(&mut self, pos: usize) {
        if pos >= self.time.len() {
            panic!("Pos cannot be equal or greater than len of time vector");
        }

        let cur_time_hand = self.time[pos];
        let limit_time_hand = self.limiter[pos];

        if cur_time_hand >= limit_time_hand - 1 {
            self.time[pos] = 0;

            if pos == 0 {
                self.finished = true;
                return;
            }

            return self.increase_pos(pos - 1);
        } else {
            self.time[pos] += 1;

            self.finished = false;
        }
    }
}
