use ethereum_types::H256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
/// Snapshot of the ibft consensus status
pub struct Snapshot<Validator> {
    /// Epoch size of the chain
    pub epoch: u64,
    /// Height of the blockchain
    pub number: u64,
    /// Hash of the snapshot ... or of the block?
    pub hash: H256,
    /// set of validators
    pub validators: Vec<Validator>,
}

