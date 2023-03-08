swig := ../swig/swig
module := blsct
module_wrap := $(module)/$(module)_wrap.cxx
module_go := $(module)/$(module).go
module_main := $(module)/lib.cxx
exe := main

$(exe): $(module_wrap) $(module_go) $(module_main) config.h src include
	go build -x $(exe).go

$(module_wrap): blsct.i
	$(swig) -go -cgo -c++ -outdir $(module) -o $@ $<

config.h:
	ln -s ../../config.h

src:
	ln -s ../../src

include:
	ln -s ../../include

$(module_main):
	cd $(module)
	ln -s ../../../src/lib.cpp $@

run:
	LD_LIBRARY_PATH=`pwd`/src/bls/lib:`pwd`/src/bls/mcl/lib ./$(exe)

clean:
	rm -f config.h
	rm -f src
	rm -f include
	rm -f $(exe)
	rm -f $(module_wrap)
	rm -f $(module_go)
	rm -f $(module_main)

.PHONY: clean run
