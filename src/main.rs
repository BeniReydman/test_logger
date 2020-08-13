use log::{error, info, warn};
use log4rs;
 
fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
 
    info!("Booting up.");
 
    for i in 0..1000 {
        if i % 50 == 0 {
            error!("Random error");
        } else
        if i % 17 == 0 {
            warn!("Random warning");
        } else {
        info!("Loop: {}", i);
        }
    }
 
    info!("Shutting down.");
}