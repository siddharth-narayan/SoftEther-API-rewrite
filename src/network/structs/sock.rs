enum SockType {
    SockTcp = 1,
    SockUdp = 2,
    SockInproc = 3,
    SockRudpListen = 5,
    SockReverseListen = 6,
}

// // Socket
// struct SOCK
// {
// 	REF *ref;					// Reference counter
// 	LOCK *lock;					// Lock
// 	LOCK *ssl_lock;				// Lock related to the SSL
// 	LOCK *disconnect_lock;		// Disconnection lock
// 	SOCKET socket;				// Socket number
// 	SSL *ssl;					// SSL object
// 	struct ssl_ctx_st *ssl_ctx;	// SSL_CTX
// 	char SniHostname[256];		// SNI host name
// 	UINT Type;					// Type of socket
// 	bool Connected;				// Connecting flag
// 	bool ServerMode;			// Server mode
// 	bool AsyncMode;				// Asynchronous mode
// 	bool SecureMode;			// SSL communication mode
// 	bool ListenMode;			// In listening
// 	BUF *SendBuf;				// Transmission buffer
// 	bool IpClientAdded;			// Whether it has been added to the list IP_CLIENT
// 	bool LocalOnly;				// Only local
// 	bool EnableConditionalAccept;	// Conditional Accept is Enabled
// 	IP RemoteIP;				// IP address of the remote host
// 	IP LocalIP;					// IP address of the local host
// 	char *RemoteHostname;		// Remote host name
// 	UINT RemotePort;			// Port number of the remote side
// 	UINT LocalPort;				// Port number of the local side
// 	UINT64 SendSize;			// Total size of the sent data
// 	UINT64 RecvSize;			// Total size of received data
// 	UINT64 SendNum;				// Number of sent data blocks
// 	UINT64 RecvNum;				// Number of received data blocks
// 	X *RemoteX;					// Certificate of the remote host
// 	X *LocalX;					// Certificate of the local host
// 	char *CipherName;			// Cipher algorithm name
// 	char *WaitToUseCipher;		// Set the algorithm name to want to use
// 	bool IgnoreRecvErr;			// Whether the RecvFrom error is ignorable
// 	bool IgnoreSendErr;			// Whether the SendTo error is ignorable
// 	UINT TimeOut;				// Time-out value
// 	SOCK_EVENT *SockEvent;		// Associated socket-event
// 	bool CancelAccept;			// Cancel flag of the Accept
// 	bool AcceptCanceled;		// Flag which shows canceling of the Accept
// 	bool WriteBlocked;			// Previous write is blocked
// 	bool NoNeedToRead;			// Is not required to read
// 	bool Disconnecting;			// Disconnecting
// 	bool UdpBroadcast;			// UDP broadcast mode
// 	void *Param;				// Any parameters
// 	bool IPv6;					// IPv6
// 	bool IsRawSocket;			// Whether it is a raw socket
// 	const char *SslVersion;		// SSL version
// 	UINT RawSocketIPProtocol;	// IP protocol number if it's a raw socket
// 	TUBE *SendTube;				// Tube for transmission
// 	TUBE *RecvTube;				// Tube for reception
// 	QUEUE *InProcAcceptQueue;	// Accept queue of the in-process socket
// 	EVENT *InProcAcceptEvent;	// Accept event of the in-process socket
// 	FIFO *InProcRecvFifo;		// Receive FIFO of the in-process socket
// 	UINT UdpMaxMsgSize;			// Maximum transmitting and receiving size at a time on UDP
// 	int CurrentTos;				// Current ToS value
// 	bool IsTtlSupported;		// Whether the TTL value is supported
// 	UINT CurrentTtl;			// Current TTL value
// 	RUDP_STACK *R_UDP_Stack;	// R-UDP stack
// 	char UnderlayProtocol[64];	// Underlying protocol
// 	char ProtocolDetails[256];	// Protocol details
// 	QUEUE *ReverseAcceptQueue;	// Accept queue for the reverse socket
// 	EVENT *ReverseAcceptEvent;	// Accept event for the reverse socket
// 	bool IsReverseAcceptedSocket;	// Whether it is a reverse socket
// 	IP Reverse_MyServerGlobalIp;	// Self global IP address when using the reverse socket
// 	UINT Reverse_MyServerPort;		// Self port number when using the reverse socket
// 	UCHAR Ssl_Init_Async_SendAlert[2];	// Initial state of SSL send_alert
// 	SSL_ACCEPT_SETTINGS SslAcceptSettings;	// SSL Accept Settings
// 	bool RawIP_HeaderIncludeFlag;

