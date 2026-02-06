use serde::{Deserialize, Serialize};

// All this is for later, right now SoftEther handles this in Cedar
#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    save_password: bool,
    encrypted_password: String,
    remote_password_only: bool,
    user_agent: String,
    secure_device_id: u32,

    AccountList: Vec<Account>,
    
    client_manager_settings: ClientManagerSetting,
    common_proxy_settings: CommonProxySetting,
    extra_config: _Config,
    root_ca: RootCA
}

#[derive(Serialize, Deserialize, Default)]
pub struct Account {
    create_datetime: u64,
    last_connect_datetime: u64,
    retry_on_server_cert: bool,
    shortcut_key: String,
    startup_account: bool,
    update_datetime: u64,
    client_auth: ClientAuthSettings,
    client_options: ClientOptions,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ClientAuthSettings {
    auth_type: u32,
    encrypted_password: String,
    username: String,
}


#[derive(Serialize, Deserialize, Default)]
pub struct ClientOptions {
    bind_local_ip: String,
    bind_local_port: u32,
    connection_disconnect_span: u32,
    custom_http_header: String,
    device_name: String,
    disable_qos: bool,
    half_connection: bool,
    hide_nic_info_window: bool,
    hide_status_window: bool,
    hostname: String,
    hub_name: String,
    max_connection: u32,
    no_routing_tracking: bool,
    no_udp_acceleration: bool,
    num_retry: u32,
    port: u32,
    port_udp: u32,
    proxy_name: String,
    proxy_port: u32,
    proxy_type: u32,
    proxy_username: String,
    require_bridge_routing_mode: bool,
    require_monitor_mode: bool,
    retry_interval: u32,
    use_compress: bool,
    use_encrypt: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ClientManagerSetting {
    easy_mode: bool,
    lock_mode: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CommonProxySetting {
    custom_http_header: String,
    proxy_host_name: String,
    proxy_port: u32,
    proxy_type: u32,
    proxy_username: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct _Config {
    allow_remote_config: bool,
    auto_delete_check_disk_free_space_min: u64,
    disable_rpc_dynamic_port_listener: bool,
    keep_connect_host: String,
    keep_connect_interval: u32,
    keep_connect_port: u32,
    keep_connect_protocol: u32,
    no_change_wcm_network_setting_on_windows8: bool,
    use_keep_connect: bool,
}

// #[derive(Serialize, Deserialize, Default)]
// pub struct AccountDatabase {
//     accounts: Vec<Account>,
// }

#[derive(Serialize, Deserialize, Default)]
pub struct RootCA {}
