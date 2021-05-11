# ok

The [.ok](../.ok) file within the root directory is a file which is used to run various shell
commands in order, usually for testing.

## Current configuration (outdated)
1. Lint and fix fixable linting warnings based on the [rustfmt.toml](../rustfmt.toml) file
2. Check for lint warnings and display
3. Check for any warnings and compilation errors

## Usage

### Prerequisites

An ok-file executor, I recommend [ok](https://github.com/juev/ok).

P.S.
Bonus points for being written in Rust!

### Executing
```shell
$ ok # In the root directory of this repository
```
