pub type Height = u64;
pub type Hash = ();
pub type ChainID = u64;
pub type Proof = ();

#[derive(std::cmp::PartialEq)]
pub struct Header {
    height: Height,
    hash: Hash,
    app_hash: Hash,
}

impl Header {
    fn default() -> Header {
        return Header{
            height: 1,
            hash: (),
            app_hash: (),
        }
    }
}

pub struct ClientState {
    height: Height,
    signed_header: Header,
}

impl ClientState {
    fn default() -> ClientState {
        return ClientState {
            height: 0,
            signed_header: Header::default(),
        }
    }
}

pub struct ProvenClientState {
    // XXX: Probably have a client ID
    header: Header ,
    proof: Proof,
    app_hash: Hash,
}

impl ProvenClientState {
    fn default() -> ProvenClientState {
        return ProvenClientState {
            header: Header::default(),
            proof: (),
            app_hash: (),
        }
    }
}

pub struct Chain {
    id: ChainID,
}

type MembershipProof = ();

struct ConsesusState {
    height: Height,
}

// TODO: make this real
pub type Subscription = Vec<Event>;

pub enum Event {
    NoOp(),
}

pub enum Datagram {
    NoOp(),
}

// TODO: Connection construction
// TODO: Channel construction
impl Chain {
   pub  fn new() -> Chain {
        return Chain { id: 0 }
    }

    pub fn proven_client_state(&self, _chain_id: ChainID, height: Height) -> ProvenClientState {
        return ProvenClientState::default();
    }

    pub fn subscribe(&self) -> Subscription {
        return vec![Event::NoOp()]
    }

    // maybe return some kind of error
    pub fn submit(&self, datagrams: Vec<Datagram>) {
        // XXX: Update client
    }

    pub fn pending_datagrams(&self, other: &Chain, event: &Event) -> Vec<Datagram> {
        // XXX: perform queries
        return vec![Datagram::NoOp()]
    }

    pub fn get_header(&self, height: Height) -> Header {
        return Header::default()
    }

    pub fn get_minimal_set(&self, from: Height, to: Height) -> Vec<Header> {
        return vec![Header::default()]
    }

    // Update the client of a foreign chain on this chain
    pub fn update_client(&mut self, dest: &Chain, target_height: Height) {
        // Check if targetHeight exists already on destination chain.
        // Query state of IBC client for `src` on chain `dest`.
        // XXX: Probably don't need to pass self but instead the chain ID
        // - So here the we need the origin state and proofs to assert it's presence on the
        // - destination state
        // So this is actually client state
        let mut proven_client_state = dest.proven_client_state(self.id, target_height);

        // Verify if installed header is equal to the header obtained the from the local client 
        // at the same height
        if !(self.get_header(target_height) == proven_client_state.header) {
            return // ;{nil, error}
        }
                
        // Verify the result of the query
        // - Then we verify destination has a valid next header
        let signed_header = dest.get_header(proven_client_state.header.height + 1);
        if !verify_client_state_proof(&proven_client_state, &signed_header.app_hash) {
            return // {nil, error}
        }

        // This code doesn't make any sense unless the dest.get_proven_client_state can return
        // target_height or the latest. 
        while proven_client_state.header.height < target_height {
            // Installed height is smaller than the target height.
            // Do an update to IBC client for `src` on `dest`.
            let signed_headers = self.get_minimal_set(proven_client_state.header.height, target_height);
            // Might fail due to conncurent client updates.
            dest.submit(vec![create_client_update_datagram(signed_headers)]);

            // Check if targetHeight exists already on destination chain.
            // Query state of IBC client for `src` on chain `dest`.
            // XXX: I think client_consensus_state needs an ID
            proven_client_state = dest.proven_client_state(self.id, target_height);
                
            // Verify if installed header is equal to the header obtained the from the local client 
            // at the same height
            // - maybe just compare the hashes here
            if !(self.get_header(proven_client_state.header.height) == proven_client_state.header) {
                return // {nil, error}
            }
                        
            // Verify the result of the query
            let sh = dest.get_header(proven_client_state.header.height + 1);
            if ! verify_client_state_proof(&proven_client_state, &sh.app_hash) {
                return // {nil, error}
            }
        }

        return // {clientState.Height, nil}

    }
}

pub fn verify_client_state_proof(_proven_client_state: &ProvenClientState, _hash: &Hash) -> bool {
    return true
}

// I think we need some error types here
pub fn verify_proofs(_datagrams: Vec<Datagram>, _header: Header) {
}

pub fn create_client_update_datagram(_header: Vec<Header>) -> Datagram  {
    return Datagram::NoOp()
}
