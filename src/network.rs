
// int GetCurrentTimezone();

// bool GetSniNameFromSslPacket(UCHAR *packet_buf, UINT packet_size, char *sni, UINT sni_size);

// void SetDhParam(DH_CTX *dh);

// bool IsUseAlternativeHostname();

// #ifdef	OS_WIN32
// int GetCurrentTimezoneWin32();
// #endif	// OS_WIN32

// bool SendPack(SOCK *s, PACK *p);
// PACK *RecvPack(SOCK *s);
// PACK *RecvPackWithHash(SOCK *s);
// bool SendPackWithHash(SOCK *s, PACK *p);

// UINT GetErrorFromPack(PACK *p);
// PACK *PackError(UINT error);

// void CreateDummyValue(PACK *p);

// char *RecvLine(SOCK *s, UINT max_size);

// bool GetIPViaDnsProxyForJapanFlets(IP *ip_ret, char *hostname, bool ipv6, UINT timeout, bool *cancel, char *dns_proxy_hostname);
// bool GetDnsProxyIPAddressForJapanBFlets(IP *ip_ret, UINT timeout, bool *cancel);
// BUF *QueryFileByUdpForJapanBFlets(UINT timeout, bool *cancel);
// BUF *QueryFileByIPv6Udp(LIST *ip_list, UINT timeout, bool *cancel);
// UINT DetectFletsType();

// void ListenTcpForPopupFirewallDialog();

// bool DetectIsServerSoftEtherVPN(SOCK *s);
// void ConnectThreadForTcp(THREAD *thread, void *param);
// void ConnectThreadForRUDP(THREAD *thread, void *param);
// void ConnectThreadForOverDnsOrIcmp(THREAD *thread, void *param);
// void ConnectThreadForIPv4(THREAD *thread, void *param);
// void ConnectThreadForIPv6(THREAD *thread, void *param);

// void BindConnectThreadForIPv4(THREAD *thread, void *param);
// void BindConnectThreadForIPv6(THREAD *thread, void *param);

