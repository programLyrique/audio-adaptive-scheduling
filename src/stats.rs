use time::{PreciseTime};

#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub mean : f64,
    pub var : f64,//We may use it later, but we don't compute it so far
    n : u64,
}

/// Compute an online mean: mean_{n+1} = f(mean_n, x)
/// TODO: make it also possible to use an exponential moving average
impl Stats {
    pub fn new() -> Stats {
        Stats {mean : 0., var : 0., n : 0}
    }

    pub fn init(m : f64) -> Stats {
        Stats {mean : m, var : 0., n : 1}
    }
    //Better to make it generic on Num types?
    //TODO: rather calculate a moving average
    #[inline(always)]
    pub fn update(&mut self, x : f64) -> f64 {
        self.n += 1;
        let delta = x - self.mean;
        self.mean += delta / self.n as f64;
        self.mean
    }

    #[inline(always)]
    pub fn update_time(&mut self, prev_time : PreciseTime) -> f64 {
        let time_now = PreciseTime::now();
        let duration = prev_time.to(time_now).num_microseconds().unwrap();
        self.update(duration as f64)
    }


}
