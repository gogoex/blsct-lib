## Requirements
- swig 4.1.0 or above
- node v12-v18
- node-gyp

## Installing `node-gyp`
```bash
$ npm i -g node-gyp
```

## Building `swig` locally
```bash
$ cd .. # move to swig dir
$ git submodule update --remote swig # maybe wrong, but this is to load the current master
$ ./autogen.sh
$ ./configure --prefix=`pwd` # setting prefix to make install locally
$ make
$ make install # this is required to locate config files which swig loads when it's executed
```

## Building `blsct.node` module
```bash
$ make
```

## Running `test.js`
```bash
$ make run
```