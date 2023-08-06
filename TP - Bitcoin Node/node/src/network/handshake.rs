use super::{
    super::messages::{header::MessageHeader, version::Version},
    network_constants::{DIG_COMMAND, DURATION_TIMEOUT_MILLIS, SHORT_ARG, VERSION_ACEPTED, SERVICES_ACEPTED},
    network_error::NetworkError,
};

use crate::{
    messages::message_constants::{SEND_HEADERS_COMMAND, VERACK_COMMAND},
    settings_mod::settings::Settings,
};

use std::{
    io::Write,
    process::Command,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream},
    str::FromStr,
    time::Duration,
};

/// Performs peer discovery by querying a DNS seed for IPv6 addresses.
///
/// # Arguments
///
/// * `dns_seed` - The DNS seed to query.
///
/// # Returns
///
/// A vector of discovered IPv6 addresses, or a `NetworkError` if an error occurs.
fn peer_discovery(dns_seed: &String) -> Result<Vec<Ipv6Addr>, NetworkError> {
    let dns_output: std::process::Output = Command::new(DIG_COMMAND)
        .arg(SHORT_ARG)
        .arg(dns_seed)
        .output()?;
    let dns_output: String = String::from_utf8(dns_output.stdout)?;

    let peers: Vec<&str> = dns_output.lines().collect();

    // No se pudo sacar este unwrap por esta dentro de un map, preguntar como sacarlo
    let ips: Vec<Ipv6Addr> = peers
        .iter()
        .filter_map(|ip_string| match Ipv4Addr::from_str(ip_string){
            Ok(ip) => Some(ip.to_ipv6_mapped()),
            Err(_) => None
        })
        .collect();

    Ok(ips)
}

/// Checks if a given version is compatible with the accepted versions.
///
/// # Arguments
///
/// * `version` - The version to check.
///
/// # Returns
///
/// `true` if the version is compatible, `false` otherwise.
fn is_version_compatible(version: &Version) -> bool {
    version.get_version() == VERSION_ACEPTED && version.get_services() == SERVICES_ACEPTED
}

/// Performs the handshake with peer nodes.
///
/// # Arguments
///
/// * `settings` - The network settings.
///
/// # Returns
///
/// A vector of established TCP streams to the peer nodes.
pub fn handshake(settings: &Settings) -> Result<Vec<TcpStream>, NetworkError> {
    println!("Node handshake has begun...");
    let ips: Vec<Ipv6Addr> = peer_discovery(settings.get_dns_seed())?;

    let mut streams: Vec<TcpStream> = Vec::new();

    for ip in ips {

        // Se crea nuestro version
        let version = Version::new(ip, settings);
        //Se establece la conexion
        let socket = SocketAddr::new(IpAddr::V6(ip), settings.get_port());

        let mut stream = match TcpStream::connect_timeout(
            &socket,
            Duration::from_millis(DURATION_TIMEOUT_MILLIS),
        ) {
            Ok(s) => s,
            Err(_) => continue,
        };

        //Se envia nuestro version
        stream.write_all(&version.as_bytes())?;

        //Se recibe el version del peer
        let header_version = match MessageHeader::from_bytes(&mut stream){
            Ok(header) => header,
            Err(_) => continue
        };
        
        let version_peer = match Version::from_bytes(header_version, &mut stream) {
            Ok(version) => version,
            Err(_) => continue,
        };

        
        if !is_version_compatible(&version_peer) || (version_peer.get_services() & 8 == 0){
            continue;
        }
        
        //Se crea nuestro verack
        let verack = MessageHeader::new(settings.get_start_string(), VERACK_COMMAND.to_string());
        
        //Se envia nuestro verack
        stream.write_all(&verack.as_bytes())?;

        //Se recibe el verack del peer
        match MessageHeader::from_bytes(&mut stream) {
            Ok(header) => header,
            Err(_) => break,
        };

        let send_headers = MessageHeader::new(
            settings.get_start_string(),
            SEND_HEADERS_COMMAND.to_string(),
        );

        stream.write_all(&send_headers.as_bytes())?;

        streams.push(stream);
    }
        println!("\nConnection has been succesfully established with {} nodes.", streams.len());
    
    if let Some(last) = streams.last() {
        println!("\nStream to be used: {:?}\n", last)
    }
        
    Ok(streams)
}
