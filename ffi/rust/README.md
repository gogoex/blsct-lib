## Building & Running
```
$ cargo run
```

## TODO
- when i run the lib with `.compile("blsct")` for the first time, it fails.
then if i change it to `.compile("blsct2")` and run again, it builds fine.
- however if i start with `.compile("blsct.so")` and do `.compile("blsct"), it doesn't work
- build dir after `.compile("blsct")` is:
  ```
  bindings.rs  e58908e91188c193-lib.o  libblsct.a
  ```
- build dir after running `.compile("blsct.so")` it becomes:
  ```
  bindings.rs  e58908e91188c193-lib.o  libblsct.a  libblsct.so.a
  ```
- if `blsct.so` is replaced with `blsct2`, it still works