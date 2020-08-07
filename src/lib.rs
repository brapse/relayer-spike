/// Align with: https://github.com/informalsystems/ibc-rs/blob/76b97a56fa68c972c55760f8cf85556b6799c05a/docs/spec/relayer/Relayer.md
pub type Height = u64;
pub type Hash = ();
pub type ChainID = u64;
pub type Proof = ();
pub type Subscription = Vec<Event>;

#[derive(std::cmp::PartialEq)]
pub struct Header {
    height: Height,
    hash: Hash,
    app_hash: Hash,
}


pub struct MembershipProof {
    height: Height,
}

impl Header {
    fn default() -> Header {
        return Header {
            height: 1,
            hash: (),
            app_hash: (),
        }
    }
}

pub struct FullNode {
    pub fn subscribe(&self) -> Subscription {
        return vec![Event::NoOp()]
    }

    // XXX: Error handling
    pub fn submit(&self, datagrams: Vec<Datagram>) {
    }

    // This method is named way because it's possible for a relayer to query
    // a source chain for packets which have the source chain has commited to sending but hasn't yet sent
    pub fn pending_datagrams(&self, other: &Chain, event: &Event) -> Vec<Datagram> {
        return vec![Datagram::NoOp()]
    }
}

impl FullNode {
    fn consensus_state(chain_id: ChainID, target_height: Height) -> (ConsensusState, MembershipProof) {
        // In practice this will query the client_state, get the height and perform a second query
        // for the consensus_state. it's possible that the client.state.height < target_height in which case this function will return the highest possible height

        return (ConsensusState::default(), MembershipProof{height})
    }
}

pub struct LightClient {
    pub fn get_header(&self, height: Height) -> Header {
        return SignedHeader::default()
    }

    pub fn get_minimal_set(&self, from: Height, to: Height) -> Vec<SignedHeader> {
        return vec![SignedHeader::default()]
    }
}

pub enum ChainError {
    VerificationError(),
    HeaderMismatch(),
}
pub struct Chain {
    chain_id: ChainID,
    full_node: FullNode,
    light_client: LightClient,
}


struct ConsesusState {
    height: Height, // Is this superflous?
    signed_header: SignedHeader,
}

struct SignedHeader {
    header: Header,
    commit: (),
}

impl SignedHeader {
    fn default() -> SignedHeader {
        return SignedHeader {
            header:  Header::default(),
            commit: (),
        }
    }
}

pub enum Event {
    NoOp(),
}

pub enum Datagram {
    NoOp(),
}

impl Chain {
   pub fn new() -> Chain {
        return Chain { 
            id: 0,
            full_node: FullNode {},
            light_client: LightCLient {},
        }
    }

    // XXX: This will always return target_height_a or ClientError
    pub fn update_client(&mut src, dest: &Chain, src_target_height: Height) -> Result<Height, ClientError> {
        let (mut src_consensus_state, mut dest_membership_proof) = dest.full_node.consensus_state(src.id, src_target_height);

        let dest_sh = dest.light_client.get_header(dest_membership_proof.height + 1);
        // type verifyMembership = (root: CommitmentRoot, proof: CommitmentProof, path: CommitmentPath, value: Value) => boolean (ICS-023)
        if ! verify_consensus_state_inclusion(src_consensus_state, dest_membership_proof, dest_sh.app_hash) {
            // Error: Destination chain provided invalid consensus_state
            return Err(ChainError::VerificationFailed())
        }

        // verify client_state on self
        if src.lc.get_header(src_consensus_state.height) == src_consensus_state.signed_header.header {
            return Err(ChainError::HeaderMismatch())
        }

        // XXX: Is there a chance of multiple iterations of this loop?
        while src_consensus_state.height < src_target_height {
            let src_signed_headers = src.light_client.get_minimal_set(src_consensus_state.height, src_target_height);

            dest.full_node.submit(create_client_update_datagram(src_signed_headers));

            let (src_consensus_state, dest_membership_proof) = dest.full_node.consensus_state(src.id, src_target_height);
            let dest_sh = dest.light_client.get_header(dest_membership_proof.height + 1);
            if ! verify_consensus_state_inclusion(src_consensus_state, dest_membership_proof, dest_sh.app_hash) {
                // Error: Destination chain provided invalid client_state
                return Err(ChainError::VerificationError())
            }

            if src.light_client.get_header(src_consensus_state.height) == consensus_state.signed_header.header {
                // Error: consesus_state isn't verified by self light client
                return  Err(ChainError::HeaderMismatch())
            }
        }

        return Ok(target_height_a)
    }
}

fn verify_consensus_state_inclusion(_consensus_state: &ConsensusState, _membership_proof: &MembershipProof, _hash: &Hash) -> bool {
    return true
}

fn create_client_update_datagram(_header: Vec<SignedHeader>) -> Datagram  {
    return Datagram::NoOp()
}
