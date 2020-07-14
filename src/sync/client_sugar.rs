use crate::sync::client::Client;

impl Client {
    pub fn must_barrier<S: ToString>(&self, state: S, target: u64) {
        self.barrier(state, target).expect("must barrier")
    }
}
