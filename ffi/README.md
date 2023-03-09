## Building `swig`

```bash
$ cd .. # move to swig dir
$ git submodule update --remote swig # maybe wrong, but this is to load the current master
$ ./autogen.sh
$ ./configure --prefix=`pwd` # setting prefix to install config files under swig dir
$ make
$ make install # install swig config files locally under swig dir
```
