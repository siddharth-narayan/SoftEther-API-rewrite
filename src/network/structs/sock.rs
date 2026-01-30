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

use crate::nullcheck;
use crate::{mem::structs::list::List, network::ssl::create_client_ctx};
use crate::network::ssl::{SSL_CTX_CLIENT, SSL_CTX_SERVER, SslUpgradable, SslVerifyOption};
use std::ffi::CStr;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, SocketAddrV6, TcpListener, ToSocketAddrs};
use std::{default, ffi::{c_char, c_void}, net::{SocketAddr, TcpStream, UdpSocket}, ptr::{null, null_mut}};

use openssl::ssl::{Ssl, SslStream};
use socket2::{Socket};

use crate::{mem::structs::{buf::Buffer, fifo::Fifo, queue::Queue}, network::{util::IP, structs::cert::{X, K}}, object::{Lock, RefCounter}};

#[derive(Default)]
pub struct Sock {
    ref_count: *mut RefCounter,
    lock: *mut Lock,
    ssl_lock: *mut Lock,
    disconnect_lock: *mut Lock,

    socket: i32,
    // TODO implement this with openssl crate -- ssl: *mut Ssl
    ssl: *mut Ssl,
    sni_hostname: [u8; 256] = [0; 256],

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
    sock_event:*mut c_void, // SockEvent,

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
    
    send_tube:*mut c_void, // Tube,
    recv_tube: *mut c_void, // Tube,
    in_proc_accept_queue: *mut Queue<*mut c_void>,
    in_proc_accept_event: *mut c_void, // Event,
    in_proc_recv_fifo: *mut Fifo<*mut c_void>,
    
    udp_max_msg_size: u32,
    current_tos: i32,
    is_ttl_supported: bool,
    current_ttl: u32,
    r_udp_stack: *mut c_void, //RudpStack,
    
    
    underlay_protocol: [u8; 64] = [0; 64],
    protocol_details: [u8; 256] = [0; 256],
    
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
    bulk_send_tube:*mut c_void, // Tube,
    bulk_recv_tube:*mut c_void, // Tube,
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
    _socket: Option<SslUpgradable<Socket>>, // Should never be none, as it's initialized in the constructor
}

impl Sock {
    // Simply creates a socket and 
    pub fn new_udp(addr: SocketAddr) -> Option<Self> {
        let raw_socket = match UdpSocket::bind(addr) {
            Ok(s) => s,
            Err(e) => { 
                println!("Failed to create UDP socket for {}", addr);
                println!("Because of the following error: {}", e);
                return None; 
            }
        };

        let socket = Socket::from(raw_socket);
        Some(Self { _socket: Some(SslUpgradable::RawStream(socket)), ..Default::default() })
    }

    pub fn new_tcp<A: ToSocketAddrs>(addr: A) -> Option<Self> {
        let addr = match addr.to_socket_addrs() {
            Ok(t) => {
                let collect = t.collect::<Vec<_>>();
                if collect.len() == 0 {
                    return None;
                }

                collect
            },
            
            Err(e) => {
                println!("Failed to convert address to SocketAddr");
                return None;
            },
        };

        let raw_socket = match TcpListener::bind(addr[0]) {
            Ok(s) => s,
            Err(e) => { 
                println!("Failed to create TCP socket for {}", addr[0]);
                println!("Because of the following error: {}", e);
                return None; 
            }
        };

        let socket = Socket::from(raw_socket);

        Some (
            Self {
                is_connected: false,
                is_async: false,
                is_server: true,
                sock_type: SockType::SockTcp as u32,
                socket: -1, // Cedar should not use this directly,
                is_listening: true,
                is_ssl_secured: false,
                local_port: addr[0].port() as u32,
                
                // only_local:
                // enable_conditional_accept: 
                _socket: Some(SslUpgradable::RawStream(Socket::from(socket))), ..Default::default() 
            }
        )
    }

