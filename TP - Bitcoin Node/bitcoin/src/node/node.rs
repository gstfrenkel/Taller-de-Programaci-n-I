use bitcoin::settings_mod::settings::Settings;
use bitcoin::settings_mod::settings_error::{SettingError, SettingTypeError};

/*fn read_settings() -> Option<Settings>{
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(
            "{:?}",
            SettingError::new(
                SettingTypeError::FileNotFound,
                "Config file path not specified.".to_string()
            )
        );
        return;
    }

    let path = &args[1];

    match Settings::from_file(path) {
        Ok(settings) => Ok(settings),
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    }
}*/


pub struct Node{
    settings: Arc<Settings>,
    blockchain: Arc<Blockchain>,
    utxo: Arc<UnspentTx>,
    streams: Vec<TcpStream>,
    headers: Vec<BlockHeader>,
    channel: (Sender<Block>, Receiver<Block>),
    broadcast: TcpStream
}


impl Node{
    pub fn new() -> Option<Node>{
        let config = match Settings::read_settings() {
            Some(settings) => settings,
            None => return None,
        };

        let handshake = match handshake(&config) {
            Some(streams) => streams,
            None => return None,
        };

        let headers = match get_headers(&config, &handshake) {
            Some(headers) => headers,
            None => return None,
        };

        let stream = match handshake.pop() {
            Some(stream) => stream,
            None => {
                println!("{:?}", NetworkError::new(NetworkTypeError::HandShake, "handshake error".to_string()));
                return None;
            }
        };

        Node{
            settings: Arc::new(config),
            blockchain: Arc::new(Mutex::new(BlockChain::new())),
            utxo: Arc::new(Mutex::new(UnspentTx::new())),
            streams: handshake,
            headers,
            channel: mpsc::channel(),
            broadcast: stream,
        }
    }
}

pub fn get_all_headers(){
    message_header = MessageHeader::new("get");
    message = GetHeaders::new();




}
