SUBDIRS = bls

bin_PROGRAMS = blsctlib
blsctlib_SOURCES = main.cpp
blsctlib_LDADD = bls/lib/libbls384_256.a bls/mcl/lib/libmcl.a
blsctlib_LDFLAGS = -pthread
blsctlib_CXXFLAGS = -Wall -std=c++17 -Ibls/include -Ibls/mcl/include
