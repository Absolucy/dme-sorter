#!/bin/bash
COSMO_VERSION="4.0.2"

set -xe
mkdir "/opt/cosmocc"
printf "set(CMAKE_ASM_OUTPUT_EXTENSION .o)\nset(CMAKE_C_OUTPUT_EXTENSION .o)\nset(CMAKE_CXX_OUTPUT_EXTENSION .o)" > /opt/cosmocc/cosmocc-override.cmake
wget -O "/opt/cosmocc/cosmocc.zip" "https://github.com/jart/cosmopolitan/releases/download/${COSMO_VERSION}/cosmocc-${COSMO_VERSION}.zip"
unzip "/opt/cosmocc/cosmocc.zip" -d "/opt/cosmocc"

chmod -R 777 "/opt/cosmocc"
chown -R "$(id -u)" "/opt/cosmocc"