// SOCK *CreateTCPSock(SOCKET s, bool is_ipv6, IP *current_ip, bool no_get_hostname, char *hostname_original);
// SOCK *NewRUDPClientNatT(char *svc_name, IP *ip, UINT *error_code, UINT timeout, bool *cancel, char *hint_str, char *target_hostname);
// RUDP_STACK *NewRUDPServer(char *svc_name, RUDP_STACK_INTERRUPTS_PROC *proc_interrupts, RUDP_STACK_RPC_RECV_PROC *proc_rpc_recv, void *param, UINT port, bool no_natt_register, bool over_dns_mode, volatile UINT *natt_global_udp_port, UCHAR rand_port_id, IP *listen_ip);
// SOCK *NewRUDPClientDirect(char *svc_name, IP *ip, UINT port, UINT *error_code, UINT timeout, bool *cancel, SOCK *sock, SOCK_EVENT *sock_event, UINT local_port, bool over_dns_mode);
// RUDP_STACK *NewRUDP(bool server_mode, char *svc_name, RUDP_STACK_INTERRUPTS_PROC *proc_interrupts, RUDP_STACK_RPC_RECV_PROC *proc_rpc_recv, void *param, UINT port, SOCK *sock, SOCK_EVENT *sock_event, bool server_no_natt_register, bool over_dns_mode, IP *client_target_ip, volatile UINT *natt_global_udp_port, UCHAR rand_port_id, IP *listen_ip);
// void FreeRUDP(RUDP_STACK *r);
// void RUDPMainThread(THREAD *thread, void *param);
// void RUDPRecvProc(RUDP_STACK *r, UDPPACKET *p);
// void RUDPInterruptProc(RUDP_STACK *r);
// void RUDPIpQueryThread(THREAD *thread, void *param);
// void RUDPSendPacket(RUDP_STACK *r, IP *dest_ip, UINT dest_port, void *data, UINT size, UINT icmp_type);
// void GetCurrentMachineIpProcessHash(void *hash);
// void GetCurrentMachineIpProcessHashInternal(void *hash);
// int RUDPCompareSessionList(void *p1, void *p2);
// RUDP_SESSION *RUDPNewSession(bool server_mode, IP *my_ip, UINT my_port, IP *your_ip, UINT your_port, UCHAR *init_key);
// void RUDPFreeSession(RUDP_SESSION *se);
// int RUDPCompareSegmentList(void *p1, void *p2);
// RUDP_SESSION *RUDPSearchSession(RUDP_STACK *r, IP *my_ip, UINT my_port, IP *your_ip, UINT your_port);
// void RUDPSendSegmentNow(RUDP_STACK *r, RUDP_SESSION *se, UINT64 seq_no, void *data, UINT size);
// void RUDPSendSegment(RUDP_STACK *r, RUDP_SESSION *se, void *data, UINT size);
// bool RUDPProcessRecvPacket(RUDP_STACK *r, RUDP_SESSION *se, void *recv_data, UINT recv_size);
// bool RUDPCheckSignOfRecvPacket(RUDP_STACK *r, RUDP_SESSION *se, void *recv_data, UINT recv_size);
// void RUDPProcessAck(RUDP_STACK *r, RUDP_SESSION *se, UINT64 seq);
// void RUDPProcessAck2(RUDP_STACK *r, RUDP_SESSION *se, UINT64 max_seq);
// void RUDPProcessRecvPayload(RUDP_STACK *r, RUDP_SESSION *se, UINT64 seq, void *payload_data, UINT payload_size);
// void RUDPInitSock(RUDP_STACK *r, RUDP_SESSION *se);
// void RUDPDisconnectSession(RUDP_STACK *r, RUDP_SESSION *se, bool disconnected_by_you);
// UINT64 RUDPGetCurrentSendingMinSeqNo(RUDP_SESSION *se);
// UINT64 RUDPGetCurrentSendingMaxSeqNo(RUDP_SESSION *se);
// SOCK *ListenRUDP(char *svc_name, RUDP_STACK_INTERRUPTS_PROC *proc_interrupts, RUDP_STACK_RPC_RECV_PROC *proc_rpc_recv, void *param, UINT port, bool no_natt_register, bool over_dns_mode);
// SOCK *ListenRUDPEx(char *svc_name, RUDP_STACK_INTERRUPTS_PROC *proc_interrupts, RUDP_STACK_RPC_RECV_PROC *proc_rpc_recv, void *param, UINT port, bool no_natt_register, bool over_dns_mode,
//                    volatile UINT *natt_global_udp_port, UCHAR rand_port_id, IP *listen_ip);
// SOCK *AcceptRUDP(SOCK *s);
// void *InitWaitUntilHostIPAddressChanged();
// void FreeWaitUntilHostIPAddressChanged(void *p);
// void WaitUntilHostIPAddressChanged(void *p, EVENT *event, UINT timeout, UINT ip_check_interval);
// UINT GetHostIPAddressHash32();
// bool GetMyPrivateIP(IP *ip, bool from_vg);
// char *GetRandHostNameForGetMyPrivateIP();
// UINT GenRandInterval(UINT min, UINT max);
// void RUDPProcess_NatT_Recv(RUDP_STACK *r, UDPPACKET *udp);
// void RUDPDo_NatT_Interrupt(RUDP_STACK *r);
// void RUDPGetRegisterHostNameByIP(char *dst, UINT size, IP *ip);
// bool RUDPParseIPAndPortStr(void *data, UINT data_size, IP *ip, UINT *port);
// void ParseNtUsername(char *src_username, char *dst_username, UINT dst_username_size, char *dst_domain, UINT dst_domain_size, bool do_not_parse_atmark);
// void RUDPBulkSend(RUDP_STACK *r, RUDP_SESSION *se, void *data, UINT data_size);
// bool RUDPProcessBulkRecvPacket(RUDP_STACK *r, RUDP_SESSION *se, void *recv_data, UINT recv_size);
// UINT RUDPCalcBestMssForBulk(RUDP_STACK *r, RUDP_SESSION *se);
// bool IsIPLocalHostOrMySelf(IP *ip);
// bool RUDPIsIpInValidateList(RUDP_STACK *r, IP *ip);
// void RUDPAddIpToValidateList(RUDP_STACK *r, IP *ip);

// bool GetBestLocalIpForTarget(IP *local_ip, IP *target_ip);
// SOCK *NewUDP4ForSpecificIp(IP *target_ip, UINT port);

// #ifdef OS_WIN32

// // Function prototype for Win32
// void Win32InitSocketLibrary();
// void Win32FreeSocketLibrary();
// void Win32Select(SOCKSET *set, UINT timeout, CANCEL *c1, CANCEL *c2);
// void Win32InitAsyncSocket(SOCK *sock);
// void Win32JoinSockToSockEvent(SOCK *sock, SOCK_EVENT *event);
// void Win32FreeAsyncSocket(SOCK *sock);
// void Win32IpForwardRow2ToRouteEntry(ROUTE_ENTRY *entry, void *ip_forward_row);
// void Win32RouteEntryToIpForwardRow2(void *ip_forward_row, ROUTE_ENTRY *entry);
// int Win32CompareRouteEntryByMetric(void *p1, void *p2);
// ROUTE_TABLE *Win32GetRouteTable2(bool ipv4, bool ipv6);
// bool Win32AddRouteEntry2(ROUTE_ENTRY *e, bool *already_exists);
// void Win32DeleteRouteEntry2(ROUTE_ENTRY *e);
// UINT Win32GetVLanInterfaceID(char *instance_name);
// char **Win32EnumVLan(char *tag_name);
// void Win32Cancel(CANCEL *c);
// void Win32CleanupCancel(CANCEL *c);
// CANCEL *Win32NewCancel();
// SOCK_EVENT *Win32NewSockEvent();
// void Win32SetSockEvent(SOCK_EVENT *event);
// void Win32CleanupSockEvent(SOCK_EVENT *event);
// bool Win32WaitSockEvent(SOCK_EVENT *event, UINT timeout);
// bool Win32GetDefaultDns(IP *ip, char *domain, UINT size);
// bool Win32GetDnsSuffix(char *domain, UINT size);
// void Win32FlushDnsCache();
// int CompareIpAdapterIndexMap(void *p1, void *p2);
// ROUTE_CHANGE *Win32NewRouteChange2(bool ipv4, bool ipv6, void *callback);
// void Win32FreeRouteChange2(ROUTE_CHANGE *r);
// bool Win32IsRouteChanged2(ROUTE_CHANGE *r);
// bool Win32GetAdapterFromGuid(void *a, char *guid);
// SOCKET Win32Accept(SOCK *sock, SOCKET s, struct sockaddr *addr, int *addrlen, bool ipv6);

