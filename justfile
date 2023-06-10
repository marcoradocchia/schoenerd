# Installation prefix.
export PREFIX := "/usr/local"
# Shell completions directories.
export ZSH_COMP_DIR := "/usr/share/zsh/site-functions"
export BASH_COMP_DIR := "/usr/share/bash-completion/completions"
export FISH_COMP_DIR := "/usr/share/fish/vendor_completions.d"

pkgver := `printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"`

default: build

# Run tests for schoenerd.
test:
	@echo "Running schoenerd tests..."
	cargo test --all --all-features

# Build schoenerd in release mode.
build:
	@echo "Building schoenerd in release mode..."
	sed -i "s/^\(\s*\)version,/\1version = \"{{pkgver}}\",/; s/v\({version}\)/\1/" ./src/cli.rs
	cargo build --release --all-features
	sed -i "s/^\(\s*version\) = \".*\",/\1,/; s/\({version}\)/v\1/" ./src/cli.rs

# Build schoenerd in debug mode.
build-debug:
	@echo "Building schoenerd in debug mode..."
	cargo build --all-features

# Clean schoenerd build artifacts.
clean:
	@echo "Cleaning schoenerd build artifacts..."
	cargo clean
	rm -f schoenerd.tar.gz

# Install schoenerd.
[linux]
install: && install-completions install-manpages
	@echo "Installing schoenerd using '$PREFIX' installation prefix..."
	install -Dm755 ./target/release/schoenerd "${PREFIX}/bin/schoenerd"

# Install shell completions for schoenerd.
[linux, private]
install-completions:
	@echo "Installing shell completions for Zsh, Bash & Fish shells..."
	install -Dm644 ./target/release/build/schoenerd-*/out/completions/_schoenerd "${ZSH_COMP_DIR}/_schoenerd"
	install -Dm644 ./target/release/build/schoenerd-*/out/completions/schoenerd.bash "${BASH_COMP_DIR}/schoenerd.bash"
	install -Dm644 ./target/release/build/schoenerd-*/out/completions/schoenerd.fish "${FISH_COMP_DIR}/schoenerd.fish"

# Install man pages for schoenerd.
[linux, private]
install-manpages:
	@echo "Installing man pages..."
	install -Dm644 ./target/release/build/schoenerd-*/out/man/schoenerd.1 "${PREFIX}/man/man1/schoenerd.1"
	
# Uninstall schoenerd.
[linux]
uninstall: && uninstall-completions uninstall-manpages
	@echo "Uninstalling schoenerd..."
	rm -f ${PREFIX}/bin/schoenerd

# Uninstall shell completions for schoenerd.
[linux, private]
uninstall-completions:
	@echo "Uninstalling shell completions..."
	rm -f ${ZSH_COMP_DIR}/_schoenerd
	rm -f ${BASH_COMP_DIR}/schoenerd.bash
	rm -f ${FISH_COMP_DIR}/schoenerd.fish

# Uninstall man pages for schoenerd.
[linux, private]
uninstall-manpages:
	@echo "Uninstalling man pages..."
	rm -f ${PREFIX}/man/man1/schoenerd.1

# Create tar archive.
[private]
archive:
	tar -czf schoenerd.tar.gz \
		LICENSE \
		README.md \
		-C ./target/release/build/schoenerd-*/out completions man \
		-C ../../.. schoenerd

# Build and package git version in schoenerd tar archive.
package: build && archive
	@echo "Packaging git version of schoenerd..."

# Build and package release version in schoenerd tar archive.
package-release: && archive
	@echo "Building schoenerd in release mode..."
	cargo build --release --all-features
	@echo "Packaging release version of schoenerd..."

# Count project's source lines of code.
sloc:
	@echo "`wc -l src/*.rs` lines of code"