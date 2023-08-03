Rust has a number of features to handle errors. In most cases, Rust requires us to acknowledge the possibility of errors and take some precautions before the code will even compile. Rust groups errors into two categories; `recoverable` and `unrecoverable` errors.

For a recoverable error, such as file not found error, we most likely want the to inform the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array, and so we want to immediately stop the program.

Most languages do not have this distinction, and handle both in the same way, usually using try except blocks. Rust does not have exceptions. Instead, we use the type `Result<T,E>` for recoverable errors and `panic!` for unrecoverable errors.

The chapter includes three topics
- [Unrecoverable Errors with panic!](1.unrecoverable_errors_with_panic.md)
- [Recoverable Errors with Result](2.recoverable_errors_with_result.md)
- [To panic! or Not to panic!](3.to_panic_or_not_to_panic.md)
- [Error Handling Guidelins](4.error_handling_guidelines.md)