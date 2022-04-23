pub struct Animation<T> {
    start_tick: usize,
    tick_duration: usize,
    pub running: bool,
    pub reversed: bool,
    pub init: T,
}

impl<T> TickingAnimation<T> for Animation<T> {
    fn tick_anim(&mut self, curr_tick: usize) -> T {
        todo!()
    }
}

pub trait TickingAnimation<T> {
    fn tick_anim(&mut self, curr_tick: usize) -> T;
}

// This macro creates a struct to map local variables to a field in AnimationManager
macro_rules! animation_table {
    ($($name:expr => $type:ident),*,) => {
       pub struct AnimationStorage{
        $(
            pub $name: Animation<$type>,
        )*
       }
    };
}

pub struct AnimationManager {}
