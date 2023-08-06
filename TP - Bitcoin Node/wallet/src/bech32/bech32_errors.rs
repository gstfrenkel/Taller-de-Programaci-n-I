
#[derive(Debug)]
pub enum Bech32Error{
    InvalidHRP,
    InvalidLength,
    MissingSeparator,
    InvalidData,
    InvalidChecksum,
    InvalidCase,
}

#[derive(Debug)]
pub enum WitnessProgramError{
    InvalidAddress,
    InvalidInput,
    InvalidPadding,
    InvalidVersion,
    InvalidLength,
    UnsopportedConversion,
}

impl From<Bech32Error> for WitnessProgramError {
    fn from(_: Bech32Error) -> Self {
        WitnessProgramError::InvalidAddress
    }
}
