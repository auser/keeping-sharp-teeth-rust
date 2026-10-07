#!/bin/sh
set -u

total=0
for problem_file in problems/[0-9]*_*.md; do
    [ -f "$problem_file" ] || continue
    total=$((total + 1))
done

if [ "$total" -eq 0 ]; then
    printf '%s\n' 'No problem statements found in problems/.' >&2
    exit 1
fi

tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/answer-tests.XXXXXX") || exit 1
progress_tmp=".PROGRESS.md.tmp.$$"
trap 'rm -f "$progress_tmp"; rm -rf "$tmp_dir"' 0
trap 'exit 1' HUP INT TERM
rows="$tmp_dir/rows"
: > "$rows"

problem_number=0
solutions_passing=0
solutions_tested=0
tests_passed=0
tests_failed=0

for problem_file in problems/[0-9]*_*.md; do
    [ -f "$problem_file" ] || continue
    problem_number=$((problem_number + 1))

    filename=${problem_file##*/}
    filename=${filename%.md}
    problem=$(printf '%s' "$filename" | tr '_' '-')
    answer_dir="answers/$problem"
    manifest="$answer_dir/Cargo.toml"

    if [ ! -f "$manifest" ]; then
        if [ -d "$answer_dir" ]; then
            solution='INCOMPLETE'
        else
            solution='NOT STARTED'
        fi
        printf '[%d/%d] %s: %s\n' "$problem_number" "$total" "$problem" "$solution"
        printf '%s\t%s\t%s\t%s\n' "$problem" "$solution" '—' '⏸️ NOT RUN' >> "$rows"
        continue
    fi

    solutions_tested=$((solutions_tested + 1))
    log="$tmp_dir/test-$problem.log"
    printf '[%d/%d] Testing %s\n' "$problem_number" "$total" "$problem"

    if CARGO_TERM_COLOR=never cargo test --manifest-path "$manifest" > "$log" 2>&1; then
        result='✅ PASS'
        solutions_passing=$((solutions_passing + 1))
    else
        result='❌ FAIL'
        cat "$log"
    fi

    counts=$(awk '
        /^test result:/ {
            found = 1
            for (i = 1; i < NF; i++) {
                if ($(i + 1) == "passed;") passed += $i
                if ($(i + 1) == "failed;") failed += $i
            }
        }
        END { printf "%d %d %d\n", passed, failed, found }
    ' "$log")
    read -r project_passed project_failed has_summary <<EOF
$counts
EOF

    if [ "$has_summary" -eq 1 ]; then
        tests_passed=$((tests_passed + project_passed))
        tests_failed=$((tests_failed + project_failed))
        test_summary="$project_passed passed, $project_failed failed"
    else
        test_summary='no test summary'
    fi
    printf '%s\t%s\t%s\t%s\n' "$problem" 'SOLVED' "$test_summary" "$result" >> "$rows"
done

printf '\n%-28s %-12s %-24s %s\n' 'Problem' 'Solution' 'Tests' 'Result'
printf '%-28s %-12s %-24s %s\n' '----------------------------' '------------' '------------------------' '------'
while IFS="$(printf '\t')" read -r problem solution test_summary result; do
    printf '%-28s %-12s %-24s %s\n' "$problem" "$solution" "$test_summary" "$result"
done < "$rows"

printf '\nSolutions passing: %d/%d problems (%d tested)\n' \
    "$solutions_passing" "$total" "$solutions_tested"
printf 'Tests: %d passed, %d failed\n' "$tests_passed" "$tests_failed"


{
    printf '# Problem Progress\n\n'
    printf '_Updated after make progress on %s._\n\n' "$(date '+%Y-%m-%d %H:%M:%S %Z')"
    printf '| Problem | Solution | Tests | Result |\n'
    printf '| --- | --- | --- | --- |\n'
    while IFS="$(printf '\t')" read -r problem solution test_summary result; do
        printf '| %s | %s | %s | %s |\n' "$problem" "$solution" "$test_summary" "$result"
    done < "$rows"
    printf '\n**Solutions passing:** %d/%d problems (%d tested)\n\n' "$solutions_passing" "$total" "$solutions_tested"
    printf '**Tests:** %d passed, %d failed\n' "$tests_passed" "$tests_failed"
} > "$progress_tmp"
mv "$progress_tmp" PROGRESS.md
printf '\nUpdated PROGRESS.md\n'

[ "$solutions_tested" -gt 0 ] && [ "$tests_failed" -eq 0 ] && [ "$solutions_passing" -eq "$solutions_tested" ]
