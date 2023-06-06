# Installation prefix.
export PREFIX := "/usr/local"
# Shell completions directories.
export ZSH_COMP_DIR := "/usr/share/zsh/site-functions"
export BASH_COMP_DIR := "/usr/share/bash-completion/completions"
export FISH_COMP_DIR := "/usr/share/fish/vendor_completions.d"

default: build

# Run tests for schoenerd.
test:
	@echo "Running schoenerd tests..."
	cargo test --all

# Build schoenerd in release mode.
build:
	@echo "Building schoenerd in release mode..."
	cargo build --release

# Build schoenerd in debug mode.
build-debug:
	@echo "Building schoenerd in debug mode..."
	cargo build

# Clean schoenerd build artifacts.
clean:
	@echo "Cleaning schoenerd build artifacts..."
	cargo clean
	rm -rf completions man

# Install schoenerd.
[linux]
install: build && install-completions install-manpages
	@echo "Installing schoenerd using '$PREFIX' installation prefix..."
	mkdir -p ${PREFIX}/bin
	cp -f target/release/schoenerd ${PREFIX}/bin
	chmod 755 ${PREFIX}/bin/schoenerd

# Install shell completions for schoenerd.
[linux, private]
install-completions:
	@echo "Installing shell completions for Zsh, Bash & Fish shells..."
	mkdir -p ${ZSH_COMP_DIR}
	cp -f completions/_schoenerd ${ZSH_COMP_DIR}
	chmod 644 ${ZSH_COMP_DIR}/_schoenerd
	mkdir -p ${BASH_COMP_DIR}
	cp -f completions/schoenerd.bash ${BASH_COMP_DIR}
	chmod 644 ${BASH_COMP_DIR}/schoenerd.bash
	mkdir -p ${FISH_COMP_DIR}
	cp -f completions/schoenerd.fish ${FISH_COMP_DIR}
	chmod 644 ${FISH_COMP_DIR}/schoenerd.fish

# Install man pages for schoenerd.
[linux, private]
install-manpages:
	@echo "Installing man pages..."
	mkdir -p ${PREFIX}/man/man1
	cp -f man/schoenerd.1 ${PREFIX}/man/man1
	chmod 644 ${PREFIX}/man/man1/schoenerd.1
	
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

# Count project's source lines of code.
sloc:
	@echo "`wc -l src/*.rs` lines of code"