// #ifdef	ENABLE_SSL_LOGGING
// 	// SSL Logging (for debug)
// 	bool IsSslLoggingEnabled;	// Flag
// 	IO *SslLogging_Recv;		// for Recv
// 	IO *SslLogging_Send;		// for Send
// 	LOCK *SslLogging_Lock;		// Locking
// #endif	// ENABLE_SSL_LOGGING

// 	void *hAcceptEvent;			// Event for Accept

// 	// R-UDP socket related
// 	bool IsRUDPSocket;			// Whether this is R-UDP socket
// 	TUBE *BulkSendTube;			// Tube for Bulk send
// 	TUBE *BulkRecvTube;			// Tube for Bulk receive
// 	SHARED_BUFFER *BulkSendKey;	// Bulk send key
// 	SHARED_BUFFER *BulkRecvKey;	// Bulk receive key
// 	UINT RUDP_OptimizedMss;		// Optimal MSS value

// #ifdef	OS_UNIX
// 	pthread_t CallingThread;	// Thread that is calling the system call
// #endif	// OS_UNIX

// #ifdef	OS_WIN32
// 	void *hEvent;				// Event for asynchronous mode
// #endif	// OS_WIN32
// };

use crate::dns::{
    resolve, resolve_all, resolve_all_ipv4, resolve_all_ipv6, resolve_ipv4, resolve_ipv6,
};
use crate::network::ssl::{SslUpgradable, SslVerifyOption, SSL_CTX_CLIENT, SSL_CTX_SERVER};
use crate::network::structs::sock;
use crate::nullcheck;
use crate::{mem::structs::list::List, network::ssl::create_client_ctx};
use std::ffi::CStr;
use std::fmt::Arguments;
use std::io::{IoSlice, Read, Write};
use std::net::{
    IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6, TcpListener, ToSocketAddrs,
};
use std::os::fd::AsRawFd;
use std::slice;
use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::Duration;
use std::{
    default,
    ffi::{c_char, c_void},
    net::{SocketAddr, TcpStream, UdpSocket},
    ptr::{null, null_mut},
};

use openssl::ssl::{Ssl, SslStream};
use socket2::{Domain, SockAddr, Socket, Type};

use crate::{
    mem::structs::{buf::Buffer, fifo::Fifo, queue::Queue},
    network::{
        structs::cert::{K, X},
        util::IP,
    },
    object::{Lock, RefCounter},
};

pub struct Sock {
    ref_count: *mut RefCounter,
    lock: *mut Lock,
    ssl_lock: *mut Lock,
    disconnect_lock: *mut Lock,

    socket: i32,
    // TODO implement this with openssl crate -- ssl: *mut Ssl
    ssl: *mut Ssl,
    sni_hostname: [u8; 256],

    sock_type: u32,

    is_connected: bool,
    is_server: bool,
    is_async: bool,
    is_ssl_secured: bool,
    is_listening: bool,

    send_buf: *mut Buffer,
    ip_client_added: bool,
    only_local: bool,
    enable_conditional_accept: bool,

    remote: IP,
    local: IP,

    remote_hostname: *const c_char,
    remote_port: u32,
    local_port: u32,

    send_size: u64,
    recv_size: u64,
    send_count: u64,
    recv_count: u64,

    remote_cert: X,
    local_cert: X,

    cipher_name: *const c_char,
    wait_cipher_name: *const c_char,

    ignore_recv_errors: bool,
    ignore_send_errors: bool,

    timeout: u32,
    sock_event: *mut c_void, // SockEvent,

    cancel_accept: bool,
    is_accept_canceled: bool,

    write_blocked: bool,
    no_need_to_read: bool,
    disconnecting: bool,
    udp_broadcast: bool,
    param: *mut std::ffi::c_void,
    ipv6: bool,
    is_raw_socket: bool,
    ssl_version: *const c_char,
    raw_socket_ip_protocol: u32,

    send_tube: *mut c_void, // Tube,
    recv_tube: *mut c_void, // Tube,
    in_proc_accept_queue: *mut Queue<*mut c_void>,
    in_proc_accept_event: *mut c_void, // Event,
    in_proc_recv_fifo: *mut Fifo<*mut c_void>,

