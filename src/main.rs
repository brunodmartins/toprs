mod os;
mod printer;

use crate::os::info::get_system_memory_kbs;
use crate::os::process::ProcessInfo;
use crate::os::process::read_processes;
use crate::printer::table::pretty_print_table;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    limit: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let processes_map = read_processes();
    let mut processes: Vec<ProcessInfo> = processes_map.into_values().collect();
    processes.sort_by_key(|p| std::cmp::Reverse(p.memory_res_kb));

    if args.limit != None {
        _ = processes.split_off(args.limit.unwrap());
    }
    pretty_print_table(processes, get_system_memory_kbs().unwrap());
}
