use node::{block_mod::{script::Script, transaction::Transaction, tx_out::TxOut, tx_in::TxIn}};
use bitcoin_hashes::{hash160, sha256, Hash, sha256d};
use secp256k1::{SecretKey, Secp256k1, PublicKey, Message};
//use bech32::wit_prog::WitnessProgram;

use crate::bech32::witness_program::WitnessProgram;

use super::create_transaction_error::TransactionCreateError;

/// Checks if a given string is a valid Bech32-encoded address.
///
/// This function verifies whether the provided string represents a valid Bech32-encoded
/// address. It does so by attempting to parse the address into a `WitnessProgram` using
/// the `from_address` method. If the parsing is successful, the function returns `true`;
/// otherwise, it returns `false`.
///
/// # Arguments
///
/// * `address`: A string representing the Bech32-encoded address to be checked.
///
/// # Returns
///
/// A boolean value indicating whether the provided string is a valid Bech32-encoded address.
pub fn is_string_bech32(address: String) -> bool{
    WitnessProgram::from_address(address).is_ok()
}

/// Checks if a given byte slice is a valid Bech32-encoded address.
///
/// This function converts the provided byte slice into a UTF-8 string and then
/// calls the `is_string_bech32` function to determine if the resulting string
/// is a valid Bech32-encoded address.
///
/// # Arguments
///
/// * `address`: A byte slice representing the Bech32-encoded address to be checked.
///
/// # Returns
///
/// A boolean value indicating whether the provided byte slice is a valid Bech32-encoded address.
fn is_array_bech32(address: &[u8]) -> bool{
    is_string_bech32(String::from_utf8_lossy(address).to_string())
}

/// Generates a Bitcoin address from a given public key and specifies whether it's for P2WPKH (SegWit) or not.
///
/// This function calculates the hash160 of the provided public key and constructs a Bitcoin address based on whether
/// it's for P2WPKH (SegWit) or not. For P2WPKH, a witness program is created and converted to an address. For non-P2WPKH,
/// a Base58Check encoding is applied to the version byte, hash160, and checksum.
///
/// # Arguments
///
/// * `public_key`: A byte slice representing the public key.
/// * `p2wpkh`: A boolean indicating whether the address should be for P2WPKH (SegWit).
///
/// # Returns
///
/// A vector of bytes representing the generated Bitcoin address.
pub fn address_from_pubkey(public_key: &[u8], p2wpkh: bool) -> Vec<u8>{
    let h160 = hash160::Hash::hash(public_key).to_byte_array();

    if p2wpkh{
        let witness_program = WitnessProgram{
            version: 0,
            program: h160.to_vec(),
        };
    
        return witness_program.to_address().unwrap().as_bytes().to_vec();
    }

    let version_prefix = [0x6f];
    let double_hash = sha256d::Hash::hash(&[&version_prefix[..], &h160[..]].concat());    
    let checksum = &double_hash[..4];
    
    let input = [&version_prefix[..], &h160[..], checksum].concat();

    bs58::encode(input).into_vec()
}

/// Decodes a Base58-encoded Bitcoin address into its raw bytes representation.
///
/// This function decodes a Base58-encoded Bitcoin address and returns its raw bytes representation
/// after removing the version byte and checksum.
///
/// # Arguments
///
/// * `address`: A reference to a vector of bytes representing the Base58-encoded Bitcoin address.
///
/// # Returns
///
/// A vector of bytes representing the decoded Bitcoin address.
fn decode_base58(address: &Vec<u8>) -> Vec<u8> {
    if let Ok(combined) = bs58::decode(address).into_vec(){
        return combined[1..combined.len() - 4].to_vec();
    }

    Vec::new()
}

