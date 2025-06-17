#!/bin/bash
W2C2_COMMIT="06022f2ff69bdc7d0514a49a18004440a03c3a61"

set -xe
git clone "https://github.com/turbolent/w2c2" "/opt/w2c2"
cd "/opt/w2c2"
git checkout "${W2C2_COMMIT}"

chmod -R 777 "/opt/w2c2"
chown -R "$(id -u)" "/opt/w2c2"

export PATH="/opt/cosmocc/bin:${PATH}"
export ASM="cosmocc"
export CC="cosmocc"
export CXX="cosmoc++"

cd "/opt/w2c2/w2c2"
cmake \
	-DCMAKE_SYSTEM_NAME="Generic" \
	-UCMAKE_SYSTEM_PROCESSOR \
	-DCMAKE_USER_MAKE_RULES_OVERRIDE="/opt/cosmocc/cosmocc-override.cmake" \
	-DCMAKE_AR="$(command -v cosmoar)" \
	-DCMAKE_RANLIB="$(command -v cosmoranlib)" \
	-DCMAKE_BUILD_TYPE="MinSizeRel" \
	-B build
cmake --build build

cd "/opt/w2c2/wasi"
cmake \
	-DCMAKE_SYSTEM_NAME="Generic" \
	-UCMAKE_SYSTEM_PROCESSOR \
	-DCMAKE_USER_MAKE_RULES_OVERRIDE="/opt/cosmocc/cosmocc-override.cmake" \
	-DCMAKE_AR="$(command -v cosmoar)" \
	-DCMAKE_RANLIB="$(command -v cosmoranlib)" \
	-DCMAKE_BUILD_TYPE="MinSizeRel" \
	-B build
cmake --build build
