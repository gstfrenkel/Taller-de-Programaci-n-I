use super::{
    headers_download::handle_other_message,
    network_constants::{DATE_LIMIT, MSG_BLOCK_DATA_TYPE},
};
use crate::{
    block_mod::{block::Block, block_header::BlockHeader},
    messages::{
        get_data::GetData, header::MessageHeader, inventory::Inventory,
        message_constants::BLOCK_COMMAND,
    },
    network::{network_constants::DATE_FORMAT, network_error::NetworkError},
    settings_mod::settings::Settings,
};
use chrono::DateTime;
use chrono::Utc;
use std::{
    io::Write,
    net::TcpStream,
    sync::mpsc::Sender,
    sync::{Arc, Mutex, MutexGuard},
    thread::{self, JoinHandle},
};

/// Manage a block download error by adding inventories to a shared collection.
///
/// This function is used to handle a block download error scenario. It takes a shared
/// collection of inventories along with a vector of new inventories to be added.
/// The function locks the shared collection, extends it with the new inventories,
/// and then releases the lock.
///
/// # Arguments
///
/// - `shared_inventories`: A reference-counted smart pointer to a mutex-protected vector of `Inventory` items.
/// - `invs`: A vector of `Inventory` items to be added to the shared collection.
fn manage_block_download_error(
    shared_inventories: &Arc<Mutex<Vec<Inventory>>>,
    invs: &[Inventory],
) {
    let mut locked_inv = match shared_inventories.lock() {
        Ok(locked_inv) => locked_inv,
        Err(_) => return,
    };

    locked_inv.extend(invs.to_vec());

    drop(locked_inv);
}

/// Downloads blocks from a network using multiple TCP streams and filters them based on date and merkle tree validation.
///
/// # Arguments
/// * `settings` - A reference to the network settings.
/// * `streams` - A mutable vector of TCP streams used for communication.
/// * `headers` - A vector of block headers to download.
/// * `utxo_set` - A mutable reference to the unspent transaction output set.
///
/// # Returns
/// A Result containing a HashMap of block headers to their corresponding blocks if successful,
/// otherwise a NetworkError indicating the reason for failure.
///
/// # Errors
/// This function can return a NetworkError if there are no available streams for block download,
/// if there's an error parsing the date, or if there are issues with network communication.
pub fn block_download(
    settings: Arc<Settings>,
    streams: &Vec<Arc<Mutex<TcpStream>>>,
    headers: &[BlockHeader],
    tx: Sender<Block>,
) -> Result<(), NetworkError> {
    let date_time = DateTime::parse_from_str(DATE_LIMIT, DATE_FORMAT)
        .map_err(|_| NetworkError::BlockDownload)?
        .with_timezone(&Utc)
        .timestamp();

    let block_headers: Vec<&BlockHeader> = headers
        .iter()
        .filter(|block_header| block_header.get_time() > date_time as u32)
        .collect();

    println!("Blocks to be downloaded: {}", block_headers.len());

    let inventories: Vec<Inventory> = block_headers
        .iter()
        .map(|block_header| Inventory::new(MSG_BLOCK_DATA_TYPE, block_header.get_header()))
        .collect();

    let shared_inventories = Arc::new(Mutex::new(inventories));
    let mut threads: Vec<JoinHandle<()>> = vec![];

    for stream in streams {
        let shared_stream = stream.clone();
        let shared_settings = settings.clone();
        let shared_tx = tx.clone();
        let shared_inv = shared_inventories.clone();

        let thread = thread::spawn(move || {
            let mut locked_stream = match shared_stream.lock() {
                Ok(locked_stream) => locked_stream,
                Err(_) => return,
            };

            'thread_loop: loop {
                let mut locked_inv = match shared_inv.lock() {
                    Ok(locked_inv) => locked_inv,
                    Err(_) => continue,
                };

                let mut get_data_size = 100;

                if locked_inv.is_empty() {
                    drop(locked_inv);
                    break;
                } else if locked_inv.len() < 100 {
                    get_data_size = locked_inv.len();
                }

                let inv = take_invs(&mut locked_inv, get_data_size);

                drop(locked_inv);

                let get_data = GetData::new(shared_settings.get_start_string(), inv.clone());

                if locked_stream.write_all(&get_data.to_bytes()).is_err() {
                    println!("Failed to send Get Data message. Trying again...");
                    manage_block_download_error(&shared_inv, &inv);
                    continue 'thread_loop;
                }

                for _ in 0..get_data_size {
                    loop {
                        let header = match MessageHeader::from_bytes(&mut *locked_stream) {
                            Ok(header) => header,
                            Err(_) => {
                                manage_block_download_error(&shared_inv, &inv);
                                break 'thread_loop;
                            }
                        };

                        if header.get_command_name() == BLOCK_COMMAND {
                            break;
                        }

                        if handle_other_message(
                            &mut locked_stream,
                            header,
                            shared_settings.get_start_string(),
                        )
                        .map_err(|_| NetworkError::BlockDownload)
                        .is_err()
                        {
                            manage_block_download_error(&shared_inv, &inv);
                            break 'thread_loop;
                        }
                    }

                    let block = match Block::from_bytes(&mut *locked_stream) {
                        Ok(block) => block,
                        Err(_) => {
                            manage_block_download_error(&shared_inv, &inv);
                            break 'thread_loop;
                        }
                    };

                    if shared_tx.send(block).is_err() {
                        manage_block_download_error(&shared_inv, &inv);
                        break 'thread_loop;
                    }
                }
            }
        });

        threads.push(thread);
    }

    for handle in threads {
        handle.join().map_err(|_| NetworkError::BlockDownload)?;
    }

    Ok(())
}

pub fn take_invs(inventories: &mut MutexGuard<Vec<Inventory>>, amount: usize) -> Vec<Inventory> {
    let mut new_list = Vec::new();

    for _ in 0..amount {
        match inventories.pop() {
            Some(inv) => new_list.push(inv),
            None => {
                break;
            }
        };
    }

    new_list
}

pub fn take_streams(
    streams: &mut Vec<Arc<Mutex<TcpStream>>>,
    amount: usize,
) -> Vec<Arc<Mutex<TcpStream>>> {
    let mut new_list = Vec::new();

    for _ in 0..amount {
        match streams.pop() {
            Some(stream) => new_list.push(stream),
            None => {
                break;
            }
        };
    }

    new_list
}
