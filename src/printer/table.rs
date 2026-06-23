use crate::os::process::ProcessInfo;
use crate::printer::formatter::format_mem_display;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;

pub fn pretty_print_table(processes: Vec<ProcessInfo>, total_memory: u64) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["PID", "NAME", "Memory Usage"]);
    for process in processes {
        table.add_row(vec![
            process.pid.to_string(),
            process.name,
            format_mem_display(process.memory_res_kb, total_memory),
        ]);
    }
    println!("{table}");
}
