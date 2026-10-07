.PHONY: help answer

help:
	@printf '%s\n' 'Targets:' '  make answer PROBLEM=003  Create answers/003-problem-name from problems/003_*.md'

answer:
	@set -eu; \
	problem='$(PROBLEM)'; \
	if [ -z "$$problem" ]; then \
		printf '%s\n' 'Usage: make answer PROBLEM=003' >&2; \
		exit 2; \
	fi; \
	problem_id=$$(printf '%s\n' "$$problem" | sed 's/_.*//'); \
	case "$$problem_id" in \
		*[!0-9]*|'') printf 'Invalid problem identifier: %s\n' "$$problem" >&2; exit 2 ;; \
	esac; \
	problem_file=$$(find problems -maxdepth 1 -type f -name "$$problem_id_*.md" -print -quit); \
	if [ -z "$$problem_file" ]; then \
		printf 'No problem found for %s in problems/\n' "$$problem_id" >&2; \
		exit 1; \
	fi; \
	stem=$$(basename "$$problem_file" .md); \
	package_name=$$(printf '%s\n' "$$stem" | tr '_' '-'); \
	answer_dir="answers/$$package_name"; \
	if [ -e "$$answer_dir" ]; then \
		printf 'Answer project already exists: %s\n' "$$answer_dir" >&2; \
		exit 1; \
	fi; \
	cargo new --bin --name "$$package_name" "$$answer_dir"; \
	printf 'Created %s\n' "$$answer_dir"
