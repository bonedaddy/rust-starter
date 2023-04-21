.PHONY: fmt
fmt:
	( find . -type f -name "*.rs" -not -path *target* -exec rustfmt --edition 2021 {} \; )

lint:
	cargo +nightly clippy --fix -Z unstable-options --release --all --broken-code

.PHONY: do-lint
do-lint: lint