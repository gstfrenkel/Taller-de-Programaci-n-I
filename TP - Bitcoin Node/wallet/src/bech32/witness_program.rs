use super::{bech32_constants::*, bech32_errors::WitnessProgramError, bech32mod::Bech32};

/// Represents a Segregated Witness (SegWit) witness program.
///
/// This struct holds information about a witness program, which is used in
/// SegWit transactions to secure data.
#[derive(Debug, Clone)]
pub struct WitnessProgram {
    version: u8,
    program: Vec<u8>, //Hash160 of public key
}

impl WitnessProgram {
    /// Creates a new instance of `WitnessProgram`.
    ///
    /// This function constructs a new `WitnessProgram` instance with the provided witness program data.
    /// The witness program is the hash160 of a public key.
    ///
    /// # Arguments
    ///
    /// * `program` - The witness program data, which typically represents the hash160 of a public key.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the newly created `WitnessProgram` instance if the witness program data is valid,
    /// or an `Err` containing a `WitnessProgramError` if the data is invalid.
    pub fn new(program: Vec<u8>) -> Result<Self, WitnessProgramError> {
        let witness_program = WitnessProgram {
            version: P2WPKH_VERSION,
            program,
        };

        witness_program.validate()?;
        Ok(witness_program)
    }

    /// Creates a new `WitnessProgram` instance from a Bech32-encoded address.
    ///
    /// This function decodes a Bech32-encoded address, extracts the version and witness program data,
    /// and constructs a new `WitnessProgram` instance with the extracted data.
    ///
    /// # Arguments
    ///
    /// * `address` - The Bech32-encoded address to be converted into a `WitnessProgram`.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the newly created `WitnessProgram` instance if the address is valid,
    /// or an `Err` containing a `WitnessProgramError` if the address is invalid or encounters other errors.
    pub fn from_address(address: String) -> Result<WitnessProgram, WitnessProgramError> {
        let b32 = Bech32::from_address(address)?;

        let (version, program) = b32.data().split_at(1);

        let witness_program = WitnessProgram {
            version: version.to_vec()[0],
            program: convert_bits(program.to_vec(), 8)?,
        };

        witness_program.validate()?;
        Ok(witness_program)
    }

    /// Converts a `WitnessProgram` instance into a Bech32-encoded address.
    ///
    /// This function encodes the `WitnessProgram` data, including the witness version and program,
    /// into a Bech32-encoded address with the specified human-readable part (HRP).
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the Bech32-encoded address if the `WitnessProgram` data is valid,
    /// or an `Err` containing a `WitnessProgramError` if the data is invalid or encounters other errors.
    pub fn to_address(&self) -> Result<String, WitnessProgramError> {
        self.validate()?;

        let program = convert_bits(self.program.clone(), 5)?;

        let mut data = vec![self.version];
        data.extend(program);

        let address = Bech32::new(LOWERCASE_HRP.to_string(), data).to_address()?;

        WitnessProgram::from_address(address.clone())?;

        Ok(address)
    }

    pub fn to_pk_script(&self) -> Vec<u8> {
        let mut pk_script = vec![self.version];
        pk_script.push(self.program.len() as u8);
        pk_script.extend(&self.program);

        pk_script
    }

    /// Converts a `WitnessProgram` instance into a Pay-to-Witness-Public-Key-Hash (P2WPKH) script.
    ///
    /// This function constructs a P2WPKH script using the witness version and program data from the `WitnessProgram`.
    /// The P2WPKH script is commonly used in Bitcoin transactions to secure the spending of funds.
    ///
    /// # Returns
    ///
    /// Returns a vector containing the P2WPKH script bytes.
    pub fn validate(&self) -> Result<(), WitnessProgramError> {
        if self.version != P2WPKH_VERSION {
            return Err(WitnessProgramError::InvalidVersion);
        } else if self.program.len() != P2WPKH_HASH_LEN {
            return Err(WitnessProgramError::InvalidLength);
        }

        Ok(())
    }
}

fn get_from(to: u32) -> Result<u32, WitnessProgramError> {
    match to {
        5 => Ok(8),
        8 => Ok(5),
        _ => Err(WitnessProgramError::UnsopportedConversion),
    }
}

