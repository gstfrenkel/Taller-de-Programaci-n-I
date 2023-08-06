use super::network_constants::MSG_BLOCK_DATA_TYPE;
use super::network_error::NetworkError;
use crate::block_mod::blockchain::BlockChain;
use crate::block_mod::mempool::Mempool;
use crate::block_mod::transaction::Transaction;
use crate::messages::inv::Inv;
use crate::messages::tx::Tx;
use crate::messages::message_constants::{INV_COMMAND, TX_COMMAND};
use crate::{
    block_mod::{block::Block, utxo::UnspentTx},
    messages::{
        get_data::GetData,
        header::MessageHeader,
        headers::Headers,
        inventory::Inventory,
        message_constants::{BLOCK_COMMAND, HEADERS_COMMAND, PING_COMMAND},
        ping::Ping,
        pong::Pong
    },
    settings_mod::settings::Settings,
};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

/// Manages the "tx" command received over the network.
/// 
/// This function reads a transaction from the provided `stream` and adds it to the mempool.
/// 
/// # Arguments
/// * `stream` - A mutable reference to a TcpStream used for network communication.
/// * `mempool` - An Arc-wrapped Mutex for accessing and modifying the mempool.
/// 
/// # Returns
/// * `Result<(), NetworkError>` - A result indicating success or an error of type NetworkError.
/// 
/// # Errors
/// The function can return a NetworkError in the following cases:
/// * If there is an error while reading the transaction from the stream.
/// * If there is an error acquiring the lock on the mempool.
/// * If there is an error while adding the transaction to the mempool.
fn manage_tx_command(
    stream: &mut TcpStream,
    mempool: &Arc<Mutex<Mempool>>
) -> Result<(), NetworkError>{
    let tx = Transaction::from_bytes(stream).map_err(|_| NetworkError::Broadcasting)?;
    mempool.lock().map_err(|_| NetworkError::Broadcasting)?.add(tx);

    println!("New transaction has been succesfully received.");

    Ok(())
}

/// Manages the "block" command received over the network.
/// 
/// This function reads a block from the provided `stream` and performs the necessary operations
/// to update the blockchain, UTXO set, and mempool if the block is valid.
/// 
/// # Arguments
/// * `stream` - A mutable reference to a TcpStream used for network communication.
/// * `blockchain` - An Arc-wrapped Mutex for accessing and modifying the blockchain.
/// * `utxo` - An Arc-wrapped Mutex for accessing and modifying the UTXO set.
/// * `mempool` - An Arc-wrapped Mutex for accessing and modifying the mempool.
/// 
/// # Returns
/// * `Result<(), NetworkError>` - A result indicating success or an error of type NetworkError.
/// 
/// # Errors
/// The function can return a NetworkError in the following cases:
/// * If there is an error while reading the block from the stream.
/// * If the block fails the proof-of-work or proof-of-inclusion validation.
/// * If there is an error acquiring the lock on the blockchain, UTXO set, or mempool.
/// * If there is an error while updating the blockchain, UTXO set, or mempool.
pub fn manage_block_command(
    stream: &mut TcpStream,
    blockchain: &Arc<Mutex<BlockChain>>,
    utxo: &Arc<Mutex<UnspentTx>>,
    mempool: &Arc<Mutex<Mempool>>
) -> Result<(), NetworkError> {
    let block = Block::from_bytes(stream).map_err(|_| NetworkError::Broadcasting)?;

    if block.proof_of_work() && block.proof_of_inclusion(){
        utxo.lock().map_err(|_| NetworkError::Broadcasting)?.update(&block);
        mempool.lock().map_err(|_| NetworkError::Broadcasting)?.update(&block);
        blockchain.lock().map_err(|_| NetworkError::Broadcasting)?.add(block);
    }

    println!("New block has been succesfully received.");

    Ok(())
}

