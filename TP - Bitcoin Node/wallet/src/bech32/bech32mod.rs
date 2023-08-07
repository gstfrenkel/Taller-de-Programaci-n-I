use super::{bech32_constants::*, bech32_errors::Bech32Error};

/// Represents a Bech32-encoded data structure.
///
/// This struct holds information about a Bech32-encoded data, which is commonly used in Bitcoin
/// and other cryptographic applications. Bech32 is a human-readable format for encoding data using
/// a base32 representation with error-detection codes.
#[derive(Debug, Clone)]
pub struct Bech32 {
    hrp: String,
    data: Vec<u8>,
}

impl Bech32 {
    /// Creates a new instance of `Bech32`.
    ///
    /// This function constructs a new `Bech32` instance with the provided human-readable part (HRP) and binary data.
    /// Bech32 is a human-readable format commonly used in Bitcoin and other cryptographic applications for encoding data
    /// using a base32 representation with error-detection codes.
    ///
    /// # Arguments
    ///
    /// * `hrp` - The human-readable part (HRP) of the Bech32-encoded data.
    /// * `data` - The binary data to be encoded using the Bech32 format.
    ///
    /// # Returns
    ///
    /// Returns a new `Bech32` instance containing the provided HRP and binary data.
    pub fn new(hrp: String, data: Vec<u8>) -> Self {
        Bech32 { hrp, data }
    }

    /// Creates a new `Bech32` instance from a Bech32-encoded address.
    ///
    /// This function decodes a Bech32-encoded address, verifies its checksum, and constructs a new `Bech32` instance
    /// with the extracted human-readable part (HRP) and data bytes. Bech32 is a human-readable format commonly used
    /// in Bitcoin and other cryptographic applications for encoding data using a base32 representation with error-detection codes.
    ///
    /// # Arguments
    ///
    /// * `string` - The Bech32-encoded address to be converted into a `Bech32` instance.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the newly created `Bech32` instance if the address is valid,
    /// or an `Err` containing a `Bech32Error` if the address is invalid or encounters other errors.
    pub fn from_address(string: String) -> Result<Bech32, Bech32Error> {
        let (hrp, data) = split_bech32(string)?;

        let data_bytes = decode_data(data, hrp == *LOWERCASE_HRP)?;

        let lowercase_hrp: String = hrp.chars().map(|c| c.to_lowercase().to_string()).collect();

        validate_checksum(lowercase_hrp.as_bytes(), &data_bytes)?;

        Ok(Bech32 {
            hrp: lowercase_hrp,
            data: data_bytes[..data_bytes.len() - 6].to_vec(),
        })
    }

