use log::LevelFilter;
use env_logger::Builder;
use std::io::Write;

pub fn init_logger() {
  Builder::new()
    .format(|buf, record| {
      writeln!(
        buf,
        "[{}] - {}",
        record.level(),
        record.args()
      )
    })
    .filter(None, LevelFilter::Info)
    .init();
}