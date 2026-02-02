#!/bin/bash

# cd into out/
cd out

# Create Horizon directory
mkdir Horizon
mkdir Horizon/bin

# Copy executable to dist
cp horizon.js Horizon/bin/horizon.js

# Copy Horizon's license to dist
cp ../LICENSE Horizon/LICENSE

# Copy third party licenses to dist
cp third-party-licenses.txt Horizon/third-party-licenses.txt

# Create zip archive
zip -r horizon.zip Horizon