// bool Win32ReleaseAddress(void *a);
// bool Win32ReleaseAddressByGuid(char *guid);
// bool Win32ReleaseAddressByGuidEx(char *guid, UINT timeout);
// void Win32ReleaseAddressByGuidExThread(THREAD *t, void *param);
// void ReleaseWin32ReleaseAddressByGuidThreadParam(WIN32_RELEASEADDRESS_THREAD_PARAM *p);
// bool Win32ReleaseOrRenewAddressByGuidEx(char *guid, UINT timeout, bool renew);
// bool Win32RenewAddress(void *a);
// bool Win32RenewAddressByGuid(char *guid);
// bool Win32RenewAddressByGuidEx(char *guid, UINT timeout);


// #else	// OS_WIN32

// // Function prototype for UNIX
// void UnixInitSocketLibrary();
// void UnixFreeSocketLibrary();
// void UnixSelect(SOCKSET *set, UINT timeout, CANCEL *c1, CANCEL *c2);
// void UnixInitAsyncSocket(SOCK *sock);
// void UnixJoinSockToSockEvent(SOCK *sock, SOCK_EVENT *event);
// void UnixFreeAsyncSocket(SOCK *sock);
// ROUTE_TABLE *UnixGetRouteTable();
// bool UnixAddRouteEntry(ROUTE_ENTRY *e, bool *already_exists);
// void UnixDeleteRouteEntry(ROUTE_ENTRY *e);
// UINT UnixGetVLanInterfaceID(char *instance_name);
// char **UnixEnumVLan(char *tag_name);
// void UnixCancel(CANCEL *c);
// void UnixCleanupCancel(CANCEL *c);
// CANCEL *UnixNewCancel();
// SOCK_EVENT *UnixNewSockEvent();
// void UnixSetSockEvent(SOCK_EVENT *event);
// void UnixCleanupSockEvent(SOCK_EVENT *event);
// bool UnixWaitSockEvent(SOCK_EVENT *event, UINT timeout);
// bool UnixGetDefaultDns(IP *ip);
// void UnixNewPipe(int *pipe_read, int *pipe_write);
// void UnixWritePipe(int pipe_write);
// void UnixDeletePipe(int p1, int p2);
// void UnixSelectInner(UINT num_read, UINT *reads, UINT num_write, UINT *writes, UINT timeout);
// void UnixSetSocketNonBlockingMode(int fd, bool nonblock);

// #endif	// OS_WIN32

// // Function prototype
// void InitNetwork();
// void FreeNetwork();

// void InAddrToIP(IP *ip, struct in_addr *addr);
// void InAddrToIP6(IP *ip, struct in6_addr *addr);
// void IPToInAddr(struct in_addr *addr, IP *ip);
// void IPToInAddr6(struct in6_addr *addr, IP *ip);
// bool StrToIP(IP *ip, char *str);
// UINT StrToIP32(char *str);
// UINT UniStrToIP32(wchar_t *str);
// void IPToStr(char *str, UINT size, IP *ip);
// void IPToStr32(char *str, UINT size, UINT ip);
// void IPToStr4or6(char *str, UINT size, UINT ip_4_uint, UCHAR *ip_6_bytes);
// void IPToUniStr(wchar_t *str, UINT size, IP *ip);
// void IPToUniStr32(wchar_t *str, UINT size, UINT ip);
// bool GetHostName(char *hostname, UINT size, IP *ip);
// void GetMachineName(char *name, UINT size);
// void GetMachineNameEx(char *name, UINT size, bool no_load_hosts);
// bool GetMachineNameFromHosts(char *name, UINT size);
// void GetMachineHostName(char *name, UINT size);
// void UINTToIP(IP *ip, UINT value);
// UINT IPToUINT(IP *ip);
// SOCK *NewSock();
// void ReleaseSock(SOCK *s);
// void CleanupSock(SOCK *s);
// SOCK *Connect(char *hostname, UINT port);
// SOCK *ConnectEx(char *hostname, UINT port, UINT timeout);
// SOCK *ConnectEx2(char *hostname, UINT port, UINT timeout, bool *cancel_flag);
// SOCK *ConnectEx3(char *hostname, UINT port, UINT timeout, bool *cancel_flag, char *nat_t_svc_name, UINT *nat_t_error_code, bool try_start_ssl, bool no_get_hostname);
// SOCK *ConnectEx4(char *hostname, UINT port, UINT timeout, bool *cancel_flag, char *nat_t_svc_name, UINT *nat_t_error_code, bool try_start_ssl, bool no_get_hostname, IP *ret_ip);
// SOCK *ConnectEx5(char *hostname, UINT port, UINT timeout, bool *cancel_flag, char *nat_t_svc_name, UINT *nat_t_error_code, bool try_start_ssl, bool no_get_hostname, SSL_VERIFY_OPTION *ssl_option, UINT *ssl_err, char *hint_str, IP *ret_ip);
// SOCKET ConnectTimeoutIPv4(IP *ip, UINT port, UINT timeout, bool *cancel_flag);

