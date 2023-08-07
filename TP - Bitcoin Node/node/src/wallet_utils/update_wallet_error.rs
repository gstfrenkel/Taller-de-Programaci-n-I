#[derive(Debug)]
pub enum UpdateWalletError {
    Read,
    Write,
    SendProof,
    BroadcastTx,
    LockMempool,
    GetTxn,
}
