use ctr::Logger;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOGGER: Logger = Logger::new("/pkrd-logs.txt");
}

pub fn debug(text: &str) {
    LOGGER.debug(text)
}

pub fn error(text: &str) {
    LOGGER.error(text)
}
