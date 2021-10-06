use std::fs;

/// The namesake struct `Logger` which provides all of the logging
/// capabilities.
/// 
/// # Examples
/// 
/// ```rs
/// let logger = Logger::new("my_app");
///
/// logger.info("no errors have been predicted");
/// logger.warn("an error has been predicted");
/// logger.error("an error occurred!");
/// 
/// logger.save_to_file("log.txt");
///
/// // prints:
/// // [my_app:INFO] no errors have been predicted
/// // [my_app:WARNING] an error has been predicted
/// // [my_app:ERROR] an error occurred!
///
/// // ...and writes all of it to log.txt
/// ```
pub struct Logger {
    /// The name of the logger.
    /// 
    /// This will be used for all 3 functions (info, warn, error):
    ///     `[name:INFO] text`,
    ///     `[name:WARN] text`,
    ///     `[name:ERROR] text`
    pub name: String,

    /// The log history, mainly so we can save it to a file.
    pub log_history: Vec<String>,
}

impl Logger {
    /// Creates a new `Logger`.
    /// 
    /// # Example
    /// 
    /// ```rs
    /// let logger = Logger::new("my_app");
    /// ```
    pub fn new(name: &str) -> Logger {
        Logger {
            name: name.to_owned(),
            log_history: Vec::new()
        }
    }

    /// Prints text as info.
    /// 
    /// # Format
    /// 
    /// The format for `info` is `[name:INFO] text`
    /// where the whole text is blue.
    pub fn info(&mut self, s: &str) {
        let log_text = format!("\x1b[0;34m[{}:INFO] {}", self.name, s);

        println!("{}", log_text);
        self.log_history.push(log_text.to_string());
    }

    /// Prints text as a warning.
    /// 
    /// # Format
    /// 
    /// The format for `warn` is `[name:WARNING] text`
    /// where the whole text is yellow.
    pub fn warn(&mut self, s: &str) {
        let log_text = format!("\x1b[0;33m[{}:WARNING] {}", self.name, s);

        println!("{}", log_text);
        self.log_history.push(log_text);
    }

    /// Prints text as an error.
    /// 
    /// # Format
    /// 
    /// The format for `error` is `[name:ERROR] text`
    /// where the whole text is red.
    pub fn error(&mut self, s: &str) {
        let log_text = format!("\x1b[0;31m[{}:ERROR] {}", self.name, s);

        println!("{}", log_text);
        self.log_history.push(log_text);
    }

    /// Saves the logs to a file.
    /// 
    /// # Example
    /// 
    /// ```rs
    /// logger.error("haha the program about to crash");
    /// logger.save_to_file("logs.txt");
    /// ```
    pub fn save_to_file(self, path: &str) {
        let fixed_log_history: Vec<String> = self.log_history
            .iter()
            .map(|x| (&x[7..x.len()]).to_string())
            .collect();

        fs::write(path, fixed_log_history.join("\n")).expect("failed saving logs to file");
    }
}