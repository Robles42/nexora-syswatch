use sysinfo::System;

pub struct ProcessInfo {
    pub name: String,
    pub mem_usage: u64,
}

pub struct SystemStats {
    pub cpu_usage: f32,
    pub mem_used: u64,
    pub mem_total: u64,
    pub host_name: String,
    pub top_processes: Vec<ProcessInfo>,
}

impl SystemStats {
    pub fn refresh(sys: &mut System) -> Self {
        sys.refresh_all();
        
        let cpu_usage = sys.global_cpu_info().cpu_usage();
        let mem_used = sys.used_memory() / 1024 / 1024; 
        let mem_total = sys.total_memory() / 1024 / 1024;
        let host_name = System::host_name().unwrap_or_else(|| "Linux-User".to_string());

        let mut processes: Vec<_> = sys.processes().values().collect();
        processes.sort_by(|a, b| b.memory().cmp(&a.memory()));
        
        let top_processes = processes.iter().take(5).map(|p| ProcessInfo {
            name: p.name().to_string(),
            mem_usage: p.memory() / 1024 / 1024,
        }).collect();

        Self {
            cpu_usage,
            mem_used,
            mem_total,
            host_name,
            top_processes,
        }
    }
}