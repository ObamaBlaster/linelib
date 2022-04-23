use std::time::Instant;
use std::time::Duration;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::thread;

pub const game_tps : usize = 30;
pub const graphics_tps : usize = 60;
pub const animation_tps : usize = 50;

pub struct Timer{
    pub events: Receiver<()>,
    pub event_loop: Box<dyn Fn() + Send>
}

impl Timer {
    pub fn new(tps : usize) -> Self{
        let (tx, rx) = channel();
        let tps = tps;
        let spt = 1.0 / tps as f32;
        // Nano seconds per tick for the most pecision
        let npt = (spt * 1000000000.0) as usize;
        Timer {
            events: rx,
            event_loop: Box::new(move || {
                loop {
                tx.send(());
                    thread::sleep(Duration::from_nanos(npt as u64));
                }
            })
        }
    }
}
