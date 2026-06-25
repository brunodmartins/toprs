use crate::os::process::ProcessInfo;
use crate::printer::formatter::{format_cpu_display, format_mem_display};
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;

pub fn pretty_print_table(processes: Vec<ProcessInfo>, total_memory: u64) {
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
