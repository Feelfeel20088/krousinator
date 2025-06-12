use common::{
    registry::{
        Handleable,
        KrousinatorInterface,
    },
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use krous_macros::register_handler;
use sysinfo::{Disks, Networks, System};
use tokio::fs;


// Supporting structs
#[derive(Debug, Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub total_space_bytes: u64,
    pub available_space_bytes: u64,
    pub is_removable: bool,
}
#[derive(Debug, Serialize)]
pub struct NetworkInterfaceInfo {
    pub name: String,
    pub mac_address: String,
    pub precent_of_inbound_packets_lost: u64,
    pub precent_of_outbound_packets_lost: u64,
    pub mtu: u64,
    pub ip_addresses: Vec<String>,  // IPv4 and IPv6
}

#[derive(Serialize, Debug)]
pub struct SystemInfoSend {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub os_architecture: String,
    pub kernel_version: String,
    pub uptime_seconds: u64,

    pub cpu_vendor: String,
    pub cpu_brand: String,
    pub cpu_physical_cores: usize,
    pub cpu_logical_cores: usize,
    pub cpu_frequency_mhz: u64,
    pub cpu_features: Vec<String>,

    pub total_memory_bytes: u64,
    pub available_memory_bytes: u64,
    pub total_swap_bytes: u64,
    pub available_swap_bytes: u64,

    pub disks: Vec<DiskInfo>,
    pub network_interfaces: Vec<NetworkInterfaceInfo>,

    pub gpu_vendor: Option<String>,
    pub gpu_model: Option<String>,

    pub machine_id: Option<String>,
    pub bios_serial_number: Option<String>,
    pub motherboard_serial_number: Option<String>,
    pub system_uuid: Option<String>,

    pub username: Option<String>,
    pub shell: Option<String>,
    pub user_langs: Option<Vec<String>>,
    pub timezone: Option<String>,

    pub is_virtual_machine: bool,
    pub battery_percentage: Option<u8>,
    pub is_laptop: bool,

    // Uncomment if you want to send environment vars, be cautious about sensitive info
    // pub environment_vars: Option<std::collections::HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
#[register_handler]
pub struct SystemInfoReq {
    _t: String,
}

// TODO get all the feilds i was to lazy to get
#[async_trait]
impl Handleable for SystemInfoReq {
    async fn handle(&self, ctx: &mut KrousinatorInterface) {
        let mut sys = System::new_all();
        sys.refresh_all();

        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());

        let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
        let os_architecture = std::env::consts::ARCH.to_string();
        let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        let uptime_seconds = System::uptime();

        let cpu = sys.cpus().get(0);
        let cpu_vendor = cpu.map(|c| c.vendor_id().to_string()).unwrap_or_default();
        let cpu_brand = cpu.map(|c| c.brand().to_string()).unwrap_or_default();
        let cpu_physical_cores = System::physical_core_count().unwrap_or(0);
        let cpu_logical_cores = sys.cpus().len();
        let cpu_frequency_mhz = cpu.map(|c| c.frequency()).unwrap_or(0);
        let cpu_features = vec![]; // Advanced: parse /proc/cpuinfo or CPUID later

        let total_memory_bytes = sys.total_memory();
        let available_memory_bytes = sys.available_memory();
        let total_swap_bytes = sys.total_swap();
        let available_swap_bytes = sys.free_swap();
        
        let disks = Disks::new_with_refreshed_list().iter().map(|d| DiskInfo {
            name: d.name().to_string_lossy().to_string(),
            mount_point: d.mount_point().to_string_lossy().to_string(),
            file_system: d.file_system().to_string_lossy().to_string(),
            total_space_bytes: d.total_space(),
            available_space_bytes: d.available_space(),
            is_removable: d.is_removable(),
        }).collect();

        let network_interfaces =  Networks::new_with_refreshed_list().iter().map(|(name, data)| NetworkInterfaceInfo {
            name: name.clone(),
            
            mac_address: data.mac_address().0
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<String>>()
            .join(":"),

            precent_of_inbound_packets_lost: if data.total_packets_received() == 0 {
                0
            } else {
                (data.total_errors_on_received() * 100) / data.total_packets_received()
            },
            
            precent_of_outbound_packets_lost: if data.total_packets_transmitted() == 0 {
                0
            } else {
                (data.total_errors_on_transmitted() * 100) / data.total_packets_transmitted()
            },

        
            mtu: data.mtu(),
            ip_addresses: data.ip_networks()
            .iter()
            .map(|ip_network| format!("{}/{}", ip_network.addr, ip_network.prefix))
            .collect(),
            
        }).collect();

        let gpu_vendor = None;  // Advanced: parse lspci (Linux), DirectX (Windows), etc.
        let gpu_model = None;

        let machine_id = fs::read_to_string("/etc/machine-id").await.ok().map(|s| s.trim().to_string());
        let bios_serial_number = None;  // Advanced: use dmidecode or WMI
        let motherboard_serial_number = None;
        let system_uuid = None;

        let username = Some(whoami::username());
        let shell = std::env::var("SHELL").ok();
        let user_langs: Option<Vec<String>> = Some(whoami::langs().unwrap().map(|lang| lang.country().to_string()).collect());
        let timezone = std::env::var("TZ").ok();

        let is_virtual_machine = false; 
        let battery_percentage = None; 
        let is_laptop = false; 

        // let environment_vars = Some(std::env::vars().collect::<HashMap<String, String>>());

        let result = SystemInfoSend {
            hostname,
            os_name,
            os_version,
            os_architecture,
            kernel_version,
            uptime_seconds,

            cpu_vendor,
            cpu_brand,
            cpu_physical_cores,
            cpu_logical_cores,
            cpu_frequency_mhz,
            cpu_features,

            total_memory_bytes,
            available_memory_bytes,
            total_swap_bytes,
            available_swap_bytes,

            disks,
            network_interfaces,

            gpu_vendor,
            gpu_model,

            machine_id,
            bios_serial_number,
            motherboard_serial_number,
            system_uuid,

            username,
            shell,
            user_langs,
            timezone,

            is_virtual_machine,
            battery_percentage,
            is_laptop,

            // environment_vars,
        };

        ctx.send(result);
        


    }
}
