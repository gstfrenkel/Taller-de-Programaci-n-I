use crate::transactions::create_transaction_error::TransactionCreateError;

#[derive(Debug)]
pub enum InterfaceError {
    MissingButton,
    MissingLabel,
    MissingEntry,
    MissingComboBox,
    MissingBox,
    MissingFrame,
    MissingWindow,
    MissingAccount,
    MissingEntryText,
    MissingSpinButton,
    MissingDialog,
    MissingImage,
    InvalidWidgetType,
    LoadCssFile,
    TxCreate,
    LockAccounts,
    LockNode,
    Write,
    Read,
    Send,
    DecodeHex,
    MissingAddress,
    MissingAmount,
    WitnessProgramError,
}

impl From<glib::Error> for InterfaceError {
    fn from(_: glib::Error) -> InterfaceError {
        InterfaceError::LoadCssFile
    }
}

impl From<TransactionCreateError> for InterfaceError {
    fn from(_: TransactionCreateError) -> InterfaceError {
        InterfaceError::WitnessProgramError
    }
}
