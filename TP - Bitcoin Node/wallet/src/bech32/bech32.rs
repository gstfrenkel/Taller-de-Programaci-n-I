use super::{bech32_errors::Bech32Error, bech32_constants::*};

#[derive(Debug, Clone)]
pub struct Bech32 {
    pub hrp: String,
    pub data: Vec<u8>
}

impl Bech32{
    pub fn from_address(string: String) -> Result<Bech32, Bech32Error> {
        let (hrp, data) = split_bech32(string)?;

        let data_bytes = validate_data(data, hrp == "tb".to_string())?;

        validate_checksum(&hrp.as_bytes().to_vec(), &data_bytes)?;

        Ok(Bech32 {
            hrp,
            data: data_bytes[..data_bytes.len() - 6].to_vec(),
        })
    }

    pub fn to_address(&self) -> Result<String, Bech32Error>{
        let hrp_bytes = self.hrp.clone().into_bytes();
        let mut data_bytes = self.data.clone();

        data_bytes.extend(create_checksum(&data_bytes));

        let mut address = "tb1".to_string();

        for byte in &data_bytes{
            address.push(CHARSET[*byte as usize]);
        }

        validate_data(address[3..].to_string(), self.hrp == "tb".to_string())?;
        validate_checksum(&hrp_bytes, &data_bytes)?;

        Ok(address)
    }

    pub fn data(&self) -> &[u8]{
        &self.data
    }
}

fn split_bech32(string: String) -> Result<(String, String), Bech32Error>{
    let len: usize = string.len();

    if len < 8 || len > 90 {
        return Err(Bech32Error::InvalidLength);
    } else if !string.starts_with("tb1") && !string.starts_with("TB1"){
        return Err(Bech32Error::InvalidHRP);
    }

    let (hrp, data) = string.split_at(2);
    Ok((hrp.to_string(), data[1..].to_string()))
}

fn validate_data(data: String, is_lowercase: bool) -> Result<Vec<u8>, Bech32Error>{
    let mut data_bytes = Vec::new();

    for byte in data.bytes() {
        if !((byte >= b'0' && byte <= b'9') || (byte >= b'A' && byte <= b'Z') || (byte >= b'a' && byte <= b'z')) || byte == b'1' || byte == b'b' || byte == b'i' || byte == b'o'{
            return Err(Bech32Error::InvalidData);
        } else if (is_lowercase && byte >= b'A' && byte <= b'Z') || (!is_lowercase && byte >= b'a' && byte <= b'z'){
            return Err(Bech32Error::InvalidCase);
        }

        data_bytes.push(CHARSET_REV[byte as usize] as u8);
    }

    Ok(data_bytes)
}


fn encode_hrp(hrp: &[u8]) -> Vec<u8> {
    let mut encoded_hrp = Vec::new();

    for byte in hrp {
        encoded_hrp.push(*byte >> 5);
    }

    encoded_hrp.push(0);

    for byte in hrp {
        encoded_hrp.push(*byte & ENCODING_MASK);    //Since values must represented as 5-bit, any higher bits are set to 0 through the AND.
    }

    encoded_hrp
}

fn polymod(values: Vec<u8>) -> u32 {
    let mut checksum = 1;
    let mut byte;

    for value in values {
        byte = (checksum >> 25) as u8;
        checksum = (checksum & 0x1ffffff) << 5 ^ (value as u32);

        for i in 0..5 {
            if (byte >> i) & 1 == 1 {
                checksum ^= GEN[i]
            }
        }
    }

    checksum
}

fn validate_checksum(hrp: &[u8], data: &[u8]) -> Result<(), Bech32Error> {
    let mut encoded_hrp = encode_hrp(hrp);

    encoded_hrp.extend(data);

    if polymod(encoded_hrp) != 1u32{
        return Err(Bech32Error::InvalidChecksum);
    }
    Ok(())
}

fn create_checksum(data: &[u8]) -> Vec<u8>{
    let mut values = encode_hrp(&"tb".to_string().into_bytes());
    let mut checksum = Vec::new();

    values.extend(data);
    values.extend(&[0u8; 6]);

    let polymod = polymod(values) ^ 1;
    
    for index in 0..6 {
        checksum.push(((polymod >> 5 * (5 - index)) & 0x1f) as u8);
    }

    checksum
}


#[cfg(test)]
mod bech32_test {
    use crate::bech32::bech32_errors::Bech32Error;
    use super::Bech32;

    #[test]
    pub fn test_valid_address() -> Result<(), Bech32Error>{
        let b32_1 = Bech32::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc".to_string())?;
        let b32_2 = Bech32::from_address("tb1q79gkmhfaw9szkn8fmg22llkx2sfhlx7ykptww6".to_string())?;

        assert_eq!(b32_1.data(), [0, 19, 23, 27, 17, 6, 20, 16, 19, 13, 29, 8, 28, 7, 12, 26, 4, 0, 17, 18, 0, 19, 21, 24, 14, 17, 7, 16, 2, 8, 12, 6, 13]);
        assert_eq!(b32_2.data(), [0, 30, 5, 8, 22, 27, 23, 9, 29, 14, 5, 16, 2, 22, 19, 7, 9, 27, 8, 10, 10, 31, 31, 22, 6, 10, 16, 9, 23, 31, 6, 30, 4]);

        Ok(())
    }

    #[test]
    pub fn test_invalid_address(){
        assert!(Bech32::from_address("mtsQWBEUBxTfqRpaaHtRwW6KicGnLCdqzW".to_string()).is_err());
        assert!(Bech32::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtcd".to_string()).is_err());
        assert!(Bech32::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtC".to_string()).is_err());
        assert!(Bech32::from_address("tb1qnhm3x5sndagu816yq3jqn4cw38szgvxdydxxtC".to_string()).is_err());
        assert!(Bech32::from_address("tb1qnhm3x5sndagu816yq3jqq4cw38szgvxdydxxtC".to_string()).is_err());
        assert!(Bech32::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgVxdydxxtc".to_string()).is_err());
        assert!(Bech32::from_address("TB1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc".to_string()).is_err());
    }

    #[test]
    pub fn test_address_decoding() -> Result<(), Bech32Error>{
        let b32_1 = Bech32::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc".to_string())?;
        let b32_2 = Bech32::from_address("tb1q79gkmhfaw9szkn8fmg22llkx2sfhlx7ykptww6".to_string())?;
        
        let address1 = b32_1.to_address()?;
        let address2 = b32_2.to_address()?;

        assert_eq!(address1, "tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc");
        assert_eq!(address2, "tb1q79gkmhfaw9szkn8fmg22llkx2sfhlx7ykptww6");

        Ok(())
    }
}