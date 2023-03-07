mcllib := ../../../src/bls/mcl/lib
swig := ../swig/swig

./build/Release/blsct.node: blsct_wrap.cxx
	node-gyp configure build

blsct_wrap.cxx: blsct.i
	$(swig) -c++ -javascript -node blsct.i

run:
	cd build && LD_LIBRARY_PATH=$(mcllib) node ../test.js

clean:
	rm -f *.o
	rm -f *_wrap.c*
	rm -f *.so
	rm -rf build

.PHONY: clean run
