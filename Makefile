.PHONY: help answer test

help:
	@printf '%s\n' 'Targets:' '  make answer              Create the next unanswered project' '  make answer PROBLEM=003  Create answers/003-problem-name from problems/003_*.md'


test:
	@set -eu; \
	found=0; \
	for manifest in answers/*/Cargo.toml; do \
		[ -f "$$manifest" ] || continue; \
		found=1; \
		printf '==> Testing %s\n' "$$manifest"; \
		cargo test --manifest-path "$$manifest"; \
	done; \
	if [ "$$found" -eq 0 ]; then \
		printf '%s\n' 'No answer projects found in answers/.' >&2; \
		exit 1; \
	fi

answer:
	@set -eu; \
	problem='$(PROBLEM)'; \
	if [ -z "$$problem" ]; then \
		for problem_file in $$(find problems -maxdepth 1 -type f -name '[0-9]*_*.md' -print | sort); do \
			stem=$$(basename "$$problem_file" .md); \
			package_name=$$(printf '%s\n' "$$stem" | tr '_' '-'); \
			if [ ! -e "answers/$$package_name" ]; then \
				problem="$$stem"; \
				break; \
			fi; \
		done; \
		if [ -z "$$problem" ]; then \
			printf '%s\n' 'No unanswered problems found in problems/.' >&2; \
			exit 1; \
		fi; \
	fi; \
	problem_id=$$(printf '%s\n' "$$problem" | sed 's/_.*//'); \
	case "$$problem_id" in \
		*[!0-9]*|'') printf 'Invalid problem identifier: %s\n' "$$problem" >&2; exit 2 ;; \
	esac; \
	problem_file=$$(find problems -maxdepth 1 -type f -name "$${problem_id}_*.md" -print -quit); \
	if [ -z "$$problem_file" ]; then \
		printf 'No problem found for %s in problems/\n' "$$problem_id" >&2; \
		exit 1; \
	fi; \
	stem=$$(basename "$$problem_file" .md); \
	package_name=$$(printf '%s\n' "$$stem" | tr '_' '-'); \
	answer_dir="answers/$$package_name"; \
	crate_name=$$(printf '%s\n' "$$package_name" | sed 's/^[0-9][0-9]*-//'); \
	if [ -z "$$crate_name" ]; then \
		printf 'Could not derive a Cargo package name from %s\n' "$$package_name" >&2; \
		exit 1; \
	fi; \
	if [ -e "$$answer_dir" ]; then \
		printf 'Answer project already exists: %s\n' "$$answer_dir" >&2; \
		exit 1; \
	fi; \
	cargo new --bin --name "$$crate_name" "$$answer_dir"; \
	printf 'Created %s (Cargo package: %s)\n' "$$answer_dir" "$$crate_name"
