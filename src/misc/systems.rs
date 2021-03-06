use std::fmt::Debug;
use std::marker::PhantomData;

use serde::Serialize;
use serde_json::Value;
use specs::prelude::*;

use crate::{
    misc::components::*,
    networking::ClientSender,
};



#[derive(Default)]
pub struct SendSys<T: Serialize + Component + Debug> {
    mutation: String,
    phantom: PhantomData<T>,
}

impl<T: Serialize + Component + Debug>  SendSys<T> {
    pub fn new<S: Into<String>>(mutation: S) -> Self {
        SendSys {
            mutation: mutation.into(),
            phantom: PhantomData,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MutationMsg<T: Serialize + Debug> {
    pub mutation: String,
    pub inner: T
}

impl<'a, T: Serialize + Component + Debug> System<'a> for SendSys<T> {
    type SystemData = (ReadStorage<'a, T>,
                       ReadExpect<'a, ClientSender>);

    fn run(&mut self, (t, out): Self::SystemData) {
        use specs::Join;

        let elts = t.join().collect::<Vec<_>>();

        let msg = MutationMsg {
            mutation: self.mutation.clone(),
            inner: elts
        };



        out.send(&msg)

    }
}


