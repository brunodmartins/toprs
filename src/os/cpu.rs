use std::fs;
use std::path::Path;
use std::thread::available_parallelism;

#[derive(Clone)]
pub struct CpuTicks {
    utime: f64,
    stime: f64,
    start_time: f64,
}

impl CpuTicks {
    fn new(utime: f64, stime: f64, start_time: f64) -> Self {
        Self { utime, stime, start_time }
    }

    fn ticks(&self) -> f64 {
        self.utime + self.stime
    }
}

pub fn get_cpu_ticks(proc_path: &Path) -> Option<CpuTicks> {
    let stat_content = fs::read_to_string(proc_path.join("stat")).ok()?;
    let r_paren_index = stat_content.rfind(')')?;
    let post_paren = &stat_content[r_paren_index + 2..];
    let parts: Vec<&str> = post_paren.split_whitespace().collect();

    let utime = parts.get(11)?.parse::<f64>().ok()?;
    let stime = parts.get(12)?.parse::<f64>().ok()?;
    let start_time = parts.get(19)?.parse::<f64>().ok()?;

    return Some(CpuTicks::new(utime, stime, start_time));
}

pub fn get_cpu_usage_last_second(cpu_ticks_init: CpuTicks, cpu_ticks_end: CpuTicks) -> f64 {
    let ticks = cpu_ticks_end.ticks() - cpu_ticks_init.ticks();
    let clk_tck = 100.0;
    let num_cores = available_parallelism().unwrap().get() as f64;
    return (ticks / clk_tck) * 100.00 * num_cores
}

pub fn get_cpu_usage_since_start(proc_path: &Path, system_uptime: f64) -> Option<f64> {
    let cpu_ticks = get_cpu_ticks(proc_path).unwrap();
    let clk_tck = 100.0;
    let num_cores = available_parallelism().unwrap().get() as f64;

    let process_start_seconds = cpu_ticks.start_time / clk_tck;

    let process_duration_seconds = system_uptime - process_start_seconds;

    // Evita divisão por zero se o processo acabou de nascer
    let cpu_usage_pct = if process_duration_seconds > 0.0 {
        // (Tempo de CPU usado / Tempo total de vida) * 100 * Cores
        ((cpu_ticks.ticks() / clk_tck) / process_duration_seconds) * 100.0 * num_cores
    } else {
        0.0
    };
    Some(cpu_usage_pct)
}