/// Manages the "inv" command received over the network.
/// 
/// This function reads an Inv message from the provided `stream`, extracts the inventory,
/// and sends a GetData message requesting the corresponding data.
/// 
/// # Arguments
/// * `header` - The MessageHeader of the received message.
/// * `settings` - An Arc-wrapped reference to the network settings.
/// * `stream` - A mutable reference to a TcpStream used for network communication.
/// 
/// # Returns
/// * `Result<(), NetworkError>` - A result indicating success or an error of type NetworkError.
/// 
/// # Errors
/// The function can return a NetworkError in the following cases:
/// * If there is an error while reading the Inv message from the stream.
/// * If there is no inventory available in the Inv message.
/// * If there is an error while creating and sending the GetData message.
/// * If there is an error while writing to the stream.
fn manage_inv_command(
    header: MessageHeader,
    settings: &Arc<Settings>,
    stream: &mut TcpStream,
) -> Result<(), NetworkError> {
    let inv = Inv::from_bytes(header, stream)?;

    let inventory = inv.get_inventories().pop().ok_or(NetworkError::Broadcasting)?;
    let get_data = GetData::new(settings.get_start_string(), vec![inventory]);
    stream.write_all(&get_data.as_bytes()).map_err(|_| NetworkError::Broadcasting)?;

    Ok(())
}

/// Handles the headers command received from the network.
///
/// # Arguments
///
/// * `header` - The message header.
/// * `settings` - The network settings.
/// * `stream` - The TCP stream for communication.
/// * `blocks` - The map of blocks.
/// * `utxo_set` - The unspent transaction set.
///
/// # Returns
///
/// An empty result if successful, or a `NetworkError` if an error occurs.
fn manage_headers_command(
    header: MessageHeader,
    settings: &Arc<Settings>,
    stream: &mut TcpStream
) -> Result<(), NetworkError> {
    let new_headers = Headers::from_bytes(header, stream)?;

    let block_header = new_headers.get_headers().pop().ok_or(NetworkError::Broadcasting)?;

    if block_header.proof_of_work() {
        let inv = vec![Inventory::new(
            MSG_BLOCK_DATA_TYPE,
            block_header.get_header(),
        )];
    
        let get_data = GetData::new(settings.get_start_string(), inv);
    
        stream.write_all(&get_data.as_bytes()).map_err(|_| NetworkError::Broadcasting)?;   
    }
    Ok(())
}

/// Handles the ping command received from the network.
///
/// # Arguments
///
/// * `header` - The message header.
/// * `settings` - The network settings.
/// * `stream` - The TCP stream for communication.
///
/// # Returns
///
/// An empty result if successful, or a `NetworkError` if an error occurs.
fn manage_ping_command(
    header: MessageHeader,
    settings: &Arc<Settings>,
    stream: &mut TcpStream,
) -> Result<(), NetworkError> {
    let ping = Ping::from_bytes(header, stream).map_err(|_| NetworkError::Broadcasting)?;
    println!("Message Ping received with nonce: {}", ping.get_nonce());
    let pong = Pong::new(settings.get_start_string(), ping.get_nonce());
    stream.write_all(&pong.as_bytes()).map_err(|_| NetworkError::Broadcasting)?;
    println!("Message Pong sent with nonce: {}\n", ping.get_nonce());
    Ok(())
}

/// Handles incoming messages based on their command type.
///
/// # Arguments
///
/// * `header` - The message header.
/// * `settings` - The network settings.
/// * `stream` - The TCP stream for communication.
/// * `blocks` - The hashmap storing blocks.
/// * `utxo_set` - The unspent transaction set.
///
/// # Returns
///
/// An empty result if successful, or a `NetworkError` if an error occurs.
pub fn handle_messages(
    header: MessageHeader,
    settings: &Arc<Settings>,
    stream: &mut TcpStream,
    blockchain: &Arc<Mutex<BlockChain>>,
    utxo: &Arc<Mutex<UnspentTx>>,
    mempool: &Arc<Mutex<Mempool>>
) -> Result<(), NetworkError> {
    let command_name: &str = header.get_command_name().as_str();

    match command_name {
        PING_COMMAND => {
            manage_ping_command(header, settings, stream)?;
        }
        HEADERS_COMMAND => {
            manage_headers_command(header, settings, stream)?;
        },
        INV_COMMAND => {
            manage_inv_command(header, settings, stream)?;
        },
        TX_COMMAND => {
            manage_tx_command(stream, mempool)?;
        },
        BLOCK_COMMAND => {
            manage_block_command(stream, blockchain, utxo, mempool)?;
        }
        _ => {
            stream.read_exact(&mut vec![0u8; header.get_payload_size() as usize]).map_err(|_| NetworkError::Broadcasting)?;
        }
    };
    Ok(())
}

