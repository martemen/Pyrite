use log::info;

const VERSION: &str = "0.1-dev";

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();


    info!("Engine Running, Version: {}", VERSION);
}
