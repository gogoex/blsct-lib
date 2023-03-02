AM_MAKEFLAGS=BLS_ETH=1
SUBDIRS = bls

noinst_LIBRARIES = libblsct.a
libblsct_a_SOURCES = lib.cpp
libblsct_a_LIBADD = bls/lib/libbls384_256.a bls/mcl/lib/libmcl.a
libblsct_a_CXXFLAGS = -Wall -std=c++17 -Ibls/include -Ibls/mcl/include
