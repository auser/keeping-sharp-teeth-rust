# Keeping Sharp Teeth in Rust

A small repository for practicing Rust with one Cargo project per problem,
collected in a Cargo workspace.

## Repository layout

- `problems/` contains the problem statements as Markdown files named
  `NNN_problem-name.md`.
- `answers/` contains the Rust binary crates created for those problems.
  Directory names keep the problem number, for example
  `003_palindrome_number.md` becomes `answers/003-palindrome-number/`. Cargo
  package names omit the numeric prefix (`palindrome-number`) because Cargo
  package names cannot start with a digit.
- `test-support/` contains the shared `test_cases!` macro. Every answer has it
  as a dev dependency; the helper has no external dependencies.
- `templates/answer.rs` is the starting source for new answers.

## Create an answer project

To create a project for the next problem that does not already have an answer
directory, run:

```sh
make answer
```

Problems are considered in numeric filename order. The command creates a
binary Cargo project in `answers/`, adds the shared test helper, and copies the
answer template. The template includes `struct Solution;` and an empty
`impl Solution {}`. Its test module imports `Solution` and `test_cases!`, with
an empty macro invocation ready for your cases. Problems whose answer
directory already exists are skipped.

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

## Write named test cases

Use `test_cases!` inside your answer's test module. For a function taking one
argument, write the function once, followed by named input and expected-output
pairs. For example, in `answers/023-climbing-stairs/src/main.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_cases;

    test_cases! {
        Solution::climb_stairs;
        single_step: 1 => 1,
        two_steps: 2 => 2,
        three_steps: 3 => 3,
        four_steps: 4 => 5,
        five_steps: 5 => 8,
    }
}
```

Each row becomes a separate `#[test]` function using `assert_eq!`, so Cargo
reports failures by case name. The helper accepts one or more arguments per
case, separated by commas. A parenthesized tuple still counts as one argument.

For a function that takes multiple arguments, list them in the same order as a
normal function call. For example, inside the two-sum answer's test module:

```rust
test_cases! {
    Solution::two_sum;
    first_pair: vec![2, 7, 11, 15], 9 => vec![0, 1],
    later_pair: vec![3, 2, 4], 6 => vec![1, 2],
}
```

Pass references explicitly when the function requires them. For
`remove_element(nums: &mut Vec<i32>, val: i32) -> i32`:

```rust
test_cases! {
    Solution::remove_element;
    simple_input: &mut vec![3, 2, 2, 3], 3 => 2,
}
```

Removing `3` leaves two elements, so the expected return value is `2`. This
case checks the returned count. To inspect the retained values as well, use a
setup block in the complete-expression form:

```rust
test_cases! {
    retained_values: {
        let mut nums = vec![3, 2, 2, 3];
        let remaining = Solution::remove_element(&mut nums, 3);
        let retained = nums[..remaining as usize].to_vec();
        (remaining, retained)
    } => (2, vec![2, 2]),
}
```

Only the first `remaining` values are part of this exercise's result.

Without a function before `;`, each expression before `=>` is compared
directly with its expected value. A complete function call also works there.
Both forms accept a trailing comma. Existing handwritten `#[test]` functions
can stay alongside the generated tests.

To run just one case, use Cargo's test filter from that answer's directory:

```sh
cd answers/023-climbing-stairs
cargo test single_step
```

## Run all answer tests

Run the shared helper's tests and every answer's tests with:

```sh
make test
```

`make test` prints a progress table with one row for every problem statement,
including its solution state, test counts, and result. The helper's tests run
separately and are not included in the problem totals.

- `✅ PASS` and `❌ FAIL` show the test outcome.
- `NOT STARTED` means no answer project exists yet.
- `INCOMPLETE` means the answer directory has no `Cargo.toml`.
- `⏸️ NOT RUN` means there was no test result to report.

Tests continue after failures, including a helper failure, so the full answer
report is available. The command exits unsuccessfully if any test run fails or
if no answer projects can be tested.

`make progress` is an alias for `make test`. Both commands leave the tracked
[PROGRESS.md](PROGRESS.md) unchanged during normal local runs.

A GitHub Actions workflow runs `UPDATE_PROGRESS=1 make progress` on pushes to
`main` and commits an updated `PROGRESS.md` when the report changes.

## Work on an answer

Change into the generated project and use Cargo as usual:

```sh
cd answers/003-palindrome-number
cargo test
cargo run
```

Run `make help` to see the available Make targets. The project generator
requires Rust and Cargo to be installed.
