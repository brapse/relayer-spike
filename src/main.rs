use relayer_spike::Chain;

fn main() {
    // Relay from chain a to chain b
    let mut chain_a = Chain::new();
    let mut chain_b = Chain::new();
    let mut subscription =  chain_a.subscribe();

    // XXX: This case be spawned in multiple threads support relaying multiple chains in multiple
    // directions in the same process space
    for event in subscription.iter() {
        // What do we do here if this needs to be mutable?
        let pending_packets = chain_a.pending_datagrams(&chain_b, event);

        // XXX: Validation

        chain_b.submit(pending_packets); // or this might include updates?
    }
}
