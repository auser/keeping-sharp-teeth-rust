# Keeping Sharp Teeth in Rust

A small repository for practicing Rust with one Cargo project per problem.

## Repository layout

- `problems/` contains the problem statements as Markdown files named
  `NNN_problem-name.md`.
- `answers/` contains the Rust binary crates created for those problems.
  Directory names keep the problem number, for example
  `003_palindrome_number.md` becomes `answers/003-palindrome-number/`. Cargo
  package names omit the numeric prefix (`palindrome-number`) because Cargo
  package names cannot start with a digit.

## Create an answer project

To create a project for the next problem that does not already have an answer
directory, run:

```sh
make answer
```

Problems are considered in numeric filename order. The command creates a
standard binary Cargo project in `answers/` and skips problems whose answer
directory already exists.

To choose a particular problem by its numeric prefix, run:

```sh
make answer PROBLEM=003
```

You can also pass the full problem filename stem:

```sh
make answer PROBLEM=003_palindrome_number
```

The explicit form selects a matching `problems/NNN_*.md` file and reports an
error if its answer project already exists.

## Run all answer tests

Run the tests for every Cargo project directly inside `answers/` with:

```sh
make test
```

`make test` runs tests for every answer crate and writes the latest results to [PROGRESS.md](PROGRESS.md). The report has one row for every problem statement, with its solution state, test counts, and result.

- `PASS` and `FAIL` show the test outcome.
- `NOT STARTED` means no answer project exists yet.
- `INCOMPLETE` means the answer directory has no `Cargo.toml`.
- `NOT RUN` means there was no test result to report.

Tests continue after failures so the full report is available. The command exits unsuccessfully if any test run fails or if no projects can be tested. `make test` refreshes `PROGRESS.md` in your working tree; commit it to share the results.

`make progress` is an alias for `make test`; it runs the tests and refreshes the same report.

A GitHub Actions workflow also runs `make progress` on pushes to `main` and commits an updated `PROGRESS.md` when the report changes.

## Work on an answer

Change into the generated project and use Cargo as usual:

```sh
cd answers/003-palindrome-number
cargo test
cargo run
```

Run `make help` to see the available Make targets. The project generator
requires Rust and Cargo to be installed.
