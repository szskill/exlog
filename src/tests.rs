#[cfg(test)]
mod tests {
    use crate::logger::Logger;
    use std::fs;

    #[test]
    fn info() {
        let mut logger = Logger::new("tests");
        logger.info("testing info");
    }

    #[test]
    fn warn() {
        let mut logger = Logger::new("tests");
        logger.warn("testing warn");
    }

    #[test]
    fn error() {
        let mut logger = Logger::new("tests");
        logger.error("testing error");
    }

    #[test]
    fn save_to_file() {
        let mut logger = Logger::new("tests");

        logger.info("testing");
        logger.warn("test warning?");
        logger.error("test error!!!");
        logger.save_to_file("test_log_file.txt");

        assert_eq!(fs::read_to_string("test_log_file.txt").expect("failed reading log file"), "[tests:INFO] testing\n[tests:WARNING] test warning?\n[tests:ERROR] test error!!!");
    }
}