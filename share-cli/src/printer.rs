use share_core::FileEntry;

const ID_WIDTH: usize = 15;
const NAME_WIDTH: usize = 30;
const SIZE_WIDTH: usize = 10;

pub fn print_link(server: &str, id: &str) {
    println!(
        "{}/files/{id}",
        server.strip_prefix("https://").unwrap_or(server)
    );
}

pub fn print_file_entries(file_entries: &[FileEntry]) {
    println!(
        "{:<ID_WIDTH$}  {:<NAME_WIDTH$}  {:>SIZE_WIDTH$}",
        "ID",
        "NAME",
        "SIZE",
        ID_WIDTH = ID_WIDTH,
        NAME_WIDTH = NAME_WIDTH,
        SIZE_WIDTH = SIZE_WIDTH,
    );

    println!("{}", "─".repeat(ID_WIDTH + 2 + NAME_WIDTH + 2 + SIZE_WIDTH));

    for file_entry in file_entries {
        println!(
            "{:<ID_WIDTH$}  {:<NAME_WIDTH$}  {:>SIZE_WIDTH$}",
            file_entry.id,
            truncate(&file_entry.name, NAME_WIDTH),
            format_size(file_entry.size),
            ID_WIDTH = ID_WIDTH,
            NAME_WIDTH = NAME_WIDTH,
            SIZE_WIDTH = SIZE_WIDTH,
        );
    }
}

fn truncate(value: &str, max_width: usize) -> String {
    if value.chars().count() <= max_width {
        return value.to_owned();
    }

    let truncated: String = value.chars().take(max_width - 1).collect();
    format!("{truncated}…")
}

fn format_size(bytes: i64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];

    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{:.1} {}", size, UNITS[unit])
    }
}