/// Performs broadcasting of messages to multiple TCP streams.
///
/// # Arguments
///
/// * `settings` - The network settings.
/// * `streams` - The vector of TCP streams to broadcast messages to.
/// * `blocks` - The hashmap storing blocks.
/// * `utxo_set` - The unspent transaction set.
///
/// # Returns
///
/// An empty result if successful, or a `NetworkError` if an error occurs.
pub fn broadcasting(
    settings: Arc<Settings>,
    streams: &mut Vec<Arc<Mutex<TcpStream>>>,
    blockchain: Arc<Mutex<BlockChain>>,
    utxo: Arc<Mutex<UnspentTx>>,
    mempool: Arc<Mutex<Mempool>>
) -> Result<Vec<JoinHandle<()>>, NetworkError> {
    println!("Broadcasting has begun.\n");

    let mut handles_broadcasting = vec![];

    for stream in streams{
        let shared_stream = stream.clone();
        let shared_settings = settings.clone();
        let shared_blockchain = blockchain.clone();
        let shared_utxo = utxo.clone();
        let shared_mempool  = mempool.clone();

        let handle_broadcasting = thread::spawn(move || {
            if let Ok(mut locked_stream) = shared_stream.lock() {
                loop {
                    if let Ok(header) = MessageHeader::from_bytes(&mut *locked_stream){
                        if let Err(err) = handle_messages(header, &shared_settings, &mut locked_stream, &shared_blockchain, &shared_utxo, &shared_mempool) {
                            println!("{:?}", err);
                            return;
                        }
                    } else{
                        return;
                    }
                }
            }
        });

        handles_broadcasting.push(handle_broadcasting);
    }

    Ok(handles_broadcasting)
}

/// Broadcasts a new transaction to a list of network streams.
/// 
/// This function sends the provided `broadcast_tx_msg` to each stream in the `streams` list.
/// 
/// # Arguments
/// * `broadcast_tx_msg` - The Tx message to broadcast.
/// * `streams` - A reference to a vector of Arc-wrapped Mutex-wrapped TcpStream objects representing network streams.
/// 
/// # Returns
/// * `Result<(), NetworkError>` - A result indicating success or an error of type NetworkError.
/// 
/// # Errors
/// The function can return a NetworkError in the following case:
/// * If there is an error while writing to a network stream.
pub fn broadcast_new_txn(broadcast_tx_msg: Tx, streams: &Vec<Arc<Mutex<TcpStream>>>) -> Result<(), NetworkError>{
    let tx = broadcast_tx_msg.as_bytes();
    let mut count = 0;

    println!("Tx broadcast message\n:{:?}", broadcast_tx_msg);
    println!("Tx to be broadcasted:\n: {:?}\n\n", broadcast_tx_msg.transaction);
    println!("Is Tx segwit: {:?}\n\n", broadcast_tx_msg.transaction.is_segwit());

    for stream in streams{
        if let Ok(mut locked_stream) = stream.lock() {
            match locked_stream.write_all(&tx){
                Ok(_) => {count += 1},
                Err(error) => {println!("Error when attempting to broadcast created transaction: {}", error)}
            }
            
            drop(locked_stream);
        }
    }

    println!("Succesfully broadcasted new transaction to {} nodes", count);

    Ok(())
}
