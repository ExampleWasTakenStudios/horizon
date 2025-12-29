#! /bin/bash
# Installs all necessary applications

set -e

# Install corepack
echo "Removing existing pnpm installs..."
npm remove -g pnpm

echo "Installing corepack from npm"
npm install -g corepack

# Install authbind
echo "Updating apt..."
sudo apt update -y

echo "Installing authbind..."
sudo apt install authbind -y

# Configure applications
./setup.sh