    udp_max_msg_size: u32,
    current_tos: i32,
    is_ttl_supported: bool,
    current_ttl: u32,
    r_udp_stack: *mut c_void, //RudpStack,

    underlay_protocol: [u8; 64],
    protocol_details: [u8; 256],

    reverse_accept_queue: *mut Queue<*mut c_void>,
    reverse_accept_event: *mut c_void, // Event,
    is_reverse_accepted_socket: bool,
    reverse_my_server_global_ip: IP,
    reverse_my_server_port: u32,

    ssl_init_async_send_alert: [u8; 2],
    // ssl_accept_settings: SslAcceptSettings,
    raw_ip_header_include_flag: bool,

    // #[cfg(feature = "enable_ssl_logging")]
    // is_ssl_logging_enabled: bool,
    // #[cfg(feature = "enable_ssl_logging")]
    // ssl_logging_recv: *mut IO,
    // #[cfg(feature = "enable_ssl_logging")]
    // ssl_logging_send: *mut IO,
    // #[cfg(feature = "enable_ssl_logging")]
    // ssl_logging_lock: *mut Lock,
    h_accept_event: *mut std::ffi::c_void,

    is_rudp_socket: bool,
    bulk_send_tube: *mut c_void, // Tube,
    bulk_recv_tube: *mut c_void, // Tube,
    bulk_send_key: *mut Buffer,
    bulk_recv_key: *mut Buffer,
    rudp_optimized_mss: u32,

    #[cfg(target_os = "linux")]
    calling_thread: u64, // pthread_t

    #[cfg(target_os = "windows")]
    h_event: *mut std::ffi::c_void,

    // Rust Internal

    // Maybe a manual Default impl is necessary with adding in?
    // struct MyStruct {
    //     a: i32,
    //     b: String,
    //     c: CustomType, // CustomType does not implement Default
    // }

    // impl Default for MyStruct {
    //     fn default() -> Self {
    //         Self {
    //             c: CustomType::new(), // your custom default
    //             ..Default::default()   // a and b use their Default
    //         }
    //     }
    // }
    _socket: SslUpgradable<Socket>, // Should never be none, as it's initialized in the constructor
}

impl Sock {
    pub fn new(s: Socket) -> Self {
        Self {
            ref_count: null_mut(),
            lock: null_mut(),
            ssl_lock: null_mut(),
            disconnect_lock: null_mut(),
            socket: 0,
            ssl: null_mut(),
            sni_hostname: [0; 256],
            sock_type: 0,
            is_connected: false,
            is_server: false,
            is_async: false,
            is_ssl_secured: false,
            is_listening: false,
            send_buf: null_mut(),
            ip_client_added: false,
            only_local: false,
            enable_conditional_accept: false,
            remote: IP::from_ip(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
            local: IP::from_ip(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
            remote_hostname: null_mut(),
            remote_port: 0,
            local_port: 0,
            send_size: 0,
            recv_size: 0,
            send_count: 0,
            recv_count: 0,
            remote_cert: X::default(),
            local_cert: X::default(),
            cipher_name: null_mut(),
            wait_cipher_name: null_mut(),
            ignore_recv_errors: false,
            ignore_send_errors: false,
            timeout: 0,
            sock_event: null_mut(),
            cancel_accept: false,
            is_accept_canceled: false,
            write_blocked: false,
            no_need_to_read: false,
            disconnecting: false,
            udp_broadcast: false,
            param: null_mut(),
            ipv6: false,
            is_raw_socket: false,
            ssl_version: null_mut(),
            raw_socket_ip_protocol: 0,
            send_tube: null_mut(),
            recv_tube: null_mut(),
            in_proc_accept_queue: null_mut(),
            in_proc_accept_event: null_mut(),
            in_proc_recv_fifo: null_mut(),
            udp_max_msg_size: 0,
            current_tos: 0,
            is_ttl_supported: false,
            current_ttl: 0,
            r_udp_stack: null_mut(),
            underlay_protocol: [0; 64],
            protocol_details: [0; 256],
            reverse_accept_queue: null_mut(),
            reverse_accept_event: null_mut(),
            is_reverse_accepted_socket: false,
            reverse_my_server_global_ip: IP::from_ip(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
            reverse_my_server_port: 0,
            ssl_init_async_send_alert: [0; 2],
            raw_ip_header_include_flag: false,
            h_accept_event: null_mut(),
            is_rudp_socket: false,
            bulk_send_tube: null_mut(),
            bulk_recv_tube: null_mut(),
            bulk_send_key: null_mut(),
            bulk_recv_key: null_mut(),
            rudp_optimized_mss: 0,
            calling_thread: 0,
            _socket: SslUpgradable::new(s),
        }
    }
}

impl Read for Sock {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self._socket.read(buf)
    }
}

impl Write for Sock {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self._socket.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self._socket.flush()
    }

    // Provided methods
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize> {
        self._socket.write_vectored(bufs)
    }

    // fn is_write_vectored(&self) -> bool {  false }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self._socket.write_all(buf)
    }

    // fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> std::io::Result<()> {  Ok(())}

    fn write_fmt(&mut self, args: Arguments<'_>) -> std::io::Result<()> {
        self._socket.write_fmt(args)
    }

    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }
}

// pub fn write(&mut self, data: &mut [u8]) {
//     self._socket.write_all(data)
// }

// pub fn peek(&mut self, data: &mut [u8], size: u32) -> u32 {

// }

impl Sock {
    pub fn listen_tcp(addr: SocketAddr) -> Option<Self> {
        let listener = match TcpListener::bind(addr) {
            Ok(l) => l,
            Err(e) => {
                println!("Failed to create a TCP listener: {}", e);
                return None;
            }
        };

        let raw_socket = Socket::from(listener);
        let raw_fd = raw_socket.as_raw_fd();
        let mut s = Self::new(raw_socket);

        s.is_connected = false;
        s.is_async = false;
        s.is_server = true;
        s.sock_type = SockType::SockTcp as u32;
        s.socket = raw_fd; // Cedar should not use this directly,
        s.is_listening = true;
        s.is_ssl_secured = false;
        s.local_port = addr.port() as u32;

        // s.only_local:
        // s.enable_conditional_accept:

        Some(s)
    }

    pub fn connect_tcp(addr: SocketAddr) -> Option<Self> {
        let listener = match TcpStream::connect(addr) {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to create a TCP stream: {}", e);
                return None;
            }
        };

        let raw_socket = Socket::from(listener);

        Some(Self::new(raw_socket))
    }

    pub fn new_udp(addr: SocketAddr) -> Option<Self> {
        let socket = match UdpSocket::bind(addr) {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to create UDP socket for {}", addr);
                println!("Because of the following error: {}", e);
                return None;
            }
        };

        let raw_socket = Socket::from(socket);
        let raw_fd = raw_socket.as_raw_fd();

        let mut s = Sock::new(raw_socket);
        s.is_connected = false;
        s.is_async = false;
        s.is_server = true;
        s.sock_type = SockType::SockUdp as u32;
        s.socket = raw_fd;
        s.is_listening = true;
        s.is_ssl_secured = false;
        s.local_port = addr.port() as u32;

        // s.only_local:
        // s/enable_conditional_accept:

        Some(s)
    }

    // pub fn connect_udp(addr: SocketAddr) -> Option<Self> {
    //     let local_socket_addr =
    //     if addr.is_ipv4() {
    //         SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0))
    //     } else {
    //         SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0))
    //     };

    //     let socket = match UdpSocket::bind(local_socket_addr) {
    //         Ok(s) => s,
    //         Err(e) => {
    //             println!("Failed to create UDP socket for {}", addr);
    //             println!("Because of the following error: {}", e);
    //             return None;
    //         }
    //     };

    //     match socket.connect(addr) {
    //         Ok(_) => (),
    //         Err(e) => {
    //             println!("Failed to connect UDP port: {}", e);
    //             return None;
    //         }
    //     };

    //     let raw_socket = Socket::from(socket);

    //     Some (
    //         Self {
    //             is_connected: true,
    //             is_async: false,
    //             is_server: false,
    //             sock_type: SockType::SockTcp as u32,
    //             socket: raw_socket.as_raw_fd(),
    //             is_listening: false,
    //             is_ssl_secured: false,
    //             // local_port: addr.port() as u32, // Should we find out what this is? Is that even neccessary?

    //             // only_local:
    //             // enable_conditional_accept:
    //             _socket: Some(SslUpgradable::RawStream(raw_socket)), ..Default::default()
    //         }
    //     )
    // }

    // Connects by any means necessary, not just TCP
    pub fn connect(
        local_address: Option<SocketAddr>,
        remote_addresses: Vec<SocketAddr>,
        timeout: Duration,
    ) -> Option<Self> {
        if remote_addresses.len() < 1 {
            return None;
        }

        let should_use_nat = match local_address {
            Some(a) => !a.ip().is_global(),
            None => true,
        };

        let should_use_only_nat = false;

        let socket = if !should_use_nat {
            connect_method_tcp_simple(local_address.as_ref(), &remote_addresses[0])
        } else if should_use_only_nat {
            connect_method_rudp_and_tcp(&remote_addresses[0])
        } else {
            connect_method_all(local_address.as_ref(), remote_addresses)
        };

        let socket = match socket {
            Some(s) => s,
            None => {
                println!("Failed to create a socket");
                return None;
            }
        };
        let raw_fd = socket.as_raw_fd();

        let mut s = Self::new(socket);

        s.is_connected = true;
        s.is_async = false;
        s.is_server = false;
        // s.sock_type: SockType::SockTcp as u32,
        s.socket = raw_fd; // Cedar should not use this directly,
        s.is_listening = false;
        s.is_ssl_secured = false;
        s.local_port = match local_address {
            Some(a) => a.port() as u32,
            None => 0,
        };

        Some(s)
    }

    pub fn upgrade(&mut self, ssl: Ssl) {
        self._socket.upgrade(ssl);
    }

    pub fn as_mut_ptr(self) -> *mut Sock {
        Box::into_raw(Box::new(self))
    }

    pub fn free_mut_ptr(ptr: *mut Sock) {
        unsafe { drop(Box::from_raw(ptr)) }
    }
}

