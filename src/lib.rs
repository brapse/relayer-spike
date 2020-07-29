pub type Height = u64;

pub struct Chain {
}

// TODO: make this real
pub type Subscription = Vec<Event>;

pub enum Event {
    NoOp(),
}

pub enum Datagram {
    NoOp(),
}

impl Chain {
   pub  fn new() -> Chain {
        return Chain {}
    }

    pub fn subscribe(&self) -> Subscription {
        return vec![Event::NoOp()]
    }

    // maybe return some kind of error
    pub fn submit(&mut self, datagrams: Vec<Datagram>) {
        // XXX: Update client
    }

    pub fn pending_datagrams(&self, other: &Chain, event: &Event) -> Vec<Datagram> {
        // XXX: perform queries
        return vec![Datagram::NoOp()]
    }
}
