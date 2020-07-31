use relayer_spike::{Chain, Datagram, Header};

/*
 * TODO:
 * + Error handling
 * + Spawn multiple paths in a Route abstraction
 */

fn main() {
    // Relay from chain a to chain b
    let mut chain_a = Chain::new();
    let mut chain_b = Chain::new();
    let mut subscription =  chain_a.subscribe();

    for event in subscription.iter() {
        let target_height = 1;
        chain_b.update_client(&chain_a, target_height);

        let header = chain_a.get_header(target_height);
        
        let datagrams = chain_a.pending_datagrams(&chain_b, event);

        verify_proof(&datagrams, &header);

        chain_b.submit(datagrams);
    }
}

fn verify_proof(_datagrams: &Vec<Datagram>, _header: &Header) {
}