// The 4 connection strategies
pub fn connect_method_tcp_simple(
    local: Option<&SocketAddr>,
    remote: &SocketAddr,
) -> Option<Socket> {
    let local = match local {
        Some(s) => s,
        None => {
            if remote.is_ipv4() {
                &SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0))
            } else {
                &SocketAddr::from((Ipv6Addr::UNSPECIFIED, 0))
            }
        }
    };

    if (local.is_ipv4() && remote.is_ipv6()) || (local.is_ipv6() && remote.is_ipv4()) {
        println!("IP version mismatch, cannot connect between IPv4 and IPv6");
        return None;
    }

    let socket = if local.is_ipv4() {
        Socket::new(Domain::IPV4, Type::STREAM, None)
    } else {
        Socket::new(Domain::IPV6, Type::STREAM, None)
    };

    let socket = match socket {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to create a socket: {}", e);
            return None;
        }
    };

    if let Err(e) = socket.bind(&SockAddr::from(*local)) {
        println!("Failed to bind socket to IP address:{} ", e);
        return None;
    }

    if let Err(e) = socket.connect(&SockAddr::from(*remote)) {
        println!("Failed to connect to remote: {}", e);
        return None;
    }

    Some(socket)
}

pub fn connect_method_rudp_and_tcp(remote: &SocketAddr) -> Option<Socket> {
    todo!()
}

pub fn connect_method_dns(remote: &SocketAddr) -> Option<Socket> {
    todo!()
}

pub fn connect_method_icmp(remote: &SocketAddr) -> Option<Socket> {
    todo!()
}

pub fn connect_method_all(
    local_address: Option<&SocketAddr>,
    remote_addresses: Vec<SocketAddr>,
) -> Option<Socket> {
    for address in remote_addresses {
        thread::scope(|s| {
            let mut threads = Vec::new();
            threads.push(s.spawn(|| connect_method_tcp_simple(local_address, &address)));

            threads.push(s.spawn(|| connect_method_rudp_and_tcp(&address)));

            threads.push(s.spawn(|| connect_method_dns(&address)));

            threads.push(s.spawn(|| connect_method_icmp(&address)));

            while !threads.iter().all(|t| t.is_finished()) {
                thread::yield_now();
            }

            let result = threads.into_iter().find_map(|t| match t.join() {
                Ok(x) => match x {
                    Some(result) => Some(result),
                    None => None,
                },
                Err(_) => None,
            });

            result
        });
    }

    None
}

// SOCK *NewUDP(UINTport)

