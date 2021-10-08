use ethereum_types::H256;
use serde::{Deserialize, Serialize};

// celo does not provide documentation for the istanbul_ APIs
// check these resources instead
// https://docs.ledgerium.io/dapp-development-guide-1/json-rpc
// https://docs.goquorum.consensys.net/en/20.10.0/Reference/Consensus/IBFT-RPC-API/#istanbul-rpc-api
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
/// Snapshot of the ibft consensus status
pub struct Snapshot<Validator> {
    /// Epoch size of the chain, plain json int
    pub epoch: u64,
    /// Height of the blockchain, plain json int
    pub number: u64,
    /// Hash of the snapshot ... or of the block?
    pub hash: H256,
    /// set of validators
    pub validators: Vec<Validator>,
}

