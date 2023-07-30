use std::{
    net::{
        TcpStream,
        TcpListener,
        SocketAddr
    },
    sync::{
        mpsc::{
            Sender,
            Receiver,
            self
        },
        Arc,
        Mutex
    },
};

use bitcoin::{
    settings_mod::{
        settings::Settings,
        settings_error::SettingError
    },
    network::{
        handshake::handshake,
        headers_download::headers_download,
        block_download::{block_download, take_n_streams},
        broadcasting::broadcasting,
    },
    block_mod::{
        block_header::BlockHeader,
        utxo::UnspentTx,
        blockchain::BlockChain,
        block::Block,
        mempool::Mempool
    },
    block_saver::download_blocks,
    wallet_utils::update_wallet::update_wallet,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!(
            "{:?}",
            SettingError::FileNotFound
        );
        return;
    }

    let settings = match Settings::from_file(&args[1]) {
        Ok(settings) => settings,
        Err(err) => {
            println!("Attempt to recover node settings has failed: {:?}.", err);
            return;
        }
    };

    let listener: TcpListener = match TcpListener::bind("127.0.0.1:8000"){
        Ok(listener) => listener,
        Err(err) => {
            println!("Attempt to create listener for wallet has failed: {}.", err);
            return;
        }
    };
    
    let mut streams: Vec<TcpStream> = match handshake(&settings) {
        Ok(streams) => streams,
        Err(err) => {
            println!("HOLAAAAA A{:?}", err);
            return;
        }
    };

    let headers: Vec<BlockHeader> = match headers_download(&settings, &mut streams ) {
        Ok(headers) => headers,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    let (tx, rx): (Sender<Block>, Receiver<Block>) = mpsc::channel();

    let blockchain: Arc<Mutex<BlockChain>> = Arc::new(Mutex::new(BlockChain::new()));
    let utxo: Arc<Mutex<UnspentTx>> = Arc::new(Mutex::new(UnspentTx::new()));
    let mempool: Arc<Mutex<Mempool>> = Arc::new(Mutex::new(Mempool::new()));
    let mut streams: Vec<Arc<Mutex<TcpStream>>> = streams.into_iter().map(|streams| Arc::new(Mutex::new(streams))).collect();
    let settings = Arc::new(settings);

    let block_download_thread = download_blocks(blockchain.clone(), utxo.clone(), rx);

    if let Err(err) = block_download(settings.clone(), &streams, &headers, tx){
        println!("Attempt to download blocks has failed: {:?}.", err);
        return;
    }

    match block_download_thread.join() {
        Ok(_) => {
            if let Ok(locked_blockchain) = blockchain.lock(){
                println!("Block download has succesfully finished after {} downloads.", locked_blockchain.cant_blocks());
                drop(locked_blockchain);
            } else{
                println!("Attempt to lock blockchain has failed.");
                return;
            }
        },
        Err(_) => {
            println!("Attempt to join block download threads has failed.");
            return;
        },
    }

    let cant_streams = streams.len();

    let streams_tx_broadcast = take_n_streams(&mut streams, cant_streams / 2);

    let handles_broadcasting = match broadcasting(settings.clone(), &mut streams, blockchain.clone(), utxo.clone(), mempool.clone()) {
        Ok(handle_broadcasting) => handle_broadcasting,
        Err(_) => {
            //println!("{:?}", err);
            return;
        }
    };
    
    let (wallet, _addr): (TcpStream, SocketAddr) = match listener.accept(){
        Ok((wallet, _addr)) => (wallet, _addr),
        Err(err) =>{
            println!("Attempt to establish connection with wallet has failed: {}.", err);
            return
        }
    };

    if let Err(err) = update_wallet(wallet, blockchain, utxo, mempool, settings, streams_tx_broadcast){
        println!("Attempt to communicate with wallet has failed: {:?}.", err);
    };
     
    for handle_broadcasting in handles_broadcasting{
        if handle_broadcasting.join().is_err(){
            println!("Attempt to join broadcast threads has failed.");
        }
    }
}