pub extern "C" fn NewUDP(port: u32) -> *mut Sock {
    NewUDPEx(port, false)
}

// SOCK *NewUDPEx(UINTport,boolipv6)
pub extern "C" fn NewUDPEx(port: u32, is_ipv6: bool) -> *mut Sock {
    NewUDPEx2(port, is_ipv6, null_mut())
}

// SOCK *NewUDPEx2(UINTport,boolipv6,IP*ip)
pub extern "C" fn NewUDPEx2(port: u32, is_ipv6: bool, ip: *mut IP) -> *mut Sock {
    if is_ipv6 {
        // TODO: NewUDP6()
        null_mut()
    } else {
        NewUDP4(port, ip)
    }
}

// SOCK *NewUDPEx3(UINTport,IP*ip)
pub extern "C" fn NewUDPEx3(port: u32, ip: *mut IP) -> *mut Sock {
    if ip.is_null() {
        return NewUDPEx2(port, false, null_mut());
    }

    let ip = unsafe { &mut *ip };
    return NewUDPEx2(port, ip.is_ipv4(), ip);
}

// TODO: Check if this actually connects to the remote, then use either connect_udp or listen_udp
// SOCK *NewUDP4(UINTport,IP*ip)
pub extern "C" fn NewUDP4(port: u32, ip: *mut IP) -> *mut Sock {
    nullcheck!(null_mut(), ip);
    // If port != 0 we're listening, otherwise we're connecting?

    let ip = unsafe { &mut *ip };
    let ip = match ip.to_ipv4() {
        None => {
            return null_mut();
        }
        Some(ip) => ip,
    };

    // Something about "special ports"? Ignore all port #s greater than 65535
    let port = if port > u16::MAX as u32 {
        0
    } else {
        port as u16
    };

    let socket_addr = SocketAddrV4::new(ip, port);

    match Sock::new_udp(SocketAddr::V4(socket_addr)) {
        Some(sock) => sock.as_mut_ptr(),
        None => null_mut(),
    }
}

// SOCK *NewUDP6(UINTport,IP*ip)
pub extern "C" fn NewUDP6(port: u32, ip: *mut IP) -> *mut Sock {
    let ip = unsafe { &mut *ip };
    let ip = match ip.to_ipv6() {
        None => {
            return null_mut();
        }
        Some(ip) => ip,
    };

    // Something about "special ports"? Ignore all port #s greater than 65535
    let port = if port > u16::MAX as u32 {
        0
    } else {
        port as u16
    };

    let socket_addr = SocketAddrV6::new(ip, port, 0, 0);

    match Sock::new_udp(SocketAddr::V6(socket_addr)) {
        Some(s) => s.as_mut_ptr(),
        None => null_mut(),
    }
}

// Listens TCP
// SOCK *Listen(UINTport)
// SOCK *ListenEx(UINTport,boollocal_only)
// SOCK *ListenEx2(UINTport,boollocal_only,boolenable_ca,IP*listen_ip)
pub extern "C" fn ListenEx2(
    port: u32,
    local_only: bool,
    enable_ca: bool,
    listen_ip: *mut IP,
) -> *mut Sock {
    nullcheck!(null_mut(), listen_ip);

    let listen_ip = unsafe { &mut *listen_ip };
    let listen_ip = match listen_ip.to_ipv4() {
        None => {
            return null_mut();
        }
        Some(ip) => ip,
    };

    if port == 0 || port > 65535 {
        return null_mut();
    }

    let addr = SocketAddrV4::new(listen_ip, port as u16);

    match Sock::listen_tcp(SocketAddr::V4(addr)) {
        Some(s) => s.as_mut_ptr(),
        None => null_mut(),
    }
}

// SOCK *ListenEx6(UINTport,boollocal_only)
// SOCK *ListenEx63(UINTport,boollocal_only,boolenable_ca,IP*listen_ip)

// Creates and connects a TCP socket -- Equivalent of using TcpListener::bind

