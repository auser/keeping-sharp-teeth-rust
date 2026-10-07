PROBLEMS := $(sort $(wildcard problems/[0-9]*_*.md))
ANSWER_DIR = answers/$(subst _,-,$(basename $(notdir $(1))))
UNANSWERED := $(foreach problem,$(PROBLEMS),$(if $(wildcard $(call ANSWER_DIR,$(problem))),,$(problem)))
NEXT_PROBLEM := $(firstword $(UNANSWERED))
PROBLEM_FILE = $(if $(PROBLEM),$(firstword $(wildcard problems/$(PROBLEM).md problems/$(PROBLEM)_*.md)),$(NEXT_PROBLEM))
ANSWER_MANIFESTS := $(wildcard answers/*/Cargo.toml)

.PHONY: help answer test

help:
	@printf '%s\n' 'Targets:' \
		'  make answer              Create the next unanswered project' \
		'  make answer PROBLEM=003  Create a project for problem 003' \
		'  make test                Run tests in every answer project'

answer:
	@set -eu; \
	problem_file='$(PROBLEM_FILE)'; \
	if [ -z "$$problem_file" ]; then \
		if [ -n "$(PROBLEM)" ]; then \
			printf 'Problem not found: %s\n' "$(PROBLEM)" >&2; \
		else \
			printf '%s\n' 'No unanswered problems found.' >&2; \
		fi; \
		exit 1; \
	fi; \
	answer_dir='$(call ANSWER_DIR,$(PROBLEM_FILE))'; \
	if [ -e "$$answer_dir" ]; then \
		printf 'Answer project already exists: %s\n' "$$answer_dir" >&2; \
		exit 1; \
	fi; \
	crate_name=$$(basename "$$answer_dir" | sed 's/^[0-9][0-9]*-//'); \
	cargo new --bin --name "$$crate_name" "$$answer_dir"; \
	printf 'Created %s\n' "$$answer_dir"

test:
	@set -eu; \
	if [ -z "$(ANSWER_MANIFESTS)" ]; then \
		printf '%s\n' 'No answer projects found in answers/.' >&2; \
		exit 1; \
	fi; \
	for manifest in $(ANSWER_MANIFESTS); do \
		printf '==> Testing %s\n' "$$manifest"; \
		cargo test --manifest-path "$$manifest"; \
	done
