use sdl2::event::Event;

use super::input_queue::InputQueue;

pub trait BufferLogic<M> where M : EventMap{
    fn get_ticked_events(&self) -> M;
    fn buffer(&mut self, event: Event);
    fn queue(&self) -> InputQueue;
    fn begin(&mut self);
    fn end(&mut self);
    fn new() -> Self;
    fn can_handle(&self) -> bool;
}

pub trait EventMap {
    fn new() -> Self;
}