    pub fn upgrade(&mut self, ssl: Ssl) {
        let inner = match self._socket.take() {
            Some(s) => s,
            None => {
                println!("Socket for {} should never be None", self.local_port);
                return;
            }
        };

        self._socket = Some(inner.upgrade(ssl))
    }

    pub fn as_mut_ptr(self) -> *mut Sock {
        Box::into_raw(Box::new(self))
    }

    pub fn free_mut_ptr(ptr: *mut Sock) {
        unsafe { drop(Box::from_raw(ptr)) }
    }

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
    return NewUDPEx2(port, ip.is_ipv4(), ip)
}

// SOCK *NewUDP4(UINTport,IP*ip)
pub extern "C" fn NewUDP4(port: u32, ip: *mut IP) -> *mut Sock {
    // If port != 0 we're listening, otherwise we're connecting?

    let ip = unsafe { &mut *ip };
    let ip = match ip.to_ipv4() {
        None => { return null_mut(); },
        Some(ip) => ip
    };

    // Something about "special ports"? Ignore all port #s greater than 65535
    let port = if port > u16::MAX as u32 { 0 } else { port as u16 }; 
    
    let socket_addr = SocketAddrV4::new(ip, port);
    
    match Sock::new_udp(SocketAddr::V4(socket_addr)) {
        Some(sock) => sock.as_mut_ptr(),
        None => null_mut()
    }
}

// SOCK *NewUDP6(UINTport,IP*ip)
pub extern "C" fn NewUDP6(port: u32, ip: *mut IP) -> *mut Sock {
    let ip = unsafe { &mut *ip };
       let ip = match ip.to_ipv6() {
        None => { return null_mut(); },
        Some(ip) => ip
    };

    // Something about "special ports"? Ignore all port #s greater than 65535
    let port = if port > u16::MAX as u32 { 0 } else { port as u16 }; 
    
    let socket_addr = SocketAddrV6::new(ip, port, 0, 0);
    
    match Sock::new_udp(SocketAddr::V6(socket_addr)) {
        Some(s) => s.as_mut_ptr(),
        None => null_mut()
    }
}