// // New function named with prefix "Bind" binds outgoing connection to a specific address. New one is wrapped in original one.
// #define	BIND_LOCALIP_NULL			NULL		// NULL IP address specifies no binding
// #define	BIND_LOCALPORT_NULL			0			// NULL port number specifies no binding
// SOCK *BindConnectEx4(IP *localIP, UINT localport, char *hostname, UINT port, UINT timeout, bool *cancel_flag, char *nat_t_svc_name, UINT *nat_t_error_code, bool try_start_ssl, bool no_get_hostname, IP *ret_ip);
// SOCK *BindConnectEx5(IP *localIP, UINT localport, char *hostname, UINT port, UINT timeout, bool *cancel_flag, char *nat_t_svc_name, UINT *nat_t_error_code, bool try_start_ssl, bool no_get_hostname, SSL_VERIFY_OPTION *ssl_option, UINT *ssl_err, char *hint_str, IP *ret_ip);
// SOCKET BindConnectTimeoutIPv4(IP *localIP, UINT localport, IP *ip, UINT port, UINT timeout, bool *cancel_flag);

// bool SetSocketBufferSize(SOCKET s, bool send, UINT size);
// UINT SetSocketBufferSizeWithBestEffort(SOCKET s, bool send, UINT size);
// void InitUdpSocketBufferSize(SOCKET s);
// void QuerySocketInformation(SOCK *sock);
// bool SetTtl(SOCK *sock, UINT ttl);
// void Disconnect(SOCK *sock);
// SOCK *Listen(UINT port);
// SOCK *ListenEx(UINT port, bool local_only);
// SOCK *ListenEx2(UINT port, bool local_only, bool enable_ca, IP *listen_ip);
// SOCK *ListenEx6(UINT port, bool local_only);
// SOCK *ListenEx62(UINT port, bool local_only, bool enable_ca);
// SOCK *ListenEx63(UINT port, bool local_only, bool enable_ca, IP *listen_ip);
// SOCK *Accept(SOCK *sock);
// SOCK *Accept6(SOCK *sock);
// UINT Send(SOCK *sock, void *data, UINT size, bool secure);
// UINT Recv(SOCK *sock, void *data, UINT size, bool secure);
// UINT Peek(SOCK *sock, void *data, UINT size);
// void SetNoNeedToRead(SOCK *sock);
// UINT SecureSend(SOCK *sock, void *data, UINT size);
// UINT SecureRecv(SOCK *sock, void *data, UINT size);
// bool StartSSL(SOCK *sock, X *x, K *priv);
// bool StartSSLEx(SOCK *sock, X *x, K *priv, UINT ssl_timeout, char *sni_hostname);
// bool StartSSLEx2(SOCK *sock, X *x, K *priv, LIST *chain, UINT ssl_timeout, char *sni_hostname);
// bool StartSSLEx3(SOCK *sock, X *x, K *priv, LIST *chain, UINT ssl_timeout, char *sni_hostname, SSL_VERIFY_OPTION *ssl_option, UINT *ssl_err);
// bool AddChainSslCert(struct ssl_ctx_st *ctx, X *x);
// void AddChainSslCertOnDirectory(struct ssl_ctx_st *ctx);
// bool SendAll(SOCK *sock, void *data, UINT size, bool secure);
// void SendAdd(SOCK *sock, void *data, UINT size);
// bool SendNow(SOCK *sock, int secure);
// bool RecvAll(SOCK *sock, void *data, UINT size, bool secure);
// bool RecvAllEx(SOCK *sock, void **data_new_ptr, UINT size, bool secure);
// bool RecvAllWithDiscard(SOCK *sock, UINT size, bool secure);
// void InitSockSet(SOCKSET *set);
// void AddSockSet(SOCKSET *set, SOCK *sock);
// CANCEL *NewCancel();
// CANCEL *NewCancelSpecial(void *hEvent);
// void ReleaseCancel(CANCEL *c);
// void CleanupCancel(CANCEL *c);
// void Cancel(CANCEL *c);
// void Select(SOCKSET *set, UINT timeout, CANCEL *c1, CANCEL *c2);
// void SetWantToUseCipher(SOCK *sock, char *name);
// SOCK *NewUDP(UINT port);
// SOCK *NewUDPEx(UINT port, bool ipv6);
// SOCK *NewUDPEx2(UINT port, bool ipv6, IP *ip);
// SOCK *NewUDPEx3(UINT port, IP *ip);
// SOCK *NewUDP4(UINT port, IP *ip);
// SOCK *NewUDP6(UINT port, IP *ip);
// SOCK *NewUDPEx2Rand(bool ipv6, IP *ip, void *rand_seed, UINT rand_seed_size, UINT num_retry);
// SOCK *NewUDPEx2RandMachineAndExePath(bool ipv6, IP *ip, UINT num_retry, UCHAR rand_port_id);
// void ClearSockDfBit(SOCK *s);
// void SetRawSockHeaderIncludeOption(SOCK *s, bool enable);
// UINT SendTo(SOCK *sock, IP *dest_addr, UINT dest_port, void *data, UINT size);
// UINT SendToEx(SOCK *sock, IP *dest_addr, UINT dest_port, void *data, UINT size, bool broadcast);
// UINT SendTo6Ex(SOCK *sock, IP *dest_addr, UINT dest_port, void *data, UINT size, bool broadcast);
// UINT RecvFrom(SOCK *sock, IP *src_addr, UINT *src_port, void *data, UINT size);
// UINT RecvFrom6(SOCK *sock, IP *src_addr, UINT *src_port, void *data, UINT size);
// void SetTimeout(SOCK *sock, UINT timeout);
// UINT GetTimeout(SOCK *sock);
// bool CheckTCPPort(char *hostname, UINT port);
// bool CheckTCPPortEx(char *hostname, UINT port, UINT timeout);
// ROUTE_TABLE *GetRouteTable();
// void FreeRouteTable(ROUTE_TABLE *t);
// bool AddRouteEntryEx(ROUTE_ENTRY *e, bool *already_exists);
// bool AddRouteEntry(ROUTE_ENTRY *e);
// void DeleteRouteEntry(ROUTE_ENTRY *e);
// char **EnumVLan(char *tag_name);
// void FreeEnumVLan(char **s);
// UINT GetVLanInterfaceID(char *tag_name);
// ROUTE_ENTRY *GetBestRouteEntry(IP *ip);
// ROUTE_ENTRY *GetBestRouteEntryEx(IP *ip, UINT exclude_if_id);
// ROUTE_ENTRY *GetBestRouteEntryFromRouteTableEx(ROUTE_TABLE *table, IP *ip, UINT exclude_if_id);
// void FreeRouteEntry(ROUTE_ENTRY *e);
// void JoinSockToSockEvent(SOCK *sock, SOCK_EVENT *event);
// SOCK_EVENT *NewSockEvent();
// void SetSockEvent(SOCK_EVENT *event);
// void CleanupSockEvent(SOCK_EVENT *event);
// bool WaitSockEvent(SOCK_EVENT *event, UINT timeout);
// void ReleaseSockEvent(SOCK_EVENT *event);
// void SetIP(IP *ip, UCHAR a1, UCHAR a2, UCHAR a3, UCHAR a4);
// UINT SetIP32(UCHAR a1, UCHAR a2, UCHAR a3, UCHAR a4);
// bool GetDefaultDns(IP *ip);
// bool GetDomainName(char *name, UINT size);
// bool UnixGetDomainName(char *name, UINT size);
// void AcceptInit(SOCK *s);
// void AcceptInitEx(SOCK *s, bool no_lookup_hostname);
// void DisableGetHostNameWhenAcceptInit();
// TOKEN_LIST *GetCipherList();
// COUNTER *GetNumTcpConnectionsCounter();
// void InitWaitThread();
// void FreeWaitThread();
// void AddWaitThread(THREAD *t);
// void DelWaitThread(THREAD *t);
// bool IsSubnetMask(IP *ip);
// bool IsSubnetMask4(IP *ip);
// bool IsSubnetMask32(UINT ip);
// bool IsNetworkAddress4(IP *ip, IP *mask);
// bool IsNetworkAddress32(UINT ip, UINT mask);
// bool IsHostIPAddress4(IP *ip);
// bool IsHostIPAddress32(UINT ip);
// bool IsZeroIP(IP *ip);
// bool IsZeroIP6Addr(IPV6_ADDR *addr);
// UINT IntToSubnetMask32(UINT i);
// void IntToSubnetMask4(IP *ip, UINT i);
// bool GetNetBiosName(char *name, UINT size, IP *ip);
// bool NormalizeMacAddress(char *dst, UINT size, char *src);
// SOCKLIST *NewSockList();
// void StopSockList(SOCKLIST *sl);
// void FreeSockList(SOCKLIST *sl);
// bool IsIPv6Supported();
// bool HasIPv6Address();
// void SetSockTos(SOCK *s, int tos);
// void SetSockHighPriority(SOCK *s, bool flag);
// void InitIpClientList();
// void FreeIpClientList();
// int CompareIpClientList(void *p1, void *p2);
// void AddIpClient(IP *ip);
// void DelIpClient(IP *ip);
// IP_CLIENT *SearchIpClient(IP *ip);
// UINT GetNumIpClient(IP *ip);
// void SetLinuxArpFilter();
// int connect_timeout(SOCKET s, struct sockaddr *addr, int size, int timeout, bool *cancel_flag);
// ROUTE_CHANGE *NewRouteChange();
// void FreeRouteChange(ROUTE_CHANGE *r);
// bool IsRouteChanged(ROUTE_CHANGE *r);
// void RouteToStr(char *str, UINT str_size, ROUTE_ENTRY *e);
// void DebugPrintRoute(ROUTE_ENTRY *e);
// void DebugPrintRouteTable(ROUTE_TABLE *r);
// bool IsIPv6LocalNetworkAddress(IP *ip);
// void AddProtocolDetailsStr(char *dst, UINT dst_size, char *str);
// void AddProtocolDetailsKeyValueStr(char *dst, UINT dst_size, char *key, char *value);
// void AddProtocolDetailsKeyValueInt(char *dst, UINT dst_size, char *key, UINT value);

// #ifdef	ENABLE_SSL_LOGGING
// void SockEnableSslLogging(SOCK *s);
// void SockWriteSslLog(SOCK *s, void *send_data, UINT send_size, void *recv_data, UINT recv_size);
// void SockCloseSslLogging(SOCK *s);
// #endif	// ENABLE_SSL_LOGGING

// void SocketTimeoutThread(THREAD *t, void *param);
// SOCKET_TIMEOUT_PARAM *NewSocketTimeout(SOCK *sock);
// void FreeSocketTimeout(SOCKET_TIMEOUT_PARAM *ttp);

// void CopyIP(IP *dst, IP *src);
// bool IsIP4(IP *ip);
// void IPv6AddrToIP(IP *ip, IPV6_ADDR *addr);
// bool IPToIPv6Addr(IPV6_ADDR *addr, IP *ip);
// void SetIP6(IP *ip, UCHAR *value);
// void GetLocalHostIP6(IP *ip);
// void GetLocalHostIP4(IP *ip);
// bool IsLocalHostIP6(IP *ip);
// bool IsLocalHostIP4(IP *ip);
// bool IsLocalHostIP(IP *ip);
// void ZeroIP4(IP *ip);
// bool CheckIPItemStr6(char *str);
// void IPItemStrToChars6(UCHAR *chars, char *str);
// bool StrToIP6(IP *ip, char *str);
// bool StrToIP6Addr(IPV6_ADDR *ip, char *str);
// void IPToStr6(char *str, UINT size, IP *ip);
// void IP6AddrToStr(char *str, UINT size, IPV6_ADDR *addr);
// void IPToStr6Array(char *str, UINT size, UCHAR *bytes);
// void IPToStr6Inner(char *str, IP *ip);
// void IntToSubnetMask6(IP *ip, UINT i);
// void IPAnd6(IP *dst, IP *a, IP *b);
// void GetAllRouterMulticastAddress6(IP *ip);
// void GetAllNodeMulticaseAddress6(IP *ip);
// void GetLoopbackAddress6(IP *ip);
// UINT GetIPAddrType6(IP *ip);
// UINT GetIPv6AddrType(IPV6_ADDR *addr);
// void GetPrefixAddress6(IP *dst, IP *ip, IP *subnet);
// bool IsInSameNetwork(IP *a1, IP *a2, IP *subnet);
// bool IsInSameNetwork6(IP *a1, IP *a2, IP *subnet);
// bool IsInSameNetwork6ByStr(char *ip1, char *ip2, char *subnet);
// void GenerateEui64Address6(UCHAR *dst, UCHAR *mac);
// void GenerateEui64LocalAddress(IP *a, UCHAR *mac);
// bool IsSubnetMask6(IP *a);
// UINT SubnetMaskToInt(IP *a);
// UINT SubnetMaskToInt6(IP *a);
// UINT SubnetMaskToInt4(IP *a);
// bool IsStrIPv6Address(char *str);
// void IPAnd4(IP *dst, IP *a, IP *b);
// bool IsInSameNetwork4(IP *a1, IP *a2, IP *subnet);
// bool IsInSameNetwork4Standard(IP *a1, IP *a2);