/// Generates a P2PKH or P2WPKH script from a public key.
///
/// This function generates a Pay-to-Public-Key-Hash (P2PKH) or Pay-to-Witness-Public-Key-Hash (P2WPKH) script
/// from a given public key.
///
/// # Arguments
///
/// * `public_key`: A slice of bytes representing the public key.
/// * `p2wpkh`: A boolean flag indicating whether to generate a P2WPKH script (true) or P2PKH script (false).
///
/// # Returns
///
/// A vector of bytes representing the generated script.
pub fn pk_script_from_pubkey(public_key: &[u8], p2wpkh: bool) -> Vec<u8> {
    let address = address_from_pubkey(public_key, p2wpkh);

    pk_script_from_address(&address, p2wpkh)
}

/// Generates a P2PKH or P2WPKH script from an address.
///
/// This function generates a Pay-to-Public-Key-Hash (P2PKH) or Pay-to-Witness-Public-Key-Hash (P2WPKH) script
/// from a given address.
///
/// # Arguments
///
/// * `address`: A vector of bytes representing the address.
/// * `p2wpkh`: A boolean flag indicating whether to generate a P2WPKH script (true) or P2PKH script (false).
///
/// # Returns
///
/// A vector of bytes representing the generated script.
pub fn pk_script_from_address(address: &Vec<u8>, p2wpkh: bool) -> Vec<u8>{
    if p2wpkh{
        let string_address = String::from_utf8_lossy(address).to_string(); 

        if let Ok(witness_program) = WitnessProgram::from_address(/*"tb".to_string(),*/ string_address){
            return witness_program./*to_scriptpubkey()*/to_pk_script();
        }
    }

    let h160 = decode_base58(address);
    let script = Script::new(Some(vec![vec![0x76], vec![0xa9], h160, vec![0x88], vec![0xac]]));
    script.as_bytes()
}

/// Creates a list of transaction outputs (TxOut) from a list of target addresses and amounts, along with a fee.
///
/// This function generates a list of transaction outputs (TxOut) based on the provided target addresses
/// and amounts, and calculates the total transaction amount including the fee.
///
/// # Arguments
///
/// * `targets`: A vector of tuples containing the target address (as bytes) and the corresponding amount (in satoshis).
/// * `fee`: The transaction fee amount in satoshis.
///
/// # Returns
///
/// A tuple containing the generated vector of transaction outputs (TxOut) and the total transaction amount.
fn create_txout_list(targets: Vec<(Vec<u8>, i64)>, fee: i64) -> (Vec<TxOut>, i64){
    let mut txout_list = vec![];
    let mut total_amount = fee;

    for (address, amount) in targets {        
        let script = pk_script_from_address(&address, is_array_bech32(&address));

        txout_list.push(TxOut::new(amount, script));
        
        total_amount += amount;
    }

    (txout_list, total_amount)
}

/// Checks if a given transaction output (TxOut) was used in an unconfirmed transaction.
///
/// This function iterates through the list of used transaction outputs and compares each one with
/// the provided transaction output (TxOut) to determine if it has been used.
///
/// # Arguments
///
/// * `txout`: The transaction output (TxOut) to be checked for usage.
/// * `used_txouts`: A slice containing the list of used transaction outputs to compare against.
///
/// # Returns
///
/// `true` if the transaction output was used, otherwise `false`.
fn was_txout_used(txout: TxOut, used_txouts: &[TxOut]) -> bool{
    for used_txout in used_txouts{
        if txout.as_bytes() == used_txout.as_bytes(){
            return true;
        }
    }

    false
}

