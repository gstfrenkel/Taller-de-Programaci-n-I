use super::{bech32_errors::WitnessProgramError, bech32::Bech32};

#[derive(PartialEq, Debug, Clone)]
pub struct WitnessProgram {
    pub version: u8,
    pub program: Vec<u8>
}

impl WitnessProgram{
    pub fn from_address(address: String) -> Result<WitnessProgram, WitnessProgramError>{
        let b32 = Bech32::from_address(address)?;

        let (version, program) = b32.data.split_at(1);

        let witness_program = WitnessProgram{
            version: version.to_vec()[0],
            program: convert_bits(program.to_vec(), 5, 8)?
        };

        witness_program.validate()?;
        Ok(witness_program)
    }

    pub fn to_address(&self) -> Result<String, WitnessProgramError>{
        self.validate()?;    //WitnessProgram is a public structure and it must be validated

        let program = convert_bits(self.program.clone(), 8, 5)?;
        
        let mut data = vec![self.version];
        data.extend(program);

        let b32 = Bech32{hrp: "tb".to_string(), data};
        let address = b32.to_address()?;
        WitnessProgram::from_address(address.clone())?;

        Ok(address)
    }

    pub fn to_pk_script(&self) -> Vec<u8>{
        let mut pk_script = vec![self.version];
        pk_script.push(self.program.len() as u8);
        pk_script.extend(&self.program);

        pk_script
    }

    pub fn validate(&self) -> Result<(), WitnessProgramError> {
        if self.version != 0 {
            return Err(WitnessProgramError::InvalidVersion);
        } else if self.program.len() != 20 && self.program.len() != 32 {
            return Err(WitnessProgramError::InvalidLength)
        }

        Ok(())
    }
}


//Cambiar
fn convert_bits(data: Vec<u8>, from: u32, to: u32) -> Result<Vec<u8>, WitnessProgramError>{
    let mut acc: u32 = 0;
    let mut bits: u32 = 0;
    let mut ret: Vec<u8> = Vec::new();
    let maxv: u32 = (1<<to) - 1;

    for value in data {
        let v: u32 = value as u32;

        if (v >> from) != 0 {
            return Err(WitnessProgramError::InvalidInput)
        }

        acc = (acc << from) | v;
        bits += from;

        while bits >= to {
            bits -= to;
            ret.push(((acc >> bits) & maxv) as u8);
        }
    }

    if from == 8 && to == 5 {
        if bits > 0 {
            ret.push(((acc << (to - bits)) & maxv) as u8);
        }
    } else if bits >= from || ((acc << (to - bits)) & maxv) != 0 {
        return Err(WitnessProgramError::InvalidPadding)
    }

    Ok(ret)
}

#[cfg(test)]
mod bech32_test {
    use crate::bech32::bech32_errors::WitnessProgramError;
    use super::WitnessProgram;

    #[test]
    pub fn test_valid_address(){
        assert!(WitnessProgram::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc".to_string()).is_ok());
        assert!(WitnessProgram::from_address("tb1q79gkmhfaw9szkn8fmg22llkx2sfhlx7ykptww6".to_string()).is_ok());
    }

    #[test]
    pub fn test_invalid_address(){
        assert!(WitnessProgram::from_address("mtsQWBEUBxTfqRpaaHtRwW6KicGnLCdqzW".to_string()).is_err());
        assert!(WitnessProgram::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtcd".to_string()).is_err());
        assert!(WitnessProgram::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtC".to_string()).is_err());
        assert!(WitnessProgram::from_address("tb1qnhm3x5sndagu816yq3jqn4cw38szgvxdydxxtC".to_string()).is_err());
        assert!(WitnessProgram::from_address("tb1qnhm3x5sndagu816yq3jqq4cw38szgvxdydxxtC".to_string()).is_err());
    }

    #[test]
    pub fn test_address_decoding() -> Result<(), WitnessProgramError>{
        let wp1 = WitnessProgram::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc".to_string())?;
        let wp2 = WitnessProgram::from_address("tb1q79gkmhfaw9szkn8fmg22llkx2sfhlx7ykptww6".to_string())?;
        
        let address1 = wp1.to_address()?;
        let address2 = wp2.to_address()?;

        assert_eq!(address1, "tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc");
        assert_eq!(address2, "tb1q79gkmhfaw9szkn8fmg22llkx2sfhlx7ykptww6");

        Ok(())
    }

    #[test]
    pub fn test_pk_script() -> Result<(), WitnessProgramError>{
        let wp1 = WitnessProgram::from_address("tb1qnhm3x5sndagu8v6yq3jqn4cw38szgvxdydxxtc".to_string())?;
        let wp2 = WitnessProgram::from_address("tb1q79gkmhfaw9szkn8fmg22llkx2sfhlx7ykptww6".to_string())?;
        
        assert_eq!(wp1.to_pk_script(), [0, 20, 157, 247, 19, 82, 19, 111, 81, 195, 179, 68, 4, 100, 9, 215, 14, 137, 224, 36, 48, 205]);
        assert_eq!(wp2.to_pk_script(), [0, 20, 241, 81, 109, 221, 61, 113, 96, 43, 76, 233, 218, 20, 175, 254, 198, 84, 19, 127, 155, 196]);

        Ok(())
    }
}
