use std::ops::Range;

#[derive(Clone)]
pub struct SimulationConfig {
    pub ai: AiConfig,
    pub autotools: AutotoolsConfig,
    pub cmake: CmakeConfig,
    pub cloud: CloudConfig,
    pub container: ContainerConfig,
    pub bios: BiosConfig,
    pub boot: BootConfig,
    pub bootloader: BootloaderConfig,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            ai: AiConfig::default(),
            autotools: AutotoolsConfig::default(),
            cmake: CmakeConfig::default(),
            cloud: CloudConfig::default(),
            container: ContainerConfig::default(),
            bios: BiosConfig::default(),
            boot: BootConfig::default(),
            bootloader: BootloaderConfig::default(),
        }
    }
}

#[derive(Clone)]
pub struct BiosConfig {
    pub vendor: &'static str,
    pub version: &'static str,
    pub new_version: &'static str,
    pub bios_date: &'static str,
    pub header_delay: u64,
    pub post_start_delay: u64,
    pub cpu_detect_time: u64,
    pub cpu_cores_time: u64,
    pub cpu_freq_time: u64,
    pub memory_test_time: u64,
    pub memory_details_time: u64,
    pub cmos_warning_time: u64,
    pub ide_master_time: u64,
    pub ide_slave_time: u64,
    pub pci_scan_time: u64,
    pub pci_device_time: u64,
    pub network_detect_time: u64,
    pub usb_detect_time: u64,
    pub system_info_time: u64,
    pub uuid_time: u64,
    pub boot_priority_time: u64,
    pub boot_display_time: u64,
    pub firmware_header_delay: u64,
    pub backup_time: u64,
    pub verify_time: u64,
    pub warning_delay: u64,
    pub erase_min: u64,
    pub erase_max: u64,
    pub write_min: u64,
    pub write_max: u64,
    pub verify_min: u64,
    pub verify_max: u64,
    pub complete_time: u64,
    pub escd_time: u64,
    pub success_delay: u64,
    pub cmos_error_chance: f64,
}

impl Default for BiosConfig {
    fn default() -> Self {
        Self {
            vendor: "American Megatrends BIOS (C)2003-2025",
            version: "AMIBIOS v08.00.15",
            new_version: "v08.00.16",
            bios_date: "11/15/2025",
            header_delay: 400,
            post_start_delay: 400,
            cpu_detect_time: 800,
            cpu_cores_time: 600,
            cpu_freq_time: 500,
            memory_test_time: 1200,
            memory_details_time: 700,
            cmos_warning_time: 800,
            ide_master_time: 900,
            ide_slave_time: 400,
            pci_scan_time: 800,
            pci_device_time: 500,
            network_detect_time: 600,
            usb_detect_time: 500,
            system_info_time: 500,
            uuid_time: 700,
            boot_priority_time: 600,
            boot_display_time: 500,
            firmware_header_delay: 600,
            backup_time: 1800,
            verify_time: 1200,
            warning_delay: 800,
            erase_min: 1500,
            erase_max: 2500,
            write_min: 3000,
            write_max: 5000,
            verify_min: 2000,
            verify_max: 3500,
            complete_time: 800,
            escd_time: 1000,
            success_delay: 600,
            cmos_error_chance: 0.25,
        }
    }
}

#[derive(Clone)]
pub struct BootConfig {
    pub log_count_range: Range<usize>,
    pub log_delay_range: Range<u64>,
    pub final_delay: u64,
}

impl Default for BootConfig {
    fn default() -> Self {
        Self {
            log_count_range: 8..15,
            log_delay_range: 50..200,
            final_delay: 300,
        }
    }
}

#[derive(Clone)]
pub struct BootloaderConfig {
    pub install_delay: u64,
    pub probe_delay: u64,
    pub device_install_delay: u64,
    pub config_gen_delay: u64,
    pub kernel_scan_delay_range: Range<u64>,
    pub windows_found_chance: f64,
    pub windows_delay: u64,
    pub write_stage_delay_range: Range<u64>,
    pub finish_delay: u64,
}