// SOCK *Connect(char*hostname,UINTport)
// SOCK *ConnectEx(char*hostname,UINTport,UINTtimeout)
// SOCK *ConnectEx2(char*hostname,UINTport,UINTtimeout,bool*cancel_flag)
// SOCK *ConnectEx3(char*hostname,UINTport,UINTtimeout,bool*cancel_flag,char*nat_t_svc_name,UINT*nat_t_error_code,booltry_start_ssl,boolno_get_hostname)
// SOCK *ConnectEx4(char*hostname,UINTport,UINTtimeout,bool*cancel_flag,char*nat_t_svc_name,UINT*nat_t_error_code,booltry_start_ssl,boolno_get_hostname,IP*ret_ip)
pub extern "C" fn ConnectEx4(
    hostname: *mut c_char,
    port: u32,
    timeout: u32,
    should_cancel: *mut bool,
    nat_table_svc_name: *mut c_char,
    nat_t_error_code: *mut u32,
    try_ssl: bool,
    no_get_hostname: bool,
) -> *mut Sock {
    nullcheck!(null_mut(), hostname);

    let hostname = unsafe {
        match CStr::from_ptr(hostname).to_str() {
            Ok(s) => s,
            Err(_) => return null_mut(),
        }
    };

    // TODO: Resolve hostname to IP
    // let raw_socket = match TcpStream::connect(format!("{}:{}", hostname, port)) {
    //     Ok(s) => s,
    //     Err(e) => {
    //         println!("Failed to build a TCP stream using TcpStream::connect");
    //         return null_mut();
    //     }
    // };

    // match Sock::listen_tcp(format!("{}:{}", hostname, port)) {
    //     Some(s) => s.as_mut_ptr(),
    //     None => null_mut()
    // }
    null_mut()
}

// SOCK *BindConnectEx5(IP*localIP,UINTlocalport,char*hostname,UINTport,UINTtimeout,bool*cancel_flag,char*nat_t_svc_name,UINT*nat_t_error_code,booltry_start_ssl,boolno_get_hostname,SSL_VERIFY_OPTION*ssl_option,UINT*ssl_err,char*hint_str,IP*ret_ip)
pub extern "C" fn BindConnectEx5(
    local_ip: *mut IP,
    local_port: u32,
    hostname: *mut c_char,
    remote_port: u32,
    timeout: u32,
    should_cancel: *mut bool,
    nat_table_svc_name: *mut c_char,
    nat_t_error_code: *mut u32,
    try_ssl: bool,
    no_get_hostname: bool,
    ssl_option: *mut SslVerifyOption,
    ssl_err: *mut u32,
    hint_str: *mut c_char,
    ret_ip: *mut IP,
) -> *mut Sock {
    // We're going to ignore all the string passed in for now, because it seems like it doesn't matter much anyways

    nullcheck!(null_mut(), hostname);

    let hostname = unsafe {
        match CStr::from_ptr(hostname).to_str() {
            Ok(s) => s,
            Err(_) => return null_mut(),
        }
    };

    let local_ip = if local_ip.is_null() {
        None
    } else {
        unsafe { Some(&mut *local_ip) }
    };

    let local_ip = local_ip.map(|ip| ip.to_ip());
    let local_address = local_ip.map(|ip| SocketAddr::from((ip, local_port as u16)));

    let remote_ips = match resolve_all(hostname) {
        Some(s) => s,
        None => {
            println!("Failed to resolve ip for {}", hostname);
            return null_mut();
        }
    };

    let remote_addresses = remote_ips
        .into_iter()
        .map(|ip| SocketAddr::from((ip, remote_port as u16)))
        .collect();

    match Sock::connect(local_address, remote_addresses, Duration::from_millis(500)) {
        Some(s) => s.as_mut_ptr(),
        None => null_mut(),
    }
}

// SOCK *Accept(SOCK*sock)

// void AcceptInit(SOCK*s)
// void AcceptInitEx(SOCK*s,boolno_lookup_hostname)

// bool StartSSL(SOCK *sock, X *x, K *priv);
pub extern "C" fn StartSSL(sock: *mut Sock, cert: *mut X, priv_key: *mut K) -> bool {
    StartSSLEx(sock, cert, priv_key, 0, null_mut())
}

// bool StartSSLEx(SOCK *sock, X *x, K *priv, UINT ssl_timeout, char *sni_hostname);
pub extern "C" fn StartSSLEx(
    sock: *mut Sock,
    cert: *mut X,
    priv_key: *mut K,
    timeout: u32,
    sni_hostname: *mut c_char,
) -> bool {
    StartSSLEx3(
        sock,
        cert,
        priv_key,
        null_mut(),
        timeout,
        sni_hostname,
        null_mut(),
        null_mut(),
    )
}

