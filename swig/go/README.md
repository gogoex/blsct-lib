## Note
- This sample is using go module and initialized by:
  ```bash
  $ go mod init blsct-test
  ```

- `blsct` module dir should be a sub-directory of the main module

- `blsct` module can be imported to the main module by "`main-module-name`/`blsct-module-dir`" e.g.
  ```golang
  import (
    "blsct-test/blsct"
  )
  ```

- Symlink to `src/lib.cpp` is crated in `blsct` module directory with the suffix changed to `.cxx` since golang compiler expects the c++ source files to have `.cxx` suffix

- Source files of `blsct` lib needs to be in the same directory as the main module. To satisify that, symlinks of `src`, `include`, `config.h` are created in the main module directory

## Building
```bash
$ make
```

## Running
```bash
$ make run
```
