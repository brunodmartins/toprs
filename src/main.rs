mod os;
mod printer;

use crate::os::info::get_system_memory_kbs;
use crate::os::process::ProcessInfo;
use crate::os::process::read_processes;
use crate::printer::table::pretty_print_table;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "5")]
    limit: Option<usize>,

    #[arg(long, value_enum, default_value_t = SortBy::RAM)]
    sort_by: SortBy,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum SortBy {
    RAM,
    CPU,
}


fn main() {
    let args = Args::parse();
    let processes_map = read_processes();
    let mut processes: Vec<ProcessInfo> = processes_map.into_values().collect();

    if args.sort_by == SortBy::RAM {
        processes.sort_by_key(|p| std::cmp::Reverse(p.memory_res_kb));
    } else {
        processes.sort_by_key(|p| std::cmp::Reverse(p.cpu_usage_avg_last_second_pct as u64));

    }

    if args.limit != None  && args.limit.unwrap() < processes.len() {
        _ = processes.split_off(args.limit.unwrap());
    }
    pretty_print_table(processes, get_system_memory_kbs().unwrap());
}
