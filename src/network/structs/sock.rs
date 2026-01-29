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
use std::{default, ffi::{c_char, c_void}, net::{SocketAddr, TcpStream, UdpSocket}, ptr::{null, null_mut}};

use openssl::ssl::{Ssl, SslStream};
use socket2::Socket;

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
    _socket: Option<SslUpgradable<Socket>>,
}

impl Sock {
    // Simply creates a socket and 
    // pub fn new_udp(addr: SocketAddr) -> Self {
    //     let mut socket = Self::default();

    //     socket._socket
    // }

    pub fn new_tcp() {

    }

}


// SOCK *NewUDP(UINTport)
// SOCK *NewUDPEx(UINTport,boolipv6)
// SOCK *NewUDPEx2(UINTport,boolipv6,IP*ip)
// SOCK *NewUDPEx3(UINTport,IP*ip)
// SOCK *NewUDP4(UINTport,IP*ip)

// SOCK *Listen(UINTport)
// SOCK *ListenEx(UINTport,boollocal_only)
// SOCK *ListenEx2(UINTport,boollocal_only,boolenable_ca,IP*listen_ip)
// SOCK *ListenEx6(UINTport,boollocal_only)
// SOCK *ListenEx63(UINTport,boollocal_only,boolenable_ca,IP*listen_ip)

// Creates and connects a TCP socket -- Equivalent of using TcpListener::bind

//     sock._socket = Some(SslUpgradable::RawStream(Socket::from(UdpSocket::bind("127.0.0.1:992").unwrap())));

// SOCK *Connect(char*hostname,UINTport)
// SOCK *ConnectEx(char*hostname,UINTport,UINTtimeout)
// SOCK *ConnectEx2(char*hostname,UINTport,UINTtimeout,bool*cancel_flag)
// SOCK *ConnectEx3(char*hostname,UINTport,UINTtimeout,bool*cancel_flag,char*nat_t_svc_name,UINT*nat_t_error_code,booltry_start_ssl,boolno_get_hostname)
// SOCK *ConnectEx4(char*hostname,UINTport,UINTtimeout,bool*cancel_flag,char*nat_t_svc_name,UINT*nat_t_error_code,booltry_start_ssl,boolno_get_hostname,IP*ret_ip)
// SOCK *BindConnectEx5(IP*localIP,UINTlocalport,char*hostname,UINTport,UINTtimeout,bool*cancel_flag,char*nat_t_svc_name,UINT*nat_t_error_code,booltry_start_ssl,boolno_get_hostname,SSL_VERIFY_OPTION*ssl_option,UINT*ssl_err,char*hint_str,IP*ret_ip)

// SOCK *Accept(SOCK*sock)
// void AcceptInit(SOCK*s)
// void AcceptInitEx(SOCK*s,boolno_lookup_hostname)

// bool StartSSL(SOCK*sock,X*x,K*priv)
// bool StartSSLEx(SOCK*sock,X*x,K*priv,UINTssl_timeout,char*sni_hostname)
// bool StartSSLEx3(SOCK*sock,X*x,K*priv,LIST*chain,UINTssl_timeout,char*sni_hostname,SSL_VERIFY_OPTION*ssl_option,UINT*ssl_err)
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

    if sock._socket.is_none() {
        return false;
    }


    let ssl = match Ssl::new(if sock.is_server { &SSL_CTX_SERVER } else { &SSL_CTX_CLIENT }) {
        Ok(s) => s,
        Err(_) => return false,
    };


    let socket = match sock._socket.take() {
        Some(s) => s,
        None => return false,
    };

    sock._socket = Some(socket.upgrade(ssl));


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