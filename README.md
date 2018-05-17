# Rust bindings for M4RI

Work in progress. I'm probably going to expand this with a safe interface.

## WARNING
As far as I can tell, M4RI is not thread-safe and turns threading into a nightmare.
I have not yet figured out how to not break everything when using threads.

When running tests in programs that use this lib, use `--test-threads 1`.