/// Creates a list of transaction inputs (TxIn) and corresponding amounts to spend from a list of unspent transaction outputs (UTXOs).
///
/// This function constructs a list of transaction inputs and corresponding amounts from a list of UTXOs.
/// It ensures that the total spent amount does not exceed the total_amount parameter.
/// It also checks if the provided UTXOs have been used before (in the used_txouts list) and skips them if they have.
///
/// # Arguments
///
/// * `utxo`: A mutable vector of tuples containing UTXOs as (prev_txout_hash, prev_txout_index, TxOut).
/// * `total_amount`: The total amount to spend in the transaction.
/// * `used_txouts`: A slice containing the list of used transaction outputs to avoid.
///
/// # Returns
///
/// Returns a Result containing a tuple of two vectors: one containing the transaction inputs (TxIn) and another
/// containing the corresponding amounts. If successful, returns `Ok`, otherwise returns an error of type
/// `TransactionCreateError` indicating the reason for failure.
fn create_txin_list(mut utxo: Vec<(Vec<u8>, u32, TxOut)>, total_amount: i64, used_txouts: &[TxOut]) -> Result<(Vec<TxIn>, Vec<i64>), TransactionCreateError> {
    let mut txin_list = vec![];
    let mut amount_list = vec![];
    let mut unavailable_output = false;
    let mut spent_amount = 0;

    while spent_amount < total_amount {
        if let Some(txout) = utxo.pop() {
            if was_txout_used(txout.2.clone(), used_txouts){
                unavailable_output = true;
                continue;
            }

            txin_list.push(TxIn::new(txout.0, txout.1, vec![], 0xffffffff));

            amount_list.push(txout.2.get_value());

            spent_amount += txout.2.get_value();
        } else {
            if unavailable_output{
                return Err(TransactionCreateError::UnavailableOutput);
            } else{
                return Err(TransactionCreateError::InsufficientFunds);
            }
        }
    }

    amount_list.push(spent_amount - total_amount);  //Change difference that must return to the sender's address

    Ok((txin_list, amount_list))
}

/// Signs a transaction by adding signatures to its inputs.
///
/// This function signs the provided transaction by adding signatures to its inputs.
/// It iterates over each input and determines whether it needs to use P2WPKH (Pay-to-Witness-Public-Key-Hash) or P2PKH
/// (Pay-to-Public-Key-Hash) signing based on the `p2wpkh` parameter.
///
/// # Arguments
///
/// * `transaction`: A mutable reference to the transaction to be signed.
/// * `private_key`: The private key used for signing.
/// * `pk_script`: The public key script associated with the input being signed.
/// * `p2wpkh`: A boolean indicating whether to use P2WPKH signing.
/// * `amount_list`: A slice containing the list of amounts corresponding to each input.
fn sign_transaction(transaction: &mut Transaction, private_key: SecretKey, pk_script: &[u8], p2wpkh: bool, amount_list: &[i64]){
    let secp = Secp256k1::new();
    let pubkey = PublicKey::from_secret_key(&secp, &private_key).serialize().to_vec();

    for i in 0..transaction.get_tx_in_list().len(){
        if p2wpkh /*&& txout_info[i].p2wpkh()*/{
            let signature_hash = transaction.p2wpkh_signature_hash(i, pk_script.to_vec(), amount_list.to_vec());
            let message = Message::from_hashed_data::<sha256::Hash>(&signature_hash); 
            let mut signature = secp.sign_ecdsa(&message, &private_key).serialize_der().to_vec();

            signature.push(0x01);

            let script = vec![signature, pubkey.clone()];

            transaction.set_witness(script);
        } else{
            let signature_hash = transaction.p2pkh_signature_hash(i, pk_script);
            let message = Message::from_hashed_data::<sha256::Hash>(&signature_hash); 
            let mut signature = secp.sign_ecdsa(&message, &private_key).serialize_der().to_vec();

            signature.push(0x01);

            let script = Script::new(Some(vec![signature, pubkey.clone()]));

            transaction.set_signature(i, script.as_bytes());
            transaction.set_witness(vec![]);
        }
    }
}

