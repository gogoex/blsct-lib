#define BLS_ETH 1
#include <bls/bls384_256.h>
#include <stdexcept>
#include <cstdio>
#include <config.h>

int main() {
    if (blsInit(MCL_BLS12_381, MCLBN_COMPILED_TIME_VAR) != 0) {
        throw std::runtime_error("blsInit failed");
    }
    printf("Hello\n");
}

