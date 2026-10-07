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

`make test` refreshes [`PROGRESS.md`](PROGRESS.md) in the repository root and prints the report. It includes every problem statement, the solution state, passed and failed test counts, and a PASS, FAIL, or NOT RUN result. Missing projects are marked `NOT STARTED`; answer directories without a `Cargo.toml` are marked `INCOMPLETE`.

The report file is visible before the first run, with every problem listed as NOT RUN. Each run replaces those initial states with current results. Tests continue after failures so the full report is available; the command exits unsuccessfully if any test fails or no projects can be tested.

## Work on an answer

Change into the generated project and use Cargo as usual:

```sh
cd answers/003-palindrome-number
cargo test
cargo run
```

Run `make help` to see the available Make targets. The project generator
requires Rust and Cargo to be installed.