    /// Converts a `Bech32` instance to a Bech32-encoded address.
    ///
    /// This function constructs a Bech32-encoded address using the human-readable part (HRP) and data bytes
    /// from the provided `Bech32` instance. Bech32 is a human-readable format commonly used in Bitcoin and
    /// other cryptographic applications for encoding data using a base32 representation with error-detection codes.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the Bech32-encoded address if the conversion is successful,
    /// or an `Err` containing a `Bech32Error` if the conversion encounters errors.
    pub fn to_address(&self) -> Result<String, Bech32Error> {
        let hrp_bytes = self.hrp.clone().into_bytes();
        let mut data_bytes = self.data.clone();

        data_bytes.extend(create_checksum(&data_bytes));

        let mut address = format!("{}{}", LOWERCASE_HRP, SEPARATOR);

        for byte in &data_bytes {
            address.push(ENCODING_ARRAY[*byte as usize]);
        }

        decode_data(address[3..].to_string(), self.hrp == *LOWERCASE_HRP)?;
        validate_checksum(&hrp_bytes, &data_bytes)?;

        Ok(address)
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

/// Splits a Bech32-encoded address into its human-readable part (HRP) and data segments.
///
/// This function takes a Bech32-encoded address as input and splits it into its HRP and data segments.
/// Bech32 is a human-readable format commonly used in Bitcoin and other cryptographic applications for encoding
/// data using a base32 representation with error-detection codes.
///
/// # Arguments
///
/// * `string` - The Bech32-encoded address to be split.
///
/// # Returns
///
/// Returns a `Result` containing a tuple of `(HRP, data)` if the address is valid and can be split,
/// or an `Err` containing a `Bech32Error` if the address is invalid or encounters other errors.
fn split_bech32(string: String) -> Result<(String, String), Bech32Error> {
    if !(8..=90).contains(&string.len()) {
        return Err(Bech32Error::InvalidLength);
    } else if !string.starts_with(&format!("{}{}", LOWERCASE_HRP, SEPARATOR))
        && !string.starts_with(&format!("{}{}", UPPERCASE_HRP, SEPARATOR))
    {
        return Err(Bech32Error::InvalidHRP);
    }

    let (hrp, data) = string.split_at(2);
    Ok((hrp.to_string(), data[1..].to_string()))
}

/// Checks if a byte is excluded from data based on specific criteria.
///
/// This function determines whether a given byte is excluded from data based on certain predefined criteria.
/// The criteria for exclusion include specific bytes that are not allowed to be part of data in certain contexts.
///
/// # Arguments
///
/// * `byte` - The byte value to be checked.
///
/// # Returns
///
/// Returns `true` if the byte is excluded from data based on the predefined criteria, and `false` otherwise.
fn is_byte_excluded_from_data(byte: u8) -> bool {
    byte == b'1' || byte == b'b' || byte == b'i' || byte == b'o'
}

/// Decodes Bech32-encoded data into a vector of bytes.
///
/// This function decodes Bech32-encoded data into a vector of bytes. It performs various checks on the input data
/// to ensure its validity, including verifying that the bytes are alphanumeric and not excluded based on specific criteria.
/// It also validates the case of the bytes based on whether the Bech32 encoding is expected to be lowercase or uppercase.
///
/// # Arguments
///
/// * `data` - The Bech32-encoded data to be decoded.
/// * `is_lowercase` - A boolean flag indicating whether the Bech32 encoding is expected to be lowercase.
///
/// # Returns
///
/// Returns a `Result` containing the decoded vector of bytes if the decoding is successful,
/// or an `Err` containing a `Bech32Error` if the decoding encounters errors or invalid data.
fn decode_data(data: String, is_lowercase: bool) -> Result<Vec<u8>, Bech32Error> {
    let mut data_bytes = Vec::new();

    for byte in data.bytes() {
        if !byte.is_ascii_alphanumeric() || is_byte_excluded_from_data(byte) {
            return Err(Bech32Error::InvalidData);
        } else if (is_lowercase && byte.is_ascii_uppercase())
            || (!is_lowercase && byte.is_ascii_lowercase())
        {
            return Err(Bech32Error::InvalidCase);
        }

        data_bytes.push(DECODING_ARRAY[byte.to_ascii_lowercase() as usize] as u8);
    }

    Ok(data_bytes)
}

/// Expands the human-readable part (HRP) of a Bech32 address into its encoded form.
///
/// This function takes a slice of bytes representing the human-readable part (HRP) of a Bech32 address and
/// expands it into its encoded form as required for the Bech32 encoding process. The expansion involves
/// splitting each byte into two parts, with the first part being the higher 5 bits and the second part being
/// the lower 3 bits, followed by the addition of a separator (0) between the two parts.
///
/// # Arguments
///
/// * `hrp` - The human-readable part (HRP) of the Bech32 address.
///
/// # Returns
///
/// Returns a vector of bytes containing the expanded encoded form of the HRP.
///
fn expand_hrp(hrp: &[u8]) -> Vec<u8> {
    let mut encoded_hrp = Vec::new();

    for byte in hrp {
        encoded_hrp.push(*byte >> 5);
    }

    encoded_hrp.push(0);

    for byte in hrp {
        encoded_hrp.push(*byte & ENCODING_MASK);
    }

    encoded_hrp
}

/// Performs the polynomial modulo operation on a sequence of values.
///
/// This function calculates the result of the polynomial modulo operation on a sequence of values,
/// as required for generating a Bech32 checksum. It uses a predefined generator polynomial and
/// bitwise operations to compute the result.
///
/// # Arguments
///
/// * `values` - A vector of bytes representing the values on which the polynomial modulo operation is performed.
///
/// # Returns
///
/// Returns the result of the polynomial modulo operation as a 32-bit unsigned integer.
fn polymod(values: Vec<u8>) -> u32 {
    let mut checksum = 1;
    let mut byte;

    for value in values {
        byte = (checksum >> 25) as u8;
        checksum = (checksum & 0x1ffffff) << 5 ^ (value as u32);

        for (i, gen) in GEN.iter().enumerate() {
            if (byte >> i) & 1 == 1 {
                checksum ^= gen
            }
        }
    }

    checksum
}

/// Validates the checksum of a Bech32 address.
///
/// This function validates the checksum of a Bech32 address by performing the necessary expansion of the human-readable
/// part (HRP) and data, and then calculating the polynomial modulo operation using the expanded values. If the calculated
/// checksum is not equal to 1, it indicates an invalid checksum, resulting in an error.
///
/// # Arguments
///
/// * `hrp` - The human-readable part (HRP) of the Bech32 address.
/// * `data` - The data part of the Bech32 address.
///
/// # Returns
///
/// Returns `Ok(())` if the checksum validation is successful, or an `Err` containing a `Bech32Error`
/// if the checksum validation fails.
fn validate_checksum(hrp: &[u8], data: &[u8]) -> Result<(), Bech32Error> {
    let mut encoded_hrp = expand_hrp(hrp);

    encoded_hrp.extend(data);

    if polymod(encoded_hrp) != 1u32 {
        return Err(Bech32Error::InvalidChecksum);
    }

    Ok(())
}

/// Creates a checksum for a Bech32 address.
///
/// This function creates a checksum for a Bech32 address by expanding the human-readable part (HRP),
/// combining it with the data, and then performing the polynomial modulo operation to calculate the checksum.
/// The calculated checksum is then encoded and returned as a vector of bytes to be used in the Bech32 address.
///
/// # Arguments
///
/// * `data` - The data part of the Bech32 address.
///
/// # Returns
///
/// Returns a vector of bytes representing the calculated checksum.
fn create_checksum(data: &[u8]) -> Vec<u8> {
    let mut values = expand_hrp(&LOWERCASE_HRP.to_string().into_bytes());
    let mut checksum = Vec::new();

    values.extend(data);
    values.extend(&[0u8; 6]);

    let polymod = polymod(values) ^ 1;

    for index in 0..6 {
        checksum.push(((polymod >> (5 * (5 - index))) & 0x1f) as u8);
    }

    checksum
}

#[cfg(test)]
mod bech32_test {
    use super::Bech32;
    use crate::bech32::{
        bech32_constants::{DECODING_ARRAY, ENCODING_ARRAY},
        bech32_errors::Bech32Error,
    };

    #[test]
    pub fn test_arrays() {
        for i in ENCODING_ARRAY {
            let decoded_value = DECODING_ARRAY[i as usize];

            if decoded_value == -1 {
                assert!(decoded_value != -1);
                return;
            }

            let encoded_value = ENCODING_ARRAY[decoded_value as usize];
            assert!(i == encoded_value);
        }
    }

    #[test]
    pub fn test_valid_address() -> Result<(), Bech32Error> {
        let b32_1 = Bech32::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc".to_string())?;
        let b32_2 = Bech32::from_address("tb1q79gkmhfaw9szkn8fmg22llkx2sfhlx7ykptww6".to_string())?;
        let b32_3 = Bech32::from_address("TB1Q79GKMHFAW9SZKN8FMG22LLKX2SFHLX7YKPTWW6".to_string())?;

        assert_eq!(
            b32_1.data(),
            [
                0, 19, 23, 27, 17, 6, 20, 16, 19, 13, 29, 8, 28, 7, 12, 26, 4, 0, 17, 18, 0, 19,
                21, 24, 14, 17, 7, 16, 2, 8, 12, 6, 13
            ]
        );
        assert_eq!(
            b32_2.data(),
            [
                0, 30, 5, 8, 22, 27, 23, 9, 29, 14, 5, 16, 2, 22, 19, 7, 9, 27, 8, 10, 10, 31, 31,
                22, 6, 10, 16, 9, 23, 31, 6, 30, 4
            ]
        );
        assert_eq!(b32_3.data(), b32_2.data());

        Ok(())
    }

    #[test]
    pub fn test_invalid_address() {
        assert!(Bech32::from_address("mtsQWBEUBxTfqRpaaHtRwW6KicGnLCdqzW".to_string()).is_err());
        assert!(
            Bech32::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtcd".to_string())
                .is_err()
        );
        assert!(
            Bech32::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtC".to_string()).is_err()
        );
        assert!(
            Bech32::from_address("tb1qnhm3x5sndagu816yq3jqn4cw38szgvxdydxxtC".to_string()).is_err()
        );
        assert!(
            Bech32::from_address("tb1qnhm3x5sndagu816yq3jqq4cw38szgvxdydxxtC".to_string()).is_err()
        );
        assert!(
            Bech32::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgVxdydxxtc".to_string()).is_err()
        );
        assert!(
            Bech32::from_address("TB1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc".to_string()).is_err()
        );
    }

    #[test]
    pub fn test_address_decoding() -> Result<(), Bech32Error> {
        let b32_1 = Bech32::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc".to_string())?;
        let b32_2 = Bech32::from_address("tb1q79gkmhfaw9szkn8fmg22llkx2sfhlx7ykptww6".to_string())?;

        let address1 = b32_1.to_address()?;
        let address2 = b32_2.to_address()?;

        assert_eq!(address1, "tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc");
        assert_eq!(address2, "tb1q79gkmhfaw9szkn8fmg22llkx2sfhlx7ykptww6");

        Ok(())
    }
}
