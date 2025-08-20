# Multi-stage Dockerfile for TrueOrigin ICP project
FROM node:18-alpine AS frontend-builder

# Install pnpm
RUN npm install -g pnpm

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json pnpm-*.yaml ./
COPY src/frontend/package*.json ./src/frontend/

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy source code
COPY . .

# Build frontend
RUN cd src/frontend && pnpm build

# Development stage for testing
FROM ubuntu:22.04 AS development

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js and pnpm
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g pnpm

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Add wasm target for ICP
RUN rustup target add wasm32-unknown-unknown

# Install DFX non-interactively
RUN DFXVM_INIT_YES=true sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
ENV PATH="/root/.local/share/dfx/bin:${PATH}"

WORKDIR /app

# Copy project files
COPY . .

# Install node dependencies
RUN pnpm install --frozen-lockfile

# Expose ports
EXPOSE 4943 8080

# Default command
CMD ["bash"]