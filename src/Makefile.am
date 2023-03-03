AM_MAKEFLAGS=BLS_ETH=1
SUBDIRS = bls

noinst_LTLIBRARIES = libblsct.la
libblsct_la_SOURCES = lib.cpp
libblsct_la_LIBADD = bls/lib/libbls384_256.a bls/mcl/lib/libmcl.a
libblsct_la_CXXFLAGS = -Wall -std=c++17 -Ibls/include -Ibls/mcl/include
