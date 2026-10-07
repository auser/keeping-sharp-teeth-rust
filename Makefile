.PHONY: help answer

help:
	@printf '%s\n' 'Targets:' '  make answer             Create the next unanswered problem' '  make answer PROBLEM=003  Create a specific problem'

answer:
	@set -eu; \
	problem='$(PROBLEM)'; \
	if [ -z "$$problem" ]; then \
		for candidate in problems/[0-9]*.md; do \
			[ -f "$$candidate" ] || continue; \
			stem=$$(basename "$$candidate" .md); \
			package_name=$$(printf '%s\n' "$$stem" | tr '_' '-'); \
			if [ ! -e "answers/$$package_name" ]; then \
				problem_file="$$candidate"; \
				break; \
			fi; \
		done; \
		if [ -z "$${problem_file:-}" ]; then \
			printf '%s\n' 'Every problem already has an answer project.'; \
			exit 0; \
		fi; \
	else \
		problem_id=$$(printf '%s\n' "$$problem" | sed 's/_.*//'); \
		case "$$problem_id" in \
			*[!0-9]*|'') printf 'Invalid problem identifier: %s\n' "$$problem" >&2; exit 2 ;; \
		esac; \
		problem_file=$$(find problems -maxdepth 1 -type f -name "$${problem_id}_*.md" -print -quit); \
		if [ -z "$$problem_file" ]; then \
			printf 'No problem found for %s in problems/\n' "$$problem_id" >&2; \
			exit 1; \
		fi; \
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