/// Converts data from one bit width to another.
///
/// This function performs a conversion of data from a source bit width to a target bit width.
/// The input data is provided as a vector of bytes (`u8`), and the target bit width is specified
/// by the `to` parameter. The function returns the converted data as a `Result<Vec<u8>, WitnessProgramError>`.
///
/// # Arguments
///
/// * `data` - The input data to be converted.
/// * `to` - The target bit width for the conversion.
///
/// # Returns
///
/// Returns a `Result` containing the converted data as a vector of bytes (`u8`) if the conversion is successful,
/// or an `Err` containing a `WitnessProgramError` if the conversion encounters errors.
fn convert_bits(data: Vec<u8>, to: u32) -> Result<Vec<u8>, WitnessProgramError> {
    let from = get_from(to)?;
    let mut bits = 0;
    let mut accumulator = 0;
    let mut result = Vec::new();
    let max_value = (1 << to) - 1;

    for data_value in data {
        let value = data_value as u32;

        if (value >> from) != 0 {
            return Err(WitnessProgramError::InvalidConversion);
        }

        accumulator = (accumulator << from) | value;
        bits += from;

        while bits >= to {
            bits -= to;
            result.push(((accumulator >> bits) & max_value) as u8);
        }
    }

    if from == 8 && to == 5 {
        if bits > 0 {
            result.push(((accumulator << (to - bits)) & max_value) as u8);
        }
    } else if bits >= from || ((accumulator << (to - bits)) & max_value) != 0 {
        return Err(WitnessProgramError::InvalidPadding);
    }

    Ok(result)
}

#[cfg(test)]
mod bech32_test {
    use super::WitnessProgram;
    use crate::bech32::bech32_errors::WitnessProgramError;

    #[test]
    pub fn test_valid_address() {
        assert!(WitnessProgram::from_address(
            "tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc".to_string()
        )
        .is_ok());
        assert!(WitnessProgram::from_address(
            "tb1q79gkmhfaw9szkn8fmg22llkx2sfhlx7ykptww6".to_string()
        )
        .is_ok());
    }

    #[test]
    pub fn test_invalid_address() {
        assert!(
            WitnessProgram::from_address("mtsQWBEUBxTfqRpaaHtRwW6KicGnLCdqzW".to_string()).is_err()
        );
        assert!(WitnessProgram::from_address(
            "tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtcd".to_string()
        )
        .is_err());
        assert!(WitnessProgram::from_address(
            "tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtC".to_string()
        )
        .is_err());
        assert!(WitnessProgram::from_address(
            "tb1qnhm3x5sndagu816yq3jqn4cw38szgvxdydxxtC".to_string()
        )
        .is_err());
        assert!(WitnessProgram::from_address(
            "tb1qnhm3x5sndagu816yq3jqq4cw38szgvxdydxxtC".to_string()
        )
        .is_err());
    }

    #[test]
    pub fn test_address_decoding() -> Result<(), WitnessProgramError> {
        let wp1 =
            WitnessProgram::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc".to_string())?;
        let wp2 =
            WitnessProgram::from_address("tb1q79gkmhfaw9szkn8fmg22llkx2sfhlx7ykptww6".to_string())?;

        let address1 = wp1.to_address()?;
        let address2 = wp2.to_address()?;

        assert_eq!(address1, "tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc");
        assert_eq!(address2, "tb1q79gkmhfaw9szkn8fmg22llkx2sfhlx7ykptww6");

        Ok(())
    }

    #[test]
    pub fn test_pk_script() -> Result<(), WitnessProgramError> {
        let wp1 =
            WitnessProgram::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc".to_string())?;
        let wp2 =
            WitnessProgram::from_address("tb1q79gkmhfaw9szkn8fmg22llkx2sfhlx7ykptww6".to_string())?;

        assert_eq!(
            wp1.to_pk_script(),
            [
                0, 20, 157, 247, 19, 82, 19, 111, 81, 195, 179, 68, 4, 100, 9, 215, 14, 137, 224,
                36, 48, 205
            ]
        );
        assert_eq!(
            wp2.to_pk_script(),
            [
                0, 20, 241, 81, 109, 221, 61, 113, 96, 43, 76, 233, 218, 20, 175, 254, 198, 84, 19,
                127, 155, 196
            ]
        );

        Ok(())
    }
}