// // Utility functions about IP and MAC address types
// bool IsValidUnicastIPAddress4(IP *ip);
// bool IsValidUnicastIPAddressUINT4(UINT ip);
// bool IsValidUnicastIPAddress6(IP *ip);
// bool IsMacUnicast(UCHAR *mac);
// bool IsMacBroadcast(UCHAR *mac);
// bool IsMacMulticast(UCHAR *mac);
// bool IsMacInvalid(UCHAR *mac);

// bool ParseIpAndSubnetMask4(char *src, UINT *ip, UINT *mask);
// bool ParseIpAndSubnetMask46(char *src, IP *ip, IP *mask);
// bool ParseIpAndMask4(char *src, UINT *ip, UINT *mask);
// bool ParseIpAndMask6(char *src, IP *ip, IP *mask);
// bool ParseIpAndMask46(char *src, IP *ip, IP *mask);
// bool IsIpStr4(char *str);
// bool IsIpStr6(char *str);
// bool IsIpMask6(char *str);
// bool StrToMask6(IP *mask, char *str);
// bool StrToMask6Addr(IPV6_ADDR *mask, char *str);
// void MaskToStr(char *str, UINT size, IP *mask);
// void Mask6AddrToStrEx(char *str, UINT size, IPV6_ADDR *mask, bool always_full_address);
// void Mask6AddrToStr(char *str, UINT size, IPV6_ADDR *mask);
// void MaskToStr32(char *str, UINT size, UINT mask);
// void MaskToStr32Ex(char *str, UINT size, UINT mask, bool always_full_address);
// void MaskToStrEx(char *str, UINT size, IP *mask, bool always_full_address);

