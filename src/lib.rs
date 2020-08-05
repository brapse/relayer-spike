pub type Height = u64;
pub type Hash = ();
pub type ChainID = u64;
pub type Proof = ();

/// Align with: https://github.com/informalsystems/ibc-rs/blob/76b97a56fa68c972c55760f8cf85556b6799c05a/docs/spec/relayer/Relayer.md

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
    // height = signed_header.header.height
    height: Height, // chain_a
    signed_header: Header, // chain_a
}

impl ClientState {
    fn default() -> ClientState {
        return ClientState {
            height: 0,
            signed_header: Header::default(),
        }
    }
}

// look at destination chain
// proove that client state is in a foreign chain at a given height
// This let's me prove that the header can is included in chain_b at proof_height (with proof)
pub struct ProvenClientState {
    // XXX: Probably have a client ID
    header: Header, // height of chain_a
    proof_height: height, // height of chain_b
    proof: Proof, // proof that chain_b includes chain_a client
}

impl ProvenClientState {
    fn default() -> ProvenClientState {
        return ProvenClientState {
            header: Header::default(),
            proof: (),
            // XXX: TODO
        }
    }
}

pub enum ChainError {
    SubmissionError()
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

    // XXX: This will always return target_height_a or ClientError
    pub fn update_client(&mut self, dest: &Chain, target_height_a: Height) -> Result<Height, ClientError> {
        /*
         * What we want to do here is update the client for self on dest to target_height_a
         * where target_height_a is event.height+1
         *
         */
        // what is the schedma of client_state?
        // ClientState {
        //  Height: (from chain A)
        //  SignedHeader: (from chain A)
        // }
        // What is the schema of membership_proof
        // MembershipProof {
        // Height: (from chain B)
        // Proof (from chain B)
        // }
        let (mut client_state, mut membership_proof) = dest.client_state(self.id, target_height_a);

        // This part is super confusing, because it isn't clear what chain the data is pertaining to
        // Prove that chain_b has the SignedHeader of self hain_a at chain_state.height)
        let sh = dest.lc.get_header(membership_proof.height + 1);
        if ! verify_client_state_proof(client_state, membership_proof, sh.app_hash) {
            // Error: Destination chain provided invalid client_state
            return Err(Error::InvalidClientState())
        }

        // verify client_state on self
        if self.lc.get_header(client_state.height) == client_State.signed_header.header {
            return Err(Error::InvalidClientState())
        }

        // update dest client state up to target_height
        while client_state.height < target_height {
            let shared_headers = self.get_minimal_set(client_state.height, target_height);

            // Send update datagram
            dest.submit(create_client_update_datagram(shared_headers));

            let (client_state, membership_proof) = dest.client_state(self.id, target_height_a);
            let sh = dest.get_header(membership_proof.height + 1);
            if ! verify_client_state_proof(client_state, membership_proof, sh.app_hash) {
                // Error: Destination chain provided invalid client_state
                return Err(ClientError::InvalidClientState());
            }

            if self.get_header(client_state.height) == client_state.signed_header.header {
                // Error: Client_state isn't verified by self light client
                return  Err(ClientError::UnverifiedClientState())
            }
        }

        return Ok(target_height_a);
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
