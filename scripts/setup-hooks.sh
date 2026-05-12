#!/bin/bash
set -e

HOOK_PATH=".git/hooks/pre-commit"

echo "Installing git hooks..."

cat <<EOF > "$HOOK_PATH"
#!/bin/bash
set -e

echo "Running pre-commit checks..."

# Run linting via mise
# Check if mise is installed, otherwise try running the command directly
if command -v mise &> /dev/null; then
    mise run lint
else
    cargo clippy -- -D warnings && cargo fmt --check
fi

echo "Pre-commit checks passed!"
EOF

chmod +x "$HOOK_PATH"

echo "Hooks installed successfully at $HOOK_PATH"