// Listens TCP
// SOCK *Listen(UINTport)
// SOCK *ListenEx(UINTport,boollocal_only)
// SOCK *ListenEx2(UINTport,boollocal_only,boolenable_ca,IP*listen_ip)
pub extern "C" fn ListenEx2(port: u32, local_only: bool, enable_ca: bool, listen_ip: *mut IP) -> *mut Sock {
    nullcheck!(null_mut(), listen_ip);
   
    let listen_ip = unsafe { &mut *listen_ip };
    let listen_ip = match listen_ip.to_ipv4() {
        None => { return null_mut(); },
        Some(ip) => ip
    };
    
    if port == 0 || port > 65535 {
        return null_mut();
    }

    let addr = SocketAddrV4::new(listen_ip, port as u16);

    match Sock::new_tcp(SocketAddr::V4(addr)) {
        Some(s) => s.as_mut_ptr(),
        None => null_mut()
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
pub extern "C" fn ConnectEx4(hostname: *mut c_char, port: u32, timeout: u32, should_cancel: *mut bool, nat_table_svc_name: *mut c_char, nat_t_error_code: *mut u32, try_ssl: bool, no_get_hostname: bool) -> *mut Sock {
    nullcheck!(null_mut(), hostname);

    let hostname = unsafe { match CStr::from_ptr(hostname).to_str() {
        Ok(s) => s,
        Err(_) => { return null_mut() }
    } };
    
    // TODO: Resolve hostname to IP 
    // let raw_socket = match TcpStream::connect(format!("{}:{}", hostname, port)) {
    //     Ok(s) => s,
    //     Err(e) => {
    //         println!("Failed to build a TCP stream using TcpStream::connect");
    //         return null_mut();
    //     }
    // };

    match Sock::new_tcp(format!("{}:{}", hostname, port)) {
        Some(s) => s.as_mut_ptr(),
        None => null_mut()
    }
}

// SOCK *BindConnectEx5(IP*localIP,UINTlocalport,char*hostname,UINTport,UINTtimeout,bool*cancel_flag,char*nat_t_svc_name,UINT*nat_t_error_code,booltry_start_ssl,boolno_get_hostname,SSL_VERIFY_OPTION*ssl_option,UINT*ssl_err,char*hint_str,IP*ret_ip)
pub extern "C" fn BindConnectEx5(local_ip: *mut IP, local_port: u32, hostname: *mut c_char, remote_port: u32, timeout: u32, should_cancel: *mut bool, nat_table_svc_name: *mut c_char, nat_t_error_code: *mut u32, try_ssl: bool, no_get_hostname: bool, ssl_option: *mut SslVerifyOption, ssl_err: *mut u32, hint_str: *mut c_char, ret_ip: *mut IP) -> *mut Sock {
    nullcheck!(null_mut(), hostname);

    let local_ip = unsafe { &mut *local_ip };
    let hostname = unsafe { match CStr::from_ptr(hostname).to_str() {
        Ok(s) => s,
        Err(_) => { return null_mut() }
    } };

    // We're going to ignore all the string passed in for now, because it seems like it doesn't matter much anyways

    // TODO: It looks like this is supposed to keep a list of IP addresses that the host can read from, including the one passed as an argument.
    // For now just use the single IP.

    // There's some more complex logic behind whether NAT is chosen, but I don't think it's that important
    let should_use_nat = !local_ip.is_local();
    let should_use_only_nat = false;

    // 4 different modes of trying to connect -- TCP, RUDP, DNS, ICMP -- And also try IPv6 versions of each


    return null_mut()
}

// SOCK *Accept(SOCK*sock)

// void AcceptInit(SOCK*s)
// void AcceptInitEx(SOCK*s,boolno_lookup_hostname)


// bool StartSSL(SOCK *sock, X *x, K *priv);
pub extern "C" fn StartSSL(sock: *mut Sock, cert: *mut X, priv_key: *mut K) -> bool {
    StartSSLEx(sock, cert, priv_key, 0, null_mut())
}

// bool StartSSLEx(SOCK *sock, X *x, K *priv, UINT ssl_timeout, char *sni_hostname);
pub extern "C" fn StartSSLEx(sock: *mut Sock, cert: *mut X, priv_key: *mut K, timeout: u32, sni_hostname: *mut c_char) -> bool {
    StartSSLEx3(sock, cert, priv_key, null_mut(), timeout, sni_hostname, null_mut(), null_mut())
}

// bool StartSSLEx3(SOCK *sock, X *x, K *priv, LIST *chain, UINT ssl_timeout, char *sni_hostname, SSL_VERIFY_OPTION *ssl_option, UINT *ssl_err);
pub extern "C" fn StartSSLEx3(sock: *mut Sock, cert: *mut X, priv_key: *mut K, chain: *mut List<u8>, timeout: u32, sni_hostname: *mut c_char, verify_options: *mut SslVerifyOption, err: *mut u32) -> bool {
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

    let ssl = match Ssl::new(if sock.is_server { &SSL_CTX_SERVER } else { &SSL_CTX_CLIENT }) {
        Ok(s) => s,
        Err(_) => return false,
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

// UINT Recv(SOCK*sock,void*data,UINTsize,boolsecure)
// UINT RecvFrom(SOCK*sock,IP*src_addr,UINT*src_port,void*data,UINTsize)
// bool RecvAllWithDiscard(SOCK*sock,UINTsize,boolsecure)
// bool RecvAll(SOCK*sock,void*data,UINTsize,boolsecure)
// PACK *RecvPack(SOCK*s)
// PACK *RecvPackWithHash(SOCK*s)

// UINT Peek(SOCK*sock,void*data,UINTsize)



// void Disconnect(SOCK*sock)
// void ReleaseSock(SOCK*s)