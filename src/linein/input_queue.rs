use std::sync::mpsc::{Receiver, Sender, channel};
use sdl2::event::Event;

unsafe impl Send for InputQueue {}
unsafe impl Sync for InputQueue {}
pub struct InputQueue {
    pub outbuf : Receiver<Event>,
    pub inbuf : Sender<Event>,
}

impl InputQueue {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        InputQueue{
            outbuf: rx,
            inbuf: tx,
        }
    }

    pub fn dequeue(&mut self) -> Option<Event>{
        let mut r= self.outbuf.try_recv();
        match r {
            Ok(e) => Some(e),
            Err(_) => None,
        }
    }

    pub fn drain(&mut self) -> Vec<Event>{
        let mut r= self.outbuf.try_iter().collect::<Vec<Event>>();
        r
    }

    pub fn queue(&mut self, e : Event){
        self.inbuf.send(e);
    }
}