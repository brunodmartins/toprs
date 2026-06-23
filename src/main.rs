use std::collections::HashMap;
use std::fs;
use std::path::Path;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    limit: Option<usize>,
}

#[derive(Debug)]
struct ProcessInfo {
    pid: u32,
    name: String,
    memory_res_kb: u64,
}

fn main() {
    let args = Args::parse();
    let processes_map = read_processes();
    let mut processes : Vec<ProcessInfo> = processes_map.into_values().collect();
    processes.sort_by_key(|p| std::cmp::Reverse(p.memory_res_kb));

    if args.limit != None {
        _ = processes.split_off(args.limit.unwrap());
    }
    pretty_print_table(processes);
}

fn read_processes() -> HashMap<String, ProcessInfo> {
    let mut processes_map: HashMap<String, ProcessInfo> = HashMap::new();
    // 1. Listar o diretório /proc
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(pid_str) = path.file_name().and_then(|s| s.to_str()) {
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        if let Some(info) = parse_process_data(pid, &path) {
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
    processes_map
}

fn pretty_print_table(processes: Vec<ProcessInfo>) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["PID", "NAME", "Memory Usage"]);
    let total_memory = get_system_memory_kbs();
    for process in processes {
        table.add_row(vec![
            process.pid.to_string(),
            process.name,
            format_mem_display(process.memory_res_kb, total_memory.unwrap()),
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

fn format_mem_display(mem_usage_kb: u64, total_memory_kb: u64) -> String {
    if mem_usage_kb == 0 {
        return "0 KB".to_string();
    }
    let usage_percentage = (mem_usage_kb as f32 * 100.00) / total_memory_kb as f32;
    if mem_usage_kb < 1024 {
        return format!("{} KB ({:.2}%)", mem_usage_kb, usage_percentage);
    }
    if mem_usage_kb < 1024 * 1024 {
        return format!("{} MB ({:.2}%)", mem_usage_kb / 1024, usage_percentage);
    }
    return format!("{} GB ({:.2}%)", mem_usage_kb / 1024 / 1024, usage_percentage);

}

fn get_system_memory_kbs() -> Option<u64> {
    let content = fs::read_to_string("/proc/meminfo").ok()?;
    let first_line = content.lines().next()?;
    let total_kb_str: String = first_line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    total_kb_str.parse::<u64>().ok()
}