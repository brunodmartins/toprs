use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::os::cpu::get_cpu_usage;
use crate::os::info::get_system_uptime;
use crate::os::memory::get_proc_memory_usage;

#[derive(Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub memory_res_kb: u64,
    pub cpu_usage_pct: f64,
}

pub fn read_processes() -> HashMap<String, ProcessInfo> {
    let mut processes_map: HashMap<String, ProcessInfo> = HashMap::new();
    let system_uptime = get_system_uptime().unwrap();
    // 1. Listar o diretório /proc
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(pid_str) = path.file_name().and_then(|s| s.to_str()) {
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        if let Some(info) = parse_process_data(pid, &path, system_uptime) {
                            if processes_map.contains_key(&info.name) {
                                let p = processes_map.get_mut(&info.name).unwrap();
                                p.memory_res_kb += info.memory_res_kb;
                            } else {
                                processes_map.insert(info.name.to_string(), info);
                            }
                        }
                    }
                }
            }
        }
    }
    processes_map.remove("toprs"); //ignore self process
    processes_map
}

fn get_proc_name(proc_path: &Path) -> Option<String> {
    let comm_path = proc_path.join("comm");
    Some(fs::read_to_string(comm_path).ok()?.trim().to_string())
}

fn parse_process_data(pid: u32, proc_path: &Path, system_uptime: f64) -> Option<ProcessInfo> {
    // Ler /proc/[PID]/comm para pegar o nome
    let name = get_proc_name(proc_path)?;
    let memory_res_kb = get_proc_memory_usage(proc_path)?;
    let cpu_usage_pct = get_cpu_usage(proc_path, system_uptime)?;
    Some(ProcessInfo {
        pid,
        name,
        memory_res_kb,
        cpu_usage_pct,
    })
}
