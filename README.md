# exlog
## a very simple logger for small projects

### Install
Install `exlog` by adding this line to your `Cargo.toml`:  
`exlog = "0.0.1"`

### Example:
```rs
let logger = Logger::new("my_app");

logger.info("no errors have been predicted");
logger.warn("an error has been predicted");
logger.error("an error occurred!");

logger.save_to_file("log.txt");

// prints:
// [my_app:INFO] no errors have been predicted
// [my_app:WARNING] an error has been predicted
// [my_app:ERROR] an error occurred!

// ...and writes all of it to log.txt
```
