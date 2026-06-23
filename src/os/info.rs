use std::fs;

pub fn get_system_memory_kbs() -> Option<u64> {
    let content = fs::read_to_string("/proc/meminfo").ok()?;
    let first_line = content.lines().next()?;
    let total_kb_str: String = first_line.chars().filter(|c| c.is_ascii_digit()).collect();
    total_kb_str.parse::<u64>().ok()
}

pub fn get_system_uptime() -> Option<f64> {
    let content = fs::read_to_string("/proc/uptime").ok()?;
    let first_word = content.split_whitespace().next()?;
    first_word.parse::<f64>().ok()
}
