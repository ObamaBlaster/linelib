use serde::{Deserialize, Serialize};

// Force states to be serializable
// This is so we can implmenet roll back networking
// and replays

// Uhh, note that states just containers for values.
// They can be mutated however, and don't need to be recreated
// for performance reasons i guess ? (really for rust compiler reaosns)
pub trait State {
    fn init(&mut self);
    fn tick(&mut self);
    fn to_beeps_and_boops(&self) -> Vec<u8>;
    fn state_name(&self) -> &'static str;
}

// This Macro creates a bunch of structs representing
// States in the game
#[macro_export]
macro_rules! state_tree {
    ($($i:ident => [$($ref:ident),*]),*,) => {
        // State Nodes wrap the states for the game
        pub enum StateNode {
            $(
                $i($i),
            )*
        }

        // this trait implemtnation enforces the state tree's transition policy
            $(
                $(
                    impl<'a> Into<$ref> for $i{
                        fn into(self) -> $ref {
                            self.$ref()
                         }
                    }
                )*
            )*



        // $(
        //     pub trait concat_idents!(To_,$i) {
        //     }
        // )*

        // $(
            // impl $i {
            //     $(
            //         pub fn (concat_idents!(to_, $i))(&mut self) -> $ref {

            //         }
            //     )*
        //     }
        // )*
    };
}