/// Creates a new transaction by assembling inputs, outputs, and signing.
///
/// This function constructs a new transaction by combining the provided inputs, outputs,
/// and other required information. It then signs the transaction using the provided private key
/// and creates the necessary signatures for each input.
///
/// # Arguments
///
/// * `targets`: A vector of tuples containing recipient addresses and amounts.
/// * `utxo`: A vector of tuples containing UTXO (unspent transaction outputs) information.
/// * `private_key`: A slice representing the private key of the sender.
/// * `fee`: The transaction fee to be paid.
/// * `used_txouts`: A slice of used transaction outputs.
/// * `p2wpkh`: A boolean indicating whether to use P2WPKH signing.
///
/// # Returns
///
/// A `Result` containing the newly created and signed transaction if successful, or a `TransactionCreateError` if an error occurs.
pub fn create_transaction(targets: Vec<(Vec<u8>, i64)>, utxo: Vec<(Vec<u8>, u32, TxOut)>, private_key: &[u8], fee: i64, used_txouts: &[TxOut], p2wpkh: bool) -> Result<Transaction, TransactionCreateError> {    
    let secp = Secp256k1::new();

    let private_key = SecretKey::from_slice(private_key).map_err(|_| TransactionCreateError::PrivateKey)?;
    let public_key = PublicKey::from_secret_key(&secp, &private_key).serialize().to_vec();
    let pk_script = pk_script_from_pubkey(&public_key, p2wpkh);

    let (mut txout_list, total_amount) = create_txout_list(targets, fee);
    let (txin_list, mut amount_list)= create_txin_list(utxo, total_amount, used_txouts)?;

    if let Some(change) = amount_list.pop(){
        if change > 0{
            let txout_change = TxOut::new(change, pk_script.clone());
            txout_list.push(txout_change);
        }
    }

    let mut transaction = Transaction::new(1, txin_list, txout_list, 0, p2wpkh);

    sign_transaction(&mut transaction, private_key, &pk_script, p2wpkh, &amount_list);

    Ok(transaction)
}

#[cfg(test)]
mod create_transactions_test {
    use std::{str::FromStr, io::Cursor};

    use node::{messages::{read_from_bytes::{decode_hex, encode_hex}, compact_size::CompactSizeUInt}, block_mod::{tx_in::TxIn, tx_out::TxOut, script::Script, transaction::Transaction}};
    use bitcoin_hashes::*;
    use secp256k1::{Secp256k1, Message, SecretKey, PublicKey};

    use crate::transactions::{create_transactions::{decode_base58, is_string_bech32, address_from_pubkey, is_array_bech32, sign_transaction}, create_transaction_error::TransactionCreateError};

    use super::{pk_script_from_address};

    #[test]
    pub fn create_transaction() -> Result<(), TransactionCreateError>{
        // total de la txout que voy a usar 0.01875221
        // quiero depositar desde la cuenta address a la cuenta target 0.012345

        let address = b"n1mDu5Zd5qS75vqK1yqnKmEZQzDyncQqj4".to_vec();
        let target = b"mp3PDnKDtxPYrPKcYLGX1pXMe6KwAsfquD".to_vec();

        //let pub_key = "02E641B11A0FB5A761814D0F166ADC4E654037C844B44226219AE3D6947EBC4DA6";
        let private_key = "740A9C5D2BD171E99DDDC268A26179FCAD9BFE9A7A8188725EDA0D1D9F6D2264";

        let mut prev_tx = decode_hex("7a56640d6c89ce4744ab77c5332c87fec02c58720a7fc1ba19d6b6546f5b29e8")?;
        prev_tx.reverse();
        let prev_index = 0; // 0.01875221 -0.012345 - 0.003

        let txin = TxIn::new(prev_tx, prev_index, vec![], 0xffffffff);

        // calculo el cambio
        let change_amount = 0.0009 * 100000000.0;
        let change_h160 = decode_base58(&address);
        let change_script = Script::new(Some(vec![vec![0x76], vec![0xa9], change_h160, vec![0x88], vec![0xac]]));
        let change_txout = TxOut::new(change_amount as i64, change_script.as_bytes());


        let target_amount = 0.0021 * 100000000.0;
        let target_h160 = decode_base58(&target);
        let target_script = Script::new(Some(vec![vec![0x76], vec![0xa9], target_h160, vec![0x88], vec![0xac]]));
        let target_txout = TxOut::new(target_amount as i64, target_script.as_bytes());

        let mut tx = Transaction::new(1, vec![txin], vec![change_txout, target_txout], 0, false);

        let secp = Secp256k1::new();

        let signature_hash = tx.p2pkh_signature_hash(0, &change_script.as_bytes());



        let private_key = SecretKey::from_str(private_key)?;

        let message = Message::from_hashed_data::<sha256::Hash>(&signature_hash);

        let der = secp.sign_ecdsa(&message, &private_key).serialize_der().to_vec();

        let sig = vec![der, vec![1_u8]].concat();

        let sec = PublicKey::from_secret_key(&secp, &private_key).serialize().to_vec();

        let signature_script = Script::new(Some(vec![sig, sec]));

        tx.set_signature(0, signature_script.as_bytes());

        println!("{:?}", encode_hex(&tx.as_bytes(false))?);

        Ok(())
    }

