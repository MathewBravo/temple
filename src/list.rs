use tracing::{debug, info, warn};

pub fn list_templates() {
    println!("listing templates");

    info!("coming from inside the list templates");

    warn!("listing");
    debug!("listing");
}