// bool StartSSLEx3(SOCK *sock, X *x, K *priv, LIST *chain, UINT ssl_timeout, char *sni_hostname, SSL_VERIFY_OPTION *ssl_option, UINT *ssl_err);
pub extern "C" fn StartSSLEx3(
    sock: *mut Sock,
    cert: *mut X,
    priv_key: *mut K,
    chain: *mut List<u8>,
    timeout: u32,
    sni_hostname: *mut c_char,
    verify_options: *mut SslVerifyOption,
    err: *mut u32,
) -> bool {
    nullcheck!(false, sock, cert, priv_key);

    let sock = unsafe { &mut *sock };

    // First figure out what this is trying to do before uncommmenting
    // if (sock.is_connected && sock.sock_type == 3 && !sock.is_listening) {
    //     sock.is_ssl_secured = true;
    //     return true;
    // }

    // Maybe only client-connections are allowed to upgrade ssl be made
    // if !sock.is_connected || sock.is_listening {
    //     return false;
    // }

    let ssl = match Ssl::new(if sock.is_server {
        &SSL_CTX_SERVER
    } else {
        &SSL_CTX_CLIENT
    }) {
        Ok(s) => s,
        Err(_) => {
            println!("Failed to upgrade SSL");
            return false;
        }
    };

    // Some SslContext options that are set in the original
    // --  server  --
    // AddChainSslCertOnDirectory

    // --  client  -- Adding a default trust store + any intermediate certificates
    // SSL_CTX_set_default_verify_paths
    // X509_STORE_add_cert
    // X509_VERIFY_PARAM_set_flags
    // X509_VERIFY_PARAM_set1_host
    // SSL_CTX_set_security_level
    // SSL_set_tlsext_host_name

    // SSL_use_certificate
    // SSL_use_PrivateKey
    // SSL_set_cipher_list
    // SSL_set1_groups_list

    sock.upgrade(ssl); // make `upgrade` take arguments for certs, ssl settings, etc

    return true;
}

// UINT Send(SOCK*sock,void*data,UINTsize,boolsecure)
// UINT SendTo(SOCK*sock,IP*dest_addr,UINTdest_port,void*data,UINTsize)
// bool SendAll(SOCK*sock,void*data,UINTsize,boolsecure)
// void SendAdd(SOCK*sock,void*data,UINTsize)
// bool SendNow(SOCK*sock,intsecure)
// bool SendPack(SOCK*s,PACK*p)
// bool SendPackWithHash(SOCK*s,PACK*p)
pub extern "C" fn Send(sock: *mut Sock, data: *mut u8, size: u32, secure: bool) -> u32 {
    nullcheck!(0, sock, data);

    let sock = unsafe { &mut *sock };
    let data = unsafe { slice::from_raw_parts_mut(data, size as usize) };

    match sock.write(data) {
        Ok(n) => n as u32,
        Err(e) => {
            println!("Failed to write to socket: {}", e);
            0
        }
    }
}

// UINT Recv(SOCK*sock,void*data,UINTsize,boolsecure)
// UINT RecvFrom(SOCK*sock,IP*src_addr,UINT*src_port,void*data,UINTsize)
// bool RecvAllWithDiscard(SOCK*sock,UINTsize,boolsecure)
// bool RecvAll(SOCK*sock,void*data,UINTsize,boolsecure)
// PACK *RecvPack(SOCK*s)
// PACK *RecvPackWithHash(SOCK*s)
pub extern "C" fn Recv(sock: *mut Sock, data: *mut u8, size: u32, secure: bool) -> u32 {
    nullcheck!(0, sock, data);

    let sock = unsafe { &mut *sock };
    let data = unsafe { slice::from_raw_parts_mut(data, size as usize) };

    match sock.read(data) {
        Ok(n) => n as u32,
        Err(e) => {
            println!("Failed to read from socket: {}", e);
            0
        }
    }
}

// UINT Peek(SOCK*sock,void*data,UINTsize)
pub extern "C" fn Peek(sock: *mut Sock, data: *mut u8, size: u32) -> u32 {
    nullcheck!(0, sock, data);

    let sock = unsafe { &mut *sock };
    let data = unsafe { slice::from_raw_parts_mut(data, size as usize) };

    0
}

// void Disconnect(SOCK*sock)
// void ReleaseSock(SOCK*s)
