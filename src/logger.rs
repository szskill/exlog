pub struct Logger {
    /// The name of the logger.
    /// 
    /// This will be used for all 3 functions (info, warn, error):
    /// 
    ///     `[name:INFO] text`,
    ///     `[name:WARN] text`,
    ///     `[name:ERROR] text`
    pub name: String
}

impl Logger {
    /// Prints text as info.
    /// 
    /// # Format
    /// 
    /// The format for `info` is `[name:INFO] text`
    /// where the whole text is blue.
    pub fn info(self, s: &str) {
        println!("\x1b[0;34m[{}:INFO] {}", self.name, s);
    }

    /// Prints text as a warning.
    /// 
    /// # Format
    /// 
    /// The format for `warn` is `[name:WARNING] text`
    /// where the whole text is yellow.
    pub fn warn(self, s: &str) {
        println!("\x1b[0;33m[{}:WARNING] {}", self.name, s);
    }

    /// Prints text as an error.
    /// 
    /// # Format
    /// 
    /// The format for `error` is `[name:ERROR] text`
    /// where the whole text is red.
    pub fn error(self, s: &str) {
        println!("\x1b[0;31m[{}:ERROR] {}", self.name, s);
    }
}