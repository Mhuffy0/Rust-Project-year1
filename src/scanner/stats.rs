use std::path::PathBuf;
use std::io;
use super::utils::human_readable_size;

pub fn drive_usage(_path: &PathBuf) -> (Vec<String>, f64) {
    let mut output = Vec::new();
    let mut used_percentage = 0.0;

    if let Ok(total_space) = get_total_disk_space() {
        if let Ok(free_space) = get_free_disk_space() {
            let used_space = total_space - free_space;
            used_percentage = (used_space as f64 / total_space as f64) * 100.0;
            let free_percentage = 100.0 - used_percentage;

            // vec
            output.push("Drive Usage:".to_string());
            output.push(format!("Total Space: {}", human_readable_size(total_space)));
            output.push(format!(
                "Used Space: {} ({:.2}%)",
                human_readable_size(used_space),
                used_percentage
            ));
            output.push(format!(
                "Free Space: {} ({:.2}%)",
                human_readable_size(free_space),
                free_percentage
            ));
        } else {
            output.push("Could not retrieve free space information.".to_string());
        }
    } else {
        output.push("Could not retrieve total space information.".to_string());
    }

    (output, used_percentage)
}

fn get_total_disk_space() -> Result<u64, io::Error> {
    Ok(1 << 40) 
}

fn get_free_disk_space() -> Result<u64, io::Error> {
    Ok(500 * (1 << 30))
}

