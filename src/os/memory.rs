use std::fs;
use std::path::Path;

pub fn get_proc_memory_usage(proc_path: &Path) -> Option<u64> {
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

pub fn get_system_memory_kbs() -> Option<u64> {
    let content = fs::read_to_string("/proc/meminfo").ok()?;
    let first_line = content.lines().next()?;
    let total_kb_str: String = first_line.chars().filter(|c| c.is_ascii_digit()).collect();
    total_kb_str.parse::<u64>().ok()
}