// TUBEDATA *NewTubeData(void *data, UINT size, void *header, UINT header_size);
// void FreeTubeData(TUBEDATA *d);
// TUBE *NewTube(UINT size_of_header);
// void ReleaseTube(TUBE *t);
// void CleanupTube(TUBE *t);
// bool TubeSend(TUBE *t, void *data, UINT size, void *header);
// bool TubeSendEx(TUBE *t, void *data, UINT size, void *header, bool no_flush);
// bool TubeSendEx2(TUBE *t, void *data, UINT size, void *header, bool no_flush, UINT max_num_in_queue);
// void TubeFlush(TUBE *t);
// void TubeFlushEx(TUBE *t, bool force);
// TUBEDATA *TubeRecvAsync(TUBE *t);
// TUBEDATA *TubeRecvSync(TUBE *t, UINT timeout);
// TUBEPAIR_DATA *NewTubePairData();
// void ReleaseTubePairData(TUBEPAIR_DATA *d);
// void CleanupTubePairData(TUBEPAIR_DATA *d);
// void NewTubePair(TUBE **t1, TUBE **t2, UINT size_of_header);
// void TubeDisconnect(TUBE *t);
// bool IsTubeConnected(TUBE *t);
// void SetTubeSockEvent(TUBE *t, SOCK_EVENT *e);
// SOCK_EVENT *GetTubeSockEvent(TUBE *t);

// TUBE_FLUSH_LIST *NewTubeFlushList();
// void FreeTubeFlushList(TUBE_FLUSH_LIST *f);
// void AddTubeToFlushList(TUBE_FLUSH_LIST *f, TUBE *t);
// void FlushTubeFlushList(TUBE_FLUSH_LIST *f);

// LIST *GetHostIPAddressListInternal();
// LIST *GetHostIPAddressList();
// LIST *CloneIPAddressList(LIST *o);
// bool IsMyIPAddress(IP *ip);
// void FreeHostIPAddressList(LIST *o);
// void AddHostIPAddressToList(LIST *o, IP *ip);
// int CmpIpAddressList(void *p1, void *p2);
// UINT64 GetHostIPAddressListHash();

// UDPLISTENER *NewUdpListener(UDPLISTENER_RECV_PROC *recv_proc, void *param, IP *listen_ip);
// UDPLISTENER *NewUdpListenerEx(UDPLISTENER_RECV_PROC *recv_proc, void *param, IP *listen_ip, UINT packet_type);
// void UdpListenerThread(THREAD *thread, void *param);
// void StopUdpListener(UDPLISTENER *u);
// void FreeUdpListener(UDPLISTENER *u);
// void AddPortToUdpListener(UDPLISTENER *u, UINT port);
// void DeletePortFromUdpListener(UDPLISTENER *u, UINT port);
// void DeleteAllPortFromUdpListener(UDPLISTENER *u);
// void UdpListenerSendPackets(UDPLISTENER *u, LIST *packet_list);
// TCP_RAW_DATA *NewTcpRawData(IP *src_ip, UINT src_port, IP *dst_ip, UINT dst_port);
// void FreeTcpRawData(TCP_RAW_DATA *trd);
// UDPPACKET *NewUdpPacket(IP *src_ip, UINT src_port, IP *dst_ip, UINT dst_port, void *data, UINT size);
// void FreeUdpPacket(UDPPACKET *p);
// UDPLISTENER_SOCK *DetermineUdpSocketForSending(UDPLISTENER *u, UDPPACKET *p);
// bool IsUdpPortOpened(UDPLISTENER *u, IP *server_ip, UINT port);

