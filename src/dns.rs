// void DnsInit();
// void DnsFree();

// UINT DnsThreadNum();
// UINT DnsThreadNumMax();
// void DnsThreadNumMaxSet(const UINT num);

// bool DnsCacheIsEnabled();
// void DnsCacheToggle(const bool enabled);

// DNS_CACHE *DnsCacheFind(const char *hostname);
// DNS_CACHE *DnsCacheUpdate(const char *hostname, const IP *ipv6, const IP *ipv4);
// DNS_CACHE *DnsCacheUpdateEx(const char *hostname, const LIST *iplist_v6, const LIST *iplist_v4);

// DNS_CACHE_REVERSE *DnsCacheReverseFind(const IP *ip);
// DNS_CACHE_REVERSE *DnsCacheReverseUpdate(const IP *ip, const char *hostname);

// bool DnsResolve(IP *ipv6, IP *ipv4, const char *hostname, UINT timeout, volatile const bool *cancel_flag);
// bool DnsResolveEx(LIST **iplist_v6, LIST **iplist_v4, const char *hostname, UINT timeout, volatile const bool *cancel_flag);
// void DnsResolver(THREAD *t, void *param);

// bool DnsResolveReverse(char *dst, const UINT size, const IP *ip, UINT timeout, volatile const bool *cancel_flag);
// void DnsResolverReverse(THREAD *t, void *param);

// bool GetIPEx(IP *ip, const char *hostname, UINT timeout, volatile const bool *cancel_flag);

// void ReleaseDnsResolver(DNS_RESOLVER *p);
// void ReleaseDnsResolverReverse(DNS_RESOLVER_REVERSE *p);