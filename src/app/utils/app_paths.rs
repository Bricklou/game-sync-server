pub fn app_data() -> String {
    return concat!(env!("CARGO_MANIFEST_DIR"), "/tmp").to_string();
}

pub fn storage_path() -> String {
    return format!("{}/{}", app_data(), "storage");
}

pub fn cache_path() -> String {
    return format!("{}/{}", app_data(), "cache");
}
