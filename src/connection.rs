use anyhow::{Result, bail};
use std::time::Instant;
use std::net::TcpStream;
use std::net::SocketAddr;
use std::net::Ipv4Addr;
use std::path::Path;

use super::protocol;

#[derive(Debug)]
enum ConnectionStatus {
    /// sent ping
    Pinged = 1,
    UnusedActive = 2,
    /// 1 if we are waiting for a non-blocking connect() to finish
    Connecting = 4,
    /// the termination of this connection was requested
    UnusedTermreq = 8,
    /// Set to 1 if you want this connection removed
    RemoveUnused = 16,
    /// 1 if gotten timeout
    TimeoutUnused = 32,
    /// 1 if we can encrypt outgoing traffic
    Encryptout = 64,
    /// 1 if we have to decrypt incoming traffic
    Decryptin = 128,
    /// 1 if this connection is part of a minimum spanning tree
    Mst = 256,
    /// 1 if this is a control connection
    Control = 512,
    /// 1 if this is a control connection requesting packet capture
    Pcap = 1024,
    /// 1 if this is a control connection requesting log dump
    Log = 2048,
    /// 1 if this connection supports ANSI escape codes
    LogColor = 4096,
    /// 1 if this is an invitation
    Invitation = 8192,
    /// 1 if the invitation has been consumed
    InvitationUsed = 16384,
    /// 1 if the connection should be added to the tarpit
    Tarpit = 32768,
}

#[derive(Debug)]
struct Timeout {
    tv: Instant,
    //cb: Option<fn(data: &mut Connection)>,
    //data: Option<Connection>,
    //node: Option<SplayNode>,

	//struct timeval tv;
	//timeout_cb_t cb;
	//void *data;
	//splay_node_t node;
}

impl Default for Timeout {
    fn default() -> Self {
        Self {
            tv: Instant::now(),
        }
    }
}


#[derive(Debug, Default)]
struct Outgoing {
    node: Node,
    timeout: u32,
    ev: Timeout,
}

#[derive(Debug, Default)]
struct Edge {}
#[derive(Debug, Default)]
struct Ecdsa {}
#[derive(Debug, Default)]
struct Node {}
#[derive(Debug, Default)]
struct Sptps {}

#[derive(Debug)]
struct Connection {
    /// name they claims to have
    name: String,
    /// the hostname of its real ip
    hostname: Option<String>,
    /// their real (internet) ip
    address: SocketAddr,
    /// used protocol
    protocol_major: u16,
    /// used protocol
    protocol_minor: u8,

    /// socket used for this connection
    socket: Option<TcpStream>,
    /// options used for this connection
    options: u32,
    /// status info
    status: ConnectionStatus,

    /// estimation for the weight of the edge for this connection
    estimated_weight: u32,
    /// time this connection was started, used for above estimation
    start_time: Instant,
    /// used to keep track of outgoing connections
    outgoing: Outgoing,

    node: Node,
    edge: Edge,

    ecdsa: Ecdsa,
    sptps: Sptps,

    outmaclength: u32,
    //log_level
 
    // is this only for RSA?
    //their_challenge: [u8; 32],
    //our_challenge: [u8; 32],

   	//struct buffer_t inbuf;
	//struct buffer_t outbuf;
	//io_t io;                        /* input/output event on this metadata connection */
	//uint32_t tcplen;                /* length of incoming TCPpacket */
	//uint32_t sptpslen;              /* length of incoming SPTPS packet */
	//int allow_request;              /* defined if there's only one request possible */

	//time_t last_ping_time;          /* last time we saw some activity from the other end or pinged them */

	//splay_tree_t *config_tree;      /* Pointer to configuration tree belonging to him */
}

impl Connection {
    fn myself(config_base: &Path) -> Result<Self> {
        let mut c = Connection {
            name: String::new(),
            hostname: None,
            address: SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), 0),
            protocol_major: protocol::PROT_MAJOR,
            protocol_minor: protocol::PROT_MINOR,
            ecdsa: Ecdsa::default(),
            sptps: Sptps::default(),
            edge: Edge::default(),
            estimated_weight: 0,
            start_time: Instant::now(),
            outgoing: Outgoing::default(),
            node: Node::default(),
            socket: None,
            options: 0,
            status: ConnectionStatus::UnusedActive,
            outmaclength: 0,
        };
        Ok(c)
    }
}
