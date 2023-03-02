#define BLS_ETH 1
#include <bls/bls384_256.h>
#include <stdexcept>
#include <cstdio>
#include <cstdlib>
#include <config.h>

void BlsInit() {
    if (blsInit(MCL_BLS12_381, MCLBN_COMPILED_TIME_VAR) != 0) {
        throw std::runtime_error("blsInit failed");
    }
}

int TestAddition() {
    mclBnFr a, b, c;
    mclBnFr_setInt(&a, 1);
    mclBnFr_setInt(&b, 3);
    mclBnFr_add(&c, &a, &b);

    char buf[1];
    mclBnFr_getStr(buf, 1, &c, 10);
    auto i = atoi(buf);

    return i;
}
