//! `Istanbul` namespace
use crate::{
    api::Namespace,
    helpers::{self, CallFuture},
    types::{BlockId, Snapshot},
    Transport,
};

/// `Istanbul` namespace
#[derive(Debug, Clone)]
pub struct Istanbul<T> {
    transport: T,
}

impl<T: Transport> Namespace<T> for Istanbul<T> {
    fn new(transport: T) -> Self
    where
        Self: Sized,
    {
        Istanbul { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T: Transport> Istanbul<T> {
    /// Get Istanbul consensus snapshot
    pub fn snapshot<Validator>(&self, block: BlockId) -> CallFuture<Option<Snapshot<Validator>>, T::Out> {
        let include_txs = helpers::serialize(&false);

        let result = match block {
            BlockId::Hash(hash) => {
                let hash = helpers::serialize(&hash);
                self.transport.execute("istanbul_getSnapshotAtHash", vec![hash])
            }
            BlockId::Number(num) => {
                let num = helpers::serialize(&num);
                self.transport.execute("istanbul_getSnapshot", vec![num, include_txs])
            }
        };

        CallFuture::new(result)
    }
}
