# exlog
## a very simple logger for small projects

### Examples:
```rs
let logger = Logger { name: String::from("my_app") };

logger.info("no errors have been predicted");
logger.warn("an error has been predicted");
logger.error("an error occurred!");

// prints:
// [my_app:INFO] no errors have been predicted
// [my_app:WARNING] an error has been predicted
// [my_app:ERROR] an error occurred!
```
