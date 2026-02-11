#!/bin/bash
# Release helper script
set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Get version from Cargo.toml
CURRENT_VERSION=$(grep '^version' crates/lob-cli/Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

echo -e "${GREEN}Current version: ${CURRENT_VERSION}${NC}"
echo ""
echo "What type of release is this?"
echo "1) Patch (0.1.0 -> 0.1.1)"
echo "2) Minor (0.1.0 -> 0.2.0)"
echo "3) Major (0.1.0 -> 1.0.0)"
echo "4) Custom"
read -p "Enter choice [1-4]: " choice

case $choice in
    1)
        NEW_VERSION=$(echo $CURRENT_VERSION | awk -F. '{$NF = $NF + 1;} 1' | sed 's/ /./g')
        ;;
    2)
        NEW_VERSION=$(echo $CURRENT_VERSION | awk -F. '{$2 = $2 + 1; $3 = 0;} 1' | sed 's/ /./g')
        ;;
    3)
        NEW_VERSION=$(echo $CURRENT_VERSION | awk -F. '{$1 = $1 + 1; $2 = 0; $3 = 0;} 1' | sed 's/ /./g')
        ;;
    4)
        read -p "Enter new version: " NEW_VERSION
        ;;
    *)
        echo -e "${RED}Invalid choice${NC}"
        exit 1
        ;;
esac

echo -e "${YELLOW}Releasing version: ${NEW_VERSION}${NC}"
read -p "Continue? [y/N] " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]
then
    exit 1
fi

# Pre-release checks
echo -e "${GREEN}Running pre-release checks...${NC}"

echo "  → Checking working directory is clean..."
if [ -n "$(git status --porcelain)" ]; then
    echo -e "${RED}Error: Working directory not clean${NC}"
    git status --short
    exit 1
fi

echo "  → Running tests..."
cargo test --workspace --quiet || exit 1

echo "  → Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings || exit 1

echo "  → Checking formatting..."
cargo fmt -- --check || exit 1

echo "  → Checking for security vulnerabilities..."
if command -v cargo-audit &> /dev/null; then
    cargo audit || echo -e "${YELLOW}Warning: Security audit found issues${NC}"
fi

# Update versions
echo -e "${GREEN}Updating version numbers...${NC}"

# Update all Cargo.toml files
sed -i.bak "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" crates/lob-cli/Cargo.toml
sed -i.bak "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" crates/lob-core/Cargo.toml
sed -i.bak "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" crates/lob-prelude/Cargo.toml

# Update npm package.json
if [ -f npm/package.json ]; then
    sed -i.bak "s/\"version\": \"$CURRENT_VERSION\"/\"version\": \"$NEW_VERSION\"/" npm/package.json
fi

# Update Homebrew formula
if [ -f homebrew/lob.rb ]; then
    sed -i.bak "s/version \"$CURRENT_VERSION\"/version \"$NEW_VERSION\"/" homebrew/lob.rb
fi

# Clean up backup files
find . -name "*.bak" -delete

# Update Cargo.lock
cargo check --quiet

echo -e "${GREEN}Updated version to ${NEW_VERSION}${NC}"

# Prompt for changelog
echo ""
echo -e "${YELLOW}Please update CHANGELOG.md before committing${NC}"
read -p "Press enter when done..."

# Git commit and tag
echo -e "${GREEN}Creating git commit and tag...${NC}"
git add -A
git commit -m "chore: release v${NEW_VERSION}"
git tag -a "v${NEW_VERSION}" -m "Release v${NEW_VERSION}"

echo ""
echo -e "${GREEN}Release prepared!${NC}"
echo ""
echo "Next steps:"
echo "1. Review the changes: git show HEAD"
echo "2. Push to GitHub: git push origin main --tags"
echo "3. GitHub Actions will automatically build and publish binaries"
echo ""
echo "To cancel:"
echo "  git tag -d v${NEW_VERSION}"
echo "  git reset --hard HEAD~1"