// INTERRUPT_MANAGER *NewInterruptManager();
// void FreeInterruptManager(INTERRUPT_MANAGER *m);
// void AddInterrupt(INTERRUPT_MANAGER *m, UINT64 tick);
// UINT GetNextIntervalForInterrupt(INTERRUPT_MANAGER *m);

// void NewSocketPair(SOCK **client, SOCK **server, IP *client_ip, UINT client_port, IP *server_ip, UINT server_port);
// SOCK *NewInProcSocket(TUBE *tube_send, TUBE *tube_recv);
// SOCK *ListenInProc();
// SOCK *AcceptInProc(SOCK *s);
// SOCK *ConnectInProc(SOCK *listen_sock, IP *client_ip, UINT client_port, IP *server_ip, UINT server_port);
// UINT SendInProc(SOCK *sock, void *data, UINT size);
// UINT RecvInProc(SOCK *sock, void *data, UINT size);
// void WaitForTubes(TUBE **tubes, UINT num, UINT timeout);

// SOCK *ListenReverse();
// SOCK *AcceptReverse(SOCK *s);
// void InjectNewReverseSocketToAccept(SOCK *listen_sock, SOCK *s, IP *client_ip, UINT client_port);

// bool NewTcpPair(SOCK **s1, SOCK **s2);
// SOCK *ListenAnyPortEx2(bool local_only, bool disable_ca);

// ICMP_RESULT *IcmpApiEchoSend(IP *dest_ip, UCHAR ttl, UCHAR *data, UINT size, UINT timeout);
// void IcmpApiFreeResult(ICMP_RESULT *ret);

// void Win32WaitForTubes(TUBE **tubes, UINT num, UINT timeout);

// void UnixWaitForTubes(TUBE **tubes, UINT num, UINT timeout);


// SSL_PIPE *NewSslPipe(bool server_mode, X *x, K *k, DH_CTX *dh);
// SSL_PIPE *NewSslPipeEx(bool server_mode, X *x, K *k, DH_CTX *dh, bool verify_peer, struct SslClientCertInfo *clientcert);
// SSL_PIPE *NewSslPipeEx2(bool server_mode, X *x, K *k, LIST *chain, DH_CTX *dh, bool verify_peer, struct SslClientCertInfo *clientcert);
// SSL_PIPE* NewSslPipeEx3(bool server_mode, X* x, K* k, LIST* chain, DH_CTX* dh, bool verify_peer, struct SslClientCertInfo* clientcert, int tls13ticketscnt, bool disableTls13);
// void FreeSslPipe(SSL_PIPE *s);
// bool SyncSslPipe(SSL_PIPE *s);

// SSL_BIO *NewSslBioMem();
// SSL_BIO *NewSslBioSsl();
// void FreeSslBio(SSL_BIO *b);
// bool SslBioSync(SSL_BIO *b, bool sync_send, bool sync_recv);

// void SetCurrentGlobalIP(IP *ip, bool ipv6);
// bool GetCurrentGlobalIP(IP *ip, bool ipv6);
// void GetCurrentGlobalIPGuess(IP *ip, bool ipv6);
// bool IsIPAddressInSameLocalNetwork(IP *a);

// bool IsIPPrivate(IP *ip);
// bool IsIPMyHost(IP *ip);
// void LoadPrivateIPFile();
// bool IsOnPrivateIPFile(UINT ip);
// void FreePrivateIPFile();

// LIST *GetNicList();
// void FreeNicList(LIST *o);
// bool IsMacAddressLocalInner(LIST *o, void *addr);
// bool IsMacAddressLocalFast(void *addr);
// void RefreshLocalMacAddressList();

// struct ssl_ctx_st *NewSSLCtx(bool server_mode);
// void FreeSSLCtx(struct ssl_ctx_st *ctx);
// UINT GetOSSecurityLevel();

// void SetCurrentDDnsFqdn(char *name);
// void GetCurrentDDnsFqdn(char *name, UINT size);
// UINT GetCurrentDDnsFqdnHash();

// void DisableRDUPServerGlobally();

// void QueryIpThreadMain(THREAD *thread, void *param);
// QUERYIPTHREAD *NewQueryIpThread(char *hostname, UINT interval_last_ok, UINT interval_last_ng);
// bool GetQueryIpThreadResult(QUERYIPTHREAD *t, IP *ip);
// void FreeQueryIpThread(QUERYIPTHREAD *t);

// LIST *Win32GetNicList();


// void InitDynList();
// void FreeDynList();
// void AddDynList(BUF *b);
// void ExtractAndApplyDynList(PACK *p);
// void SetDynListValue(char *name, UINT64 value);
// UINT64 GetDynValue(char *name);
// UINT64 GetDynValueOrDefault(char *name, UINT64 default_value, UINT64 min_value, UINT64 max_value);
// UINT64 GetDynValueOrDefaultSafe(char *name, UINT64 default_value);
