use anyhow::Result;
use sysinfo::System;

#[derive(Debug, Clone)]
pub struct SystemStats {
    pub cpu_usage: f32,
    pub ram_used: u64,
    pub ram_total: u64,
    pub disk_used: u64,
    pub disk_total: u64,
    pub uptime: u64,
}

pub struct SystemModule {
    system: System,
}

impl SystemModule {
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_all();
        SystemModule { system }
    }
    
    pub fn get_stats(&mut self) -> Result<SystemStats> {
        self.system.refresh_all();
        
        let cpu_usage = self.system.global_cpu_usage() as f32;
        let ram_total = self.system.total_memory();
        let ram_used = self.system.used_memory();
        let disk_total = self.system.total_swap();
        let disk_used = self.system.used_swap();
        let uptime = sysinfo::System::uptime();
        
        Ok(SystemStats {
            cpu_usage,
            ram_used,
            ram_total,
            disk_used,
            disk_total,
            uptime,
        })
    }
}
