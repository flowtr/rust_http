# rust-client

A fast, opinionated command line HTTP client.

Fundamentally, `rust-client` is a thin wrapper around Rust's fantastic `reqwest` library. Unlike `curl`, however, it is designed more as a debugging tool. Headers are displayed above the response body, the command line interface is more intuitive than remembering flags, and default en-/decoding behavior.

## Quickstart

```sh
cargo install --git https://github.com/flowtr/rust_http
```

## Performance

TODO: add hyperfine benchmarks

## Contributing

For new features and bug reports, please open an issue on [GitLab](https://gitlab.com/rakenodiax/rust-client/issues/new?issue%5Bassignee_id%5D=&issue%5Bmilestone_id%5D=) or [Github](https://github.com/rakenodiax/rust-client/issues/new).

## Code of Conduct

This project is governed by its [code of conduct](code_of_conduct.md), which all participants (maintainers, contributors, and commenters) must follow.

## TODO

- [x] Have `Command` include request body from `RunConfig`
- [ ] Add `JSON` and `Form` arguments which automatically set headers and serialize appropriately
- [ ] Allow adding arbitrary headers to request
- [ ] Add documentation to major types and functions
- [ ] Flag to disable ansi escaping for output
- [ ] HTML pretty printing
- [ ] encode body content in a specific format (JSON, YAML, etc)
- [ ] decode response based on Content-Type
