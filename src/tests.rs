#[cfg(test)]
mod tests {
    use crate::logger::Logger;

    #[test]
    fn info() {
        let logger = Logger { name: String::from("") };
        logger.info("testing info");
    }

    #[test]
    fn warn() {
        let logger = Logger { name: String::from("") };
        logger.warn("testing warn");
    }

    #[test]
    fn error() {
        let logger = Logger { name: String::from("") };
        logger.error("testing error");
    }
}