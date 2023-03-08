mcllib := ../../../src/bls/mcl/lib
swig := ../swig/swig
module := blsct

main: $(module)/$(module)_wrap.cxx $(module)/$(module).go $(module)/$(module).cxx config.h src include
	go build -x main.go

$(module)/$(module)_wrap.cxx: blsct.i
	$(swig) -go -cgo -c++ -outdir $(module) -o $@ $<

config.h:
	ln -s ../../config.h

src:
	ln -s ../../src

include:
	ln -s ../../include

$(module)/$(module).cxx:
	cd $(module)
	ln -s ../../../src/lib.cpp $(module)/$(module).cxx

run:
	LD_LIBRARY_PATH=`pwd`/src/bls/lib:`pwd`/src/bls/mcl/lib ./main

clean:
	rm -f config.h
	rm -f src
	rm -f include
	rm -f main
	rm -f $(module)/$(module)_wrap.cxx
	rm -f $(module)/$(module).go
	rm -f $(module)/$(module).cxx

.PHONY: clean run main
