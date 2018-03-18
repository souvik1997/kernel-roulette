extern crate rand;
use self::rand::Rng;
use spin::Mutex;

extern "C" {
    fn nanosecond_timer_c() -> u64;
}

pub const MIN_RAND: u8 = 0;
pub const MAX_RAND: u8 = 100;

#[derive(Default)]
pub struct RouletteConfig {
    pub chance: u8,
}

impl RouletteConfig {
    pub fn new() -> Self {
        let chance = RNG.lock().gen_range(MIN_RAND, MAX_RAND);
        RouletteConfig { chance: chance }
    }
}

lazy_static! {
    static ref RNG: Mutex<rand::JitterRng> = Mutex::new(rand::JitterRng::new_with_timer(|| unsafe { nanosecond_timer_c() }));
    pub static ref CONFIG: Mutex<RouletteConfig> = Mutex::new(RouletteConfig::new());
}

#[no_mangle]
pub extern "C" fn sample() -> u8 {
    let sampled = RNG.lock().gen_range(MIN_RAND, MAX_RAND);
    if sampled < CONFIG.lock().chance {
        panic!("Boom!");
    } else {
        sampled
    }
}

#[no_mangle]
pub extern "C" fn set_chance(chance: u8) {
    if chance >= MIN_RAND && chance <= MAX_RAND {
        CONFIG.lock().chance = chance
    }
}

#[no_mangle]
pub extern "C" fn get_chance() -> u8 {
    CONFIG.lock().chance
}
