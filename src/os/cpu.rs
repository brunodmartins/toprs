use std::fs;
use std::path::Path;
use std::thread::available_parallelism;

pub fn get_cpu_usage(proc_path: &Path, system_uptime: f64) -> Option<f64> {
    let stat_content = fs::read_to_string(proc_path.join("stat")).ok()?;
    let r_paren_index = stat_content.rfind(')')?;
    let post_paren = &stat_content[r_paren_index + 2..];
    let parts: Vec<&str> = post_paren.split_whitespace().collect();

    let utime = parts.get(11)?.parse::<f64>().ok()?;
    let stime = parts.get(12)?.parse::<f64>().ok()?;
    let starttime = parts.get(19)?.parse::<f64>().ok()?;

    let clk_tck = 100.0;
    let num_cores = available_parallelism().unwrap().get() as f64;

    let total_time_ticks = utime + stime;
    let process_start_seconds = starttime / clk_tck;

    let process_duration_seconds = system_uptime - process_start_seconds;

    // Evita divisão por zero se o processo acabou de nascer
    let cpu_usage_pct = if process_duration_seconds > 0.0 {
        // (Tempo de CPU usado / Tempo total de vida) * 100 * Cores
        ((total_time_ticks / clk_tck) / process_duration_seconds) * 100.0 * num_cores
    } else {
        0.0
    };
    Some(cpu_usage_pct)
}