    #[test]
    pub fn test_address_from_public_key() -> Result<(), TransactionCreateError>{
        let public_key = decode_hex("02E641B11A0FB5A761814D0F166ADC4E654037C844B44226219AE3D6947EBC4DA6")?;
        let address = b"n1mDu5Zd5qS75vqK1yqnKmEZQzDyncQqj4".to_vec();

        let address_calculated = address_from_pubkey(&public_key, false);

        assert_eq!(address_calculated, address);
        Ok(())
    }

    #[test]
    pub fn test_pk_script_from_address() -> Result<(), TransactionCreateError>{
        let address = b"mzx5YhAH9kNHtcN481u6WkjeHjYtVeKVh2".to_vec(); //ejemplo sacado del libro

        let h160 = decode_hex("d52ad7ca9b3d096a38e752c2018e6fbc40cdf26f")?;
        let pk_script = vec![vec![0x76], vec![0xa9], vec![20], h160, vec![0x88], vec![0xac]].concat();
        
        let pk_script_calculated = pk_script_from_address(&address, false);

        assert_eq!(pk_script, pk_script_calculated);

        Ok(())
    }

    #[test]
    pub fn test_pk_script_from_pubkey() -> Result<(), TransactionCreateError>{
        let public_key = decode_hex("0362599B444272856B51E7EE10A4B70A683A9965AD3859E4D75E9B9EC136F84144")?;
        let address = address_from_pubkey(&public_key, false);
        let pk_script = pk_script_from_address(&address, false);

        println!("{:?}", pk_script);

        Ok(())
    }