impl Default for BootloaderConfig {
    fn default() -> Self {
        Self {
            install_delay: 800,
            probe_delay: 600,
            device_install_delay: 500,
            config_gen_delay: 700,
            kernel_scan_delay_range: 200..400,
            windows_found_chance: 0.3,
            windows_delay: 400,
            write_stage_delay_range: 400..800,
            finish_delay: 500,
        }
    }
}

#[derive(Clone)]
pub struct AiConfig {
    pub model_download_speed_range: Range<u64>,
    pub failure_rate_network: f64,
    pub failure_rate_checksum: f64,
    pub failure_rate_kernel_panic: f64,
    pub failure_rate_oom: f64,
    pub layer_load_delay_range: Range<u64>,
    pub compilation_speed_range: Range<u64>,
    pub checksum_delay_range: Range<u64>,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            model_download_speed_range: 1000..3000,
            failure_rate_network: 0.15,
            failure_rate_checksum: 0.05,
            failure_rate_kernel_panic: 0.1,
            failure_rate_oom: 0.2,
            layer_load_delay_range: 300..800,
            compilation_speed_range: 800..2000,
            checksum_delay_range: 500..1500,
        }
    }
}

#[derive(Clone)]
pub struct CloudConfig {
    pub failure_rate_rate_limit: f64,
    pub failure_rate_insufficient_capacity: f64,
    pub failure_rate_dependency_violation: f64,
    pub failure_rate_checksum_mismatch: f64,
    pub provision_speed_range: Range<u64>,
}

impl Default for CloudConfig {
    fn default() -> Self {
        Self {
            failure_rate_rate_limit: 0.05,
            failure_rate_insufficient_capacity: 0.3,
            failure_rate_dependency_violation: 0.3,
            failure_rate_checksum_mismatch: 0.2,
            provision_speed_range: 300..1200,
        }
    }
}

#[derive(Clone)]
pub struct ContainerConfig {
    pub failure_rate_image_pull: f64,
    pub failure_rate_readiness_probe: f64,
    pub failure_rate_crash_loop: f64,
    pub probability_volume_mount: f64,
    pub probability_secret_mount: f64,
    pub probability_sidecar_injection: f64,
    pub layer_pull_speed_range: Range<u64>,
}

impl Default for ContainerConfig {
    fn default() -> Self {
        Self {
            failure_rate_image_pull: 0.15,
            failure_rate_readiness_probe: 0.2,
            failure_rate_crash_loop: 0.05,
            probability_volume_mount: 0.4,
            probability_secret_mount: 0.3,
            probability_sidecar_injection: 0.6,
            layer_pull_speed_range: 150..2500,
        }
    }
}

#[derive(Clone)]
pub struct AutotoolsConfig {
    pub failure_rate: f64,
    pub check_delay_range: Range<u64>,
    pub compile_delay_range: Range<u64>,
    pub link_duration_range: Range<u64>,
    pub retry_delay: u64,
}

impl Default for AutotoolsConfig {
    fn default() -> Self {
        Self {
            failure_rate: 0.25,
            check_delay_range: 15..70,
            compile_delay_range: 40..180,
            link_duration_range: 400..1200,
            retry_delay: 1500,
        }
    }
}

#[derive(Clone)]
pub struct CmakeConfig {
    pub failure_rate: f64,
    pub probe_delay_range: Range<u64>,
    pub build_delay_range: Range<u64>,
    pub link_duration_range: Range<u64>,
    pub retry_delay: u64,
}

impl Default for CmakeConfig {
    fn default() -> Self {
        Self {
            failure_rate: 0.3,
            probe_delay_range: 30..120,
            build_delay_range: 60..220,
            link_duration_range: 600..1800,
            retry_delay: 1500,
        }
    }
}
