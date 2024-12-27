#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Installing aineed...${NC}"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo is not installed. Please install Rust and cargo first.${NC}"
    echo "Visit https://rustup.rs/ for installation instructions."
    exit 1
fi

# Build the release binary
echo -e "${YELLOW}Building release binary...${NC}"
cargo build --release
if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Failed to build aineed${NC}"
    exit 1
fi

# Create ~/.local/bin if it doesn't exist
mkdir -p ~/.local/bin

# Copy binary to ~/.local/bin
echo -e "${YELLOW}Installing binary to ~/.local/bin...${NC}"
cp target/release/aineed ~/.local/bin/
if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Failed to copy binary${NC}"
    exit 1
fi

# Make binary executable
chmod +x ~/.local/bin/aineed

# Function to add path to shell config if not present
add_to_path() {
    local config_file=$1
    local path_line='export PATH="$HOME/.local/bin:$PATH"'
    
    if [ -f "$config_file" ]; then
        if ! grep -q "$path_line" "$config_file"; then
            echo "" >> "$config_file"
            echo "# Added by aineed installer" >> "$config_file"
            echo "$path_line" >> "$config_file"
            return 0
        fi
    else
        echo "$path_line" > "$config_file"
        return 0
    fi
    return 1
}

# Detect shell and update appropriate config file
echo -e "${YELLOW}Configuring shell...${NC}"

# Check for zsh first (default on macOS)
if [ -f "$HOME/.zshrc" ] || [ "$SHELL" = "/bin/zsh" ] || [ "$SHELL" = "/usr/bin/zsh" ]; then
    add_to_path "$HOME/.zshrc"
    echo -e "${GREEN}Added to .zshrc${NC}"
elif [ -f "$HOME/.bashrc" ] || [ "$SHELL" = "/bin/bash" ] || [ "$SHELL" = "/usr/bin/bash" ]; then
    add_to_path "$HOME/.bashrc"
    echo -e "${GREEN}Added to .bashrc${NC}"
elif [ -f "$HOME/.config/fish/config.fish" ] || [ "$SHELL" = "/usr/bin/fish" ] || [ "$SHELL" = "/bin/fish" ]; then
    # Fish has a different path setting mechanism
    fish -c "set -U fish_user_paths $HOME/.local/bin \$fish_user_paths"
    echo -e "${GREEN}Added to fish configuration${NC}"
else
    echo -e "${YELLOW}Unknown shell configuration. Please manually add ~/.local/bin to your PATH${NC}"
    echo -e "${YELLOW}Add this line to your shell's config file:${NC}"
    echo -e "${GREEN}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
fi

echo -e "${GREEN}Installation complete!${NC}"
echo -e "${YELLOW}Please restart your shell or run:${NC}"
echo -e "    source ~/.zshrc   ${GREEN}# for zsh${NC}"
echo -e "    source ~/.bashrc  ${GREEN}# for bash${NC}"
echo -e "    exec fish        ${GREEN}# for fish${NC}"
echo ""
echo -e "${GREEN}You can now use 'aineed' from any directory.${NC}" 