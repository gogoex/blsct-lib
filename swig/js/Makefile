cflags :=
ldflags := -L/usr/lib/x86_64-linux-gnu -lv8
src := ../../src
bls := $(src)/bls
mcl := $(bls)/mcl
config := ../..
include := -I$(bls)/include -I$(mcl)/include -I$(config) -I/usr/include/nodejs/src -I/usr/include/nodejs/deps/v8/include

blsct.so: lib.o blsct_wrap.o
	g++ -Wl,--no-undefined --shared lib.o blsct_wrap.o -L$(bls)/lib -lbls384_256 -L$(mcl)/lib -lmcl $(ldflags) -o _blsct.so

lib.o: $(src)/lib.cpp
	g++ -c -fpic $(src)/lib.cpp $(cflags) $(include)

blsct_wrap.o: blsct_wrap.cxx
	g++ -c -fpic blsct_wrap.cxx $(cflags) $(include)

blsct_wrap.cxx: blsct.i
	swig -c++ -javascript -node blsct.i

run:
	LD_LIBRARY_PATH=$(bls)/lib:$(mcl)/lib python3

clean:
	rm -f *.o
	rm -f *.js
	rm -f *_wrap.c*
	rm -f *.so

.PHONY: clean run
