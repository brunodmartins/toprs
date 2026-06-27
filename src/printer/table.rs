use crate::os::process::ProcessInfo;
use crate::printer::formatter::{format_cpu_display, format_mem_display};
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;

pub fn pretty_print_table(processes: Vec<ProcessInfo>, total_memory: u64) {
    println!("Summary:");
    println!("Total Memory available: {}", total_memory);
    let total_memory_used: u64 = processes.iter().map(|p| p.memory_res_kb).sum();
    println!("Total Memory used: {} ({}%)", total_memory_used, (total_memory_used * 100) / total_memory);
    println!("Processes:");
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["PID", "NAME", "Memory Usage","CPU Usage (avg last second)", "CPU Usage (avg since start)"]);
    for process in processes {
        table.add_row(vec![
            process.pid.to_string(),
            process.name,
            format_mem_display(process.memory_res_kb, total_memory),
            format_cpu_display(process.cpu_usage_avg_last_second_pct),
            format_cpu_display(process.cpu_usage_avg_since_start_pct),
        ]);
    }
    println!("{table}");
}
