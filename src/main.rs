use std::fs;
use std::path::Path;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, Table};

#[derive(Debug)]
struct ProcessInfo {
    pid: u32,
    name: String,
    memory_res_kb: u64,
}

fn main() {
    let mut processes: Vec<ProcessInfo> = vec![];
    // 1. Listar o diretório /proc
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(pid_str) = path.file_name().and_then(|s| s.to_str()) {
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        if let Some(info) = parse_process_data(pid, &path) {
                            processes.push(info);
                        }
                    }
                }
            }
        }
    }
    pretty_print_table(processes);
}

fn pretty_print_table(processes: Vec<ProcessInfo>) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["PID", "NAME", "Memory Usage"]);
    for process in processes {
        table.add_row(vec![
            process.pid.to_string(),
            process.name,
            format_mem_display(process.memory_res_kb)
        ]);
    }
    println!("{table}");
}

fn parse_process_data(pid: u32, proc_path: &Path) -> Option<ProcessInfo> {
    // Ler /proc/[PID]/comm para pegar o nome
    let name = get_proc_name(proc_path)?;
    let memory_res_kb = get_proc_memory_usage(proc_path)?;
    Some(ProcessInfo {
        pid,
        name,
        memory_res_kb,
    })
}

fn get_proc_name(proc_path: &Path) -> Option<String> {
    let comm_path = proc_path.join("comm");
    Some(fs::read_to_string(comm_path).ok()?.trim().to_string())
}

fn get_proc_memory_usage(proc_path: &Path) -> Option<u64> {
    // Ler /proc/[PID]/statm para pegar a memória residente
    let statm_path = proc_path.join("statm");
    let statm_content = fs::read_to_string(statm_path).ok()?;
    let mut statm_parts = statm_content.split_whitespace();

    // O segundo elemento é a memória residente em páginas
    let _virt_pages = statm_parts.next();
    let res_pages = statm_parts.next()?.parse::<u64>().ok()?;

    // Convertendo páginas de 4KB para Kilobytes
    let memory_res_kb = res_pages * 4;
    Some(memory_res_kb)
}

fn format_mem_display(mem_usage_kb: u64) -> String {
    if mem_usage_kb > 1024 ^ 2 {
        return format!("{} MB", mem_usage_kb / 1024);
    }
    if mem_usage_kb > 1024 ^ 3 {
        return format!("{} GB", mem_usage_kb / (1024 ^ 2));
    }
    return format!("{} KB", mem_usage_kb)
}