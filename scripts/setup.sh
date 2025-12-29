#! /bin/bash
# Configures all necessary applications to be used in Horizon's development.

set -e

PNPM_VERSION="pnpm@latest-10"

PORT="53"
PORT_DIR="/etc/authbind/byport"
PORT_FILE="$PORT_DIR/$PORT"

# Configure corepack to use pnpm
echo "Enabling pnpm in corepack"
corepack enable pnpm

echo "Pinning pnpm to $PNPM_VERSION"
corepack use $PNPM_VERSION

# Configure authbind
echo "Configuring authbind..."

echo "Creating authbind directory..."
sudo mkdir -p $PORT_DIR

echo "Creating authbind file..."
sudo touch $PORT_FILE

# Configure authbind permissions
echo "Configuring authbind permissions..."
REAL_USER=${SUDO_USER:-$(whoami)}
sudo chown $REAL_USER $PORT_FILE
sudo chmod 744 $PORT_FILE
