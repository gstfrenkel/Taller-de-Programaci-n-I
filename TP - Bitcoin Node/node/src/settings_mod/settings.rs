use crate::messages::read_from_bytes::decode_hex;
use crate::settings_mod::settings_constants::*;
use crate::settings_mod::settings_error::SettingError;
use std::collections::HashMap;
use std::net::Ipv6Addr;
use std::str::FromStr;
use std::{env, fs};

/// Configuration settings for network communication.
#[derive(Debug)]
pub struct Settings {
    dns_seed: String,
    protocol_version: i32,
    services: u64,
    port: u16,
    ip: Ipv6Addr,
    user_agent: String,
    start_height: i32,
    relay: bool,
    start_string: Vec<u8>,
}

impl Settings {
    /// Reads the settings from command-line arguments and loads them from a file.
    ///
    /// This function collects the command-line arguments and expects the path to a settings file as the
    /// second argument. It attempts to read the settings from the file and returns an `Option<Settings>`
    /// representing the loaded settings if successful, or `None` if there was an error.
    ///
    /// # Returns
    ///
    /// - `Some(settings)`: The loaded settings if successful.
    /// - `None`: If there was an error in reading the settings file or if the command-line arguments
    ///   were not provided correctly.
    pub fn read_settings() -> Option<Settings> {
        let args: Vec<String> = env::args().collect();

        if args.len() != 2 {
            println!("{:?}", SettingError::FileNotFound);
            return None;
        }

        let path = &args[1];

        match Settings::from_file(path) {
            Ok(settings) => Some(settings),
            Err(err) => {
                println!("Error when reading settings: {:?}", err);
                None
            }
        }
    }

    /// Loads the settings from a file.
    ///
    /// This function reads the settings from a file located at the specified path and returns a
    /// `Result<Settings, SettingError>` representing the loaded settings if successful, or an error if
    /// there was a problem in reading or parsing the file.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice representing the path to the settings file.
    ///
    /// # Returns
    ///
    /// - `Ok(settings)`: The loaded settings if successful.
    /// - `Err(err)`: If there was an error in reading or parsing the settings file.
    ///
    /// # Errors
    ///
    /// The function can return the following errors:
    ///
    /// - `SettingError::FileNotFound`: If the settings file was not found.
    /// - `SettingError::TokenNotFound`: If a required token is missing in the settings file.
    /// - `SettingError::FieldNotFound`: If a required field is missing in the settings file.
    /// - `SettingError::ParseError`: If there was an error in parsing a field value from the settings file.
    /// - `SettingError::DecodeError`: If there was an error in decoding a hex string from the settings file.
    pub fn from_file(path: &str) -> Result<Settings, SettingError> {
        let mut parser_config: HashMap<String, String> = HashMap::new();
        let file = fs::read_to_string(path)?;

        for line in file.lines() {
            let token: Vec<&str> = line.split(EQUAL).collect();

            if matches!(
                token[0],
                DNS_SEED
                    | PROCOCOL_VERSION
                    | SERVICES
                    | PORT
                    | IP
                    | USER_AGENT
                    | START_HEIGHT
                    | RELAY
                    | START_STRING
            ) {
                parser_config.insert(token[0].to_string(), token[1].to_string());
            } else {
                return Err(SettingError::TokenNotFound);
            }
        }

        //println!("Archivo en hash map: {:?}", parser_config);

        Ok(Settings {
            dns_seed: parser_config
                .get(DNS_SEED)
                .ok_or(SettingError::FieldNotFound)?
                .to_string(),
            protocol_version: i32::from_str(
                parser_config
                    .get(PROCOCOL_VERSION)
                    .ok_or(SettingError::FieldNotFound)?,
            )?,
            services: parser_config
                .get(SERVICES)
                .ok_or(SettingError::FieldNotFound)?
                .parse()?,
            port: parser_config
                .get(PORT)
                .ok_or(SettingError::FieldNotFound)?
                .parse()?,
            ip: Ipv6Addr::from_str(parser_config.get(IP).ok_or(SettingError::FieldNotFound)?)?,
            user_agent: parser_config
                .get(USER_AGENT)
                .ok_or(SettingError::FieldNotFound)?
                .to_string(),
            start_height: parser_config
                .get(START_HEIGHT)
                .ok_or(SettingError::FieldNotFound)?
                .parse()?,
            relay: parser_config
                .get(RELAY)
                .ok_or(SettingError::FieldNotFound)?
                .parse()?,
            start_string: decode_hex(
                parser_config
                    .get(START_STRING)
                    .ok_or(SettingError::FieldNotFound)?,
            )?,
        })
    }

    pub fn get_dns_seed(&self) -> &String {
        &self.dns_seed
    }
    pub fn get_protocol_version(&self) -> i32 {
        self.protocol_version
    }
    pub fn get_services(&self) -> u64 {
        self.services
    }
    pub fn get_port(&self) -> u16 {
        self.port
    }
    pub fn get_ip(&self) -> Ipv6Addr {
        self.ip
    }
    pub fn get_user_agent(&self) -> String {
        self.user_agent.clone()
    }
    pub fn get_start_height(&self) -> i32 {
        self.start_height
    }
    pub fn get_relay(&self) -> bool {
        self.relay
    }
    pub fn get_start_string(&self) -> Vec<u8> {
        self.start_string.clone()
    }
}
