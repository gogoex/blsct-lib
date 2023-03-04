cflags := $(shell python3-config --cflags)
ldflags := $(shell python3-config --ldflags --embed)
src := ../../src
bls := $(src)/bls
mcl := $(bls)/mcl
config := ../..
include := -I$(bls)/include -I$(mcl)/include -I$(config)

blsct.a: lib.o blsct_wrap.o
	g++ -Wl,--no-undefined --shared lib.o blsct_wrap.o -L$(bls)/lib -lbls384_256 -L$(mcl)/lib -lmcl $(ldflags) -o _blsct.so

lib.o: blsct_wrap.c blsct.py
	g++ -c -fpic $(src)/lib.cpp $(cflags) $(include)

blsct_wrap.o: blsct_wrap.c blsct.py
	g++ -c -fpic blsct_wrap.c $(cflags) $(include)

blsct_wrap.c: blsct.i
	swig -python blsct.i

run:
	LD_LIBRARY_PATH=$(bls)/lib:$(mcl)/lib python3

clean:
	rm -f *.o
	rm -f *.py
	rm -f *_wrap.c
	rm -f *.so
	rm -rf __pycache__

.PHONY: clean run
