use async_trait::async_trait;
use common::registry::{HiveContext, HiveHandleable};
use krous_macros::register_hive_handler;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct DiskInfo {
    name: String,
    mount_point: String,
    file_system: String,
    total_space_bytes: u64,
    available_space_bytes: u64,
    is_removable: bool,
}
#[derive(Debug, Deserialize)]
pub struct NetworkInterfaceInfo {
    name: String,
    mac_address: String,
    precent_of_inbound_packets_lost: u64,
    precent_of_outbound_packets_lost: u64,
    mtu: u64,
    ip_addresses: Vec<String>, // IPv4 and IPv6
}

#[derive(Deserialize, Debug)]
#[register_hive_handler]
pub struct SystemInfoRecv {
    manual_request_id: Option<Uuid>,
    hostname: String,
    os_name: String,
    os_version: String,
    os_architecture: String,
    kernel_version: String,
    uptime_seconds: u64,

    cpu_vendor: String,
    cpu_brand: String,
    cpu_physical_cores: usize,
    cpu_logical_cores: usize,
    cpu_frequency_mhz: u64,
    cpu_features: Vec<String>,

    total_memory_bytes: u64,
    available_memory_bytes: u64,
    total_swap_bytes: u64,
    available_swap_bytes: u64,

    disks: Vec<DiskInfo>,
    network_interfaces: Vec<NetworkInterfaceInfo>,

    gpu_vendor: Option<String>,
    gpu_model: Option<String>,

    machine_id: Option<String>,
    bios_serial_number: Option<String>,
    motherboard_serial_number: Option<String>,
    system_uuid: Option<String>,

    username: Option<String>,
    shell: Option<String>,
    user_langs: Option<Vec<String>>,
    timezone: Option<String>,

    is_virtual_machine: bool,
    battery_percentage: Option<u8>,
    is_laptop: bool,
    // pub environment_vars: Option<std::collections::HashMap<String, String>>,
}

#[async_trait]
impl HiveHandleable for SystemInfoRecv {
    async fn handle(&self, ctx: &HiveContext) {}
}
