FROM rust:1.83

# Install GUI libraries (adjust based on your GUI framework)
RUN apt-get update && apt-get install -y \
    libgtk-3-dev \
    libx11-dev \
    libxcb1-dev \
    libxrandr-dev \
    libxi-dev \
    libxext-dev \
    libxrender-dev \
    zsh \
    && apt-get clean

# Create a non-root user for development
RUN useradd -m vscode \
    && usermod -aG sudo vscode

# Switch to the new user
USER vscode

RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# Set up Rust environment
ENV PATH="/home/vscode/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /workspaces/rs-minesweeper