    #[test]
    pub fn test_create_transaction_2() -> Result<(), TransactionCreateError> {
        let address = b"mq5boK8wasubp4QHZ349damhWQLCthdrKP".to_vec();
        let target = b"n3yL92bzbMkicfYwUS3K7huHj81ew877ob".to_vec();

        let private_key = "11063638E1C47A9EEEDCDB476654644B00F7BFF9798031CFBB1EB9DA4D8B51F4";

        let mut prev_tx = decode_hex("3464a5386b818c901b910d96ee71bce0ea9a4465719ea458deea9df81e8504f5")?;
        prev_tx.reverse();
        let prev_index = 0;

        let txin = TxIn::new(prev_tx, prev_index, vec![], 0xffffffff);

        // calculo el cambio
        let change_amount = 0.0009 * 100000000.0;
        let change_h160 = decode_base58(&address);
        let change_script = Script::new(Some(vec![vec![0x76], vec![0xa9], change_h160, vec![0x88], vec![0xac]]));
        let change_txout = TxOut::new(change_amount as i64, change_script.as_bytes());


        let target_amount = 0.0021 * 100000000.0;
        let target_h160 = decode_base58(&target);
        let target_script = Script::new(Some(vec![vec![0x76], vec![0xa9], target_h160, vec![0x88], vec![0xac]]));
        let target_txout = TxOut::new(target_amount as i64, target_script.as_bytes());

        let mut tx = Transaction::new(1, vec![txin], vec![change_txout, target_txout], 0, false);

        let secp = Secp256k1::new();

        let signature_hash = tx.p2pkh_signature_hash(0, &change_script.as_bytes());

        let private_key = SecretKey::from_str(private_key)?;

        let message = Message::from_hashed_data::<sha256::Hash>(&signature_hash);

        let der = secp.sign_ecdsa(&message, &private_key).serialize_der().to_vec();

        let sig = vec![der, vec![1_u8]].concat();
        println!("len sig {}", sig.len());

        let sec = PublicKey::from_secret_key(&secp, &private_key).serialize().to_vec();

        println!("len sec {}", sec.len());

        let signature_script = Script::new(Some(vec![sig, sec]));

        tx.set_signature(0, signature_script.as_bytes());

        println!("len script {}", signature_script.as_bytes().len());

        println!("{:?}", encode_hex(&tx.as_bytes(false))?);

        assert_eq!("0100000001f504851ef89deade58a49e7165449aeae0bc71ee960d911b908c816b38a56434000000006a47304402204f191223988c6121ba7d0d6657db265d308140ac00282caed4336017d25c232002201ffff97e6b44a99b462b84bf831f3f15a67b2f05f6a18fb106cd4bd79974bb4701210362599b444272856b51e7ee10a4b70a683a9965ad3859e4d75e9b9ec136f84144ffffffff02905f0100000000001976a91468e5bfff52953b4179bf03c990d8ac81aa65173e88ac50340300000000001976a914f64fd1289550f634e20bf2ac6e95fb5d5fdbd50d88ac00000000", "0100000001f504851ef89deade58a49e7165449aeae0bc71ee960d911b908c816b38a56434000000006a47304402204f191223988c6121ba7d0d6657db265d308140ac00282caed4336017d25c232002201ffff97e6b44a99b462b84bf831f3f15a67b2f05f6a18fb106cd4bd79974bb4701210362599b444272856b51e7ee10a4b70a683a9965ad3859e4d75e9b9ec136f84144ffffffff02905f0100000000001976a91468e5bfff52953b4179bf03c990d8ac81aa65173e88ac50340300000000001976a914f64fd1289550f634e20bf2ac6e95fb5d5fdbd50d88ac00000000");
        
        

        Ok(())
    }

    #[test]
    pub fn test_is_bech32() {
        assert!(is_string_bech32("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string()));
        assert!(!is_string_bech32("tb1qw308d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string()));
    }
    
    #[test]
    pub fn test_pk_script() {
        let address = "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string();
        let address2 = b"mq5boK8wasubp4QHZ349damhWQLCthdrKP".to_vec();

        assert_eq!(pk_script_from_address(&address.as_bytes().to_vec(), true), [0, 20, 117, 30, 118, 232, 25, 145, 150, 212, 84, 148, 28, 69, 209, 179, 163, 35, 241, 67, 59, 214]);
        assert_eq!(pk_script_from_address(&address2, true), pk_script_from_address(&address2, false));

        let asas = CompactSizeUInt::from_number([117, 30, 118, 232, 25, 145, 150, 212, 84, 148, 28, 69, 209, 179, 163, 35, 241, 67, 59, 214].len().try_into().unwrap());

        println!("{:?}", asas.as_bytes());
    }

    #[test]
    pub fn test_pk_to_address() -> Result<(), TransactionCreateError>{
        let public_key1 = decode_hex("0279BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798")?;
        let public_key2 = decode_hex("FEABBD73DDD97F2C00CC6023B38C08214736CAF26A8ED91CE1ABA30D8BE46B35")?;

        let address1 = address_from_pubkey(&public_key1, true);
        let address2 = address_from_pubkey(&public_key2, false);

        assert_eq!(address1, "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string().as_bytes());
        assert!(is_array_bech32(&address1));
        assert!(!is_array_bech32(&address2));

        Ok(())
    }
    
    #[test]
    pub fn test_signature() -> Result<(), TransactionCreateError>{
        let h160: Vec<u8> = vec![0, 20, 117, 30, 118, 232, 25, 145, 150, 212, 84, 148, 28, 69, 209, 179, 163, 35, 241, 67, 59, 214];
        let private_key = decode_hex("11063638E1C47A9EEEDCDB476654644B00F7BFF9798031CFBB1EB9DA4D8B51F4")?;
        let private_key = SecretKey::from_slice(&private_key).map_err(|_| TransactionCreateError::PrivateKey)?;

        let secp = Secp256k1::new();

        let message = Message::from_hashed_data::<sha256::Hash>(&h160);
        let der = secp.sign_ecdsa(&message, &private_key).serialize_der().to_vec();
        let sig = vec![der, vec![1_u8]].concat();
        let sec = PublicKey::from_secret_key(&secp, &private_key).serialize().to_vec();

        println!("{:?}", sig);
        println!("{:?}", sec);

        Ok(())
    }

    #[test]
    pub fn test_p2wpkh_tx() -> Result<(), TransactionCreateError>{
        let tx_hex = decode_hex("0100000002fff7f7881a8099afa6940d42d1e7f6362bec38171ea3edf433541db4e4ad969f0000000000eeffffffef51e1b804cc89d182d279655c3aa89e815b1b309fe287d9b2b55d57b90ec68a0100000000ffffffff02202cb206000000001976a9148280b37df378db99f66f85c95a783a76ac7a6d5988ac9093510d000000001976a9143bde42dbee7e4dbe6a21b2d50ce2f0167faa815988ac11000000")?;
        let tx_hex_final = decode_hex("01000000000102fff7f7881a8099afa6940d42d1e7f6362bec38171ea3edf433541db4e4ad969f00000000494830450221008b9d1dc26ba6a9cb62127b02742fa9d754cd3bebf337f7a55d114c8e5cdd30be022040529b194ba3f9281a99f2b1c0a19c0489bc22ede944ccf4ecbab4cc618ef3ed01eeffffffef51e1b804cc89d182d279655c3aa89e815b1b309fe287d9b2b55d57b90ec68a0100000000ffffffff02202cb206000000001976a9148280b37df378db99f66f85c95a783a76ac7a6d5988ac9093510d000000001976a9143bde42dbee7e4dbe6a21b2d50ce2f0167faa815988ac000247304402203609e17b84f6a7d30c80bfa610b5b4542f32a8a0d5447a12fb1366d7f01cc44a0220573a954c4518331561406f90300e8f3358f51928d43c212a8caed02de67eebee0121025476c2e83188368da1ff3e292e7acafcdb3566bb0ad253f62fc70f07aeee635711000000")?;
        
        let pk_script = decode_hex("00201d0f172a0ecb48aee1be1f2687d2963ae33f71a1")?;
        let private_key = SecretKey::from_slice(&decode_hex("619c335025c7f4012e556c2a58b2506e30b8511b53ade95ea316fd8c3286feb9")?).map_err(|_| TransactionCreateError::PrivateKey)?;
        
        let mut cursor = Cursor::new(tx_hex);
        let mut tx = Transaction::from_bytes(&mut cursor).unwrap();

        let amount_list = vec![625000000, 600000000];

        sign_transaction(&mut tx, private_key, &pk_script, true, &amount_list);

        cursor = Cursor::new(tx_hex_final);
        let final_tx = Transaction::from_bytes(&mut cursor).unwrap();

        assert_eq!(final_tx.get_witness()[1].stack_items[0], tx.get_witness()[1].stack_items[0]);
        assert_eq!(final_tx.get_witness()[1].stack_items[1], tx.get_witness()[1].stack_items[1]);

        Ok(())
    }

    #[test]
    pub fn aaa(){
        let address = address_from_pubkey(&decode_hex(&"031280622311CD5642DD6ED8815349FEE1C1FE9CD961AE48981C3309F2816DDBAD".to_string()).unwrap(),true);
        println!("{}", String::from_utf8_lossy(&address));
    }
}
