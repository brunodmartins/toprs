pub fn format_mem_display(mem_usage_kb: u64, total_memory_kb: u64) -> String {
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
    return format!(
        "{} GB ({:.2}%)",
        mem_usage_kb / 1024 / 1024,
        usage_percentage
    );
}
