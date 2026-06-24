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

pub fn format_cpu_display(cpu_usage_pct: f64) -> String {
    return format!("{:.2}%", cpu_usage_pct);
}

#[cfg(test)]
mod tests {
    use crate::printer::formatter::{format_cpu_display, format_mem_display};

    #[test]
    fn test_format_mem_display() {
        const AVAILABLE_MEMORY: u64 = 2048 * 1024;
        assert_eq!(format_mem_display(0, AVAILABLE_MEMORY), "0 KB".to_string());
        assert_eq!(
            format_mem_display(1, AVAILABLE_MEMORY),
            "1 KB (0.00%)".to_string()
        );
        assert_eq!(
            format_mem_display(1024, AVAILABLE_MEMORY),
            "1 MB (0.05%)".to_string()
        );
        assert_eq!(
            format_mem_display(1024 * 5, AVAILABLE_MEMORY),
            "5 MB (0.24%)".to_string()
        );
        assert_eq!(
            format_mem_display(1024 * 1024, AVAILABLE_MEMORY),
            "1 GB (50.00%)".to_string()
        );
        assert_eq!(
            format_mem_display(AVAILABLE_MEMORY, AVAILABLE_MEMORY),
            "2 GB (100.00%)".to_string()
        );
    }

    #[test]
    fn test_format_cpu_display() {
        assert_eq!(format_cpu_display(44.4), "44.40%".to_string());
        assert_eq!(format_cpu_display(16.395), "16.39%".to_string());
        assert_eq!(format_cpu_display(0.0), "0.00%".to_string());
    }
}
