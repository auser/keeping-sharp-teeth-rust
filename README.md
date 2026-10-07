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

The report includes a row for every problem statement. It runs every available
answer project, records passed and failed test counts, and continues after a
failure so you get a complete report. Problems without an answer project show
`NOT STARTED`; answer directories without a `Cargo.toml` show `INCOMPLETE`.
The command exits unsuccessfully if any test run fails or if no answer projects
can be tested.

Example output (counts and states are illustrative):

| Problem | Solution | Tests | Result |
| --- | --- | --- | --- |
| 001-two-sum | SOLVED | 5 passed, 0 failed | PASS |
| 002-reverse-string | SOLVED | 4 passed, 0 failed | PASS |
| 003-palindrome-number | SOLVED | 3 passed, 1 failed | FAIL |
| 004-fizzbuzz | SOLVED | 2 passed, 0 failed | PASS |
| 005-fibonacci-number | SOLVED | 4 passed, 0 failed | PASS |
| 006-factorial | NOT STARTED | — | NOT RUN |
| 007-count-digits | INCOMPLETE | — | NOT RUN |
| 008-sum-of-array | SOLVED | no test summary | FAIL |
| 009-find-maximum | SOLVED | 1 passed, 0 failed | PASS |

Solutions passing: 5/9 problems (7 tested)
Tests: 19 passed, 1 failed

## Work on an answer

Change into the generated project and use Cargo as usual:

```sh
cd answers/003-palindrome-number
cargo test
cargo run
```

Run `make help` to see the available Make targets. The project generator
requires Rust and Cargo to be installed.
