use std::collections::HashMap;
use std::fs::DirEntry;
use std::path::Path;
use std::{fs, time};

use crate::os::cpu::{get_cpu_ticks, get_cpu_usage_last_second, get_cpu_usage_since_start, CpuTicks};
use crate::os::info::get_system_uptime;
use crate::os::memory::get_proc_memory_usage;

#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub memory_res_kb: u64,
    pub cpu_usage_avg_since_start_pct: f64,
    pub cpu_ticks: CpuTicks,
    pub cpu_usage_avg_last_second_pct: f64
}

pub fn read_processes() -> HashMap<String, ProcessInfo> {
    let mut processes_map: HashMap<String, ProcessInfo> = HashMap::new();
    let system_uptime = get_system_uptime().unwrap();
    if let Ok(entries) = fs::read_dir("/proc") {
        let entries_vec: Vec<_> = entries.filter_map(Result::ok).collect();
        let init_process_map = read_each_process(system_uptime, &entries_vec);
        std::thread::sleep(time::Duration::from_millis(1000));
        let end_process_map = read_each_process(system_uptime, &entries_vec);

        end_process_map.iter().for_each(|e| {
            let pid = e.0;
            let process_info = e.1;
            let mut updated_process_info = process_info.clone();

            if init_process_map.contains_key(&pid.to_string()) {
                let init_cpu_ticks = init_process_map.get(pid).unwrap().cpu_ticks.clone();
                let end_cpu_ticks = process_info.cpu_ticks.clone();
                let cpu_usage_last_second = get_cpu_usage_last_second(init_cpu_ticks, end_cpu_ticks);
                updated_process_info.cpu_usage_avg_last_second_pct = cpu_usage_last_second;
                processes_map.insert(pid.to_string(), updated_process_info);
            }
        })
    }
    processes_map
}

fn read_each_process(system_uptime: f64, entries: &Vec<DirEntry>) -> HashMap<String, ProcessInfo> {
    let mut processes_map: HashMap<String, ProcessInfo> = HashMap::new();

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            if let Some(pid_str) = path.file_name().and_then(|s| s.to_str()) {
                if let Ok(pid) = pid_str.parse::<u32>() {
                    if let Some(info) = parse_process_data(pid, &path, system_uptime) {
                        if processes_map.contains_key(&info.name) {
                            let p = processes_map.get_mut(&info.name).unwrap();
                            p.memory_res_kb += info.memory_res_kb;
                            p.cpu_usage_avg_since_start_pct += info.cpu_usage_avg_since_start_pct;
                        } else {
                            processes_map.insert(info.name.to_string(), info);
                        }
                    }
                }
            }
        }
    }
    processes_map.remove("toprs"); //ignore self process
    return processes_map
}

fn get_proc_name(proc_path: &Path) -> Option<String> {
    let comm_path = proc_path.join("comm");
    Some(fs::read_to_string(comm_path).ok()?.trim().to_string())
}

fn parse_process_data(pid: u32, proc_path: &Path, system_uptime: f64) -> Option<ProcessInfo> {
    // Ler /proc/[PID]/comm para pegar o nome
    let name = get_proc_name(proc_path)?;
    let memory_res_kb = get_proc_memory_usage(proc_path)?;
    let cpu_ticks = get_cpu_ticks(proc_path)?;
    let cpu_usage_pct = get_cpu_usage_since_start(proc_path, system_uptime)?;
    Some(ProcessInfo {
        pid,
        name,
        memory_res_kb,
        cpu_usage_avg_since_start_pct: cpu_usage_pct,
        cpu_ticks,
        cpu_usage_avg_last_second_pct: 0.0,
    })
}
