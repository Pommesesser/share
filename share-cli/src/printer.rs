use share_core::FileEntry;

pub fn print_link(server: &str, id: &str) {
    println!(
        "{}/files/{id}",
        server.strip_prefix("https://").unwrap_or(server)
    );
}

pub fn print_file_entries(file_entries: &[FileEntry]) {
    for file_entry in file_entries {
        println!(
            "{} | {} | {}",
            file_entry.id, file_entry.name, file_entry.size
        );
    }
}
