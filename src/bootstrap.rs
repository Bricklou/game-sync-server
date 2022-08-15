use std::{fs, path::Path};

const FOLDES_TO_CREATE: [&'static str; 2] = ["tmp/storage/images", "tmp/cache"];

pub async fn boot() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    for f in &FOLDES_TO_CREATE {
        let p = Path::new(env!("CARGO_MANIFEST_DIR")).join(f);
        if p.exists() {
            continue;
        }
        fs::create_dir_all(p)?;
    }

    Ok(())
}
