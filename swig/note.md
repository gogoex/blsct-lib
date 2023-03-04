```sh
$ g++ -c -fpic ../src/lib.cpp blsct_wrap.c $(python3-config --cflags) -I../src/bls/include -I../src/bls/mcl/include -I..
$ g++ --shared lib.o blsct_wrap.o $(python3-config --ldflags) -o blsct.so
```