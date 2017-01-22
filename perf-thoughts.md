Resources:
* https://llogiq.github.io/2015/07/15/profiling.html
* https://blog.mozilla.org/nnethercote/category/rust/

Using `cargo build --features sqs -vv` as a test:

See output for build script compilation:

Running `rustc build.rs --crate-name build_script_build --crate-type bin -g --cfg feature=\"rusoto_codegen\" --cfg feature=\"serde_codegen\" --cfg feature=\"default\" --cfg feature=\"sqs\" -C metadata=741c7d923e3f0e37 --out-dir /Users/matthew/Documents/rusoto/target/debug/build/rusoto-741c7d923e3f0e37 --emit=dep-info,link -L dependency=/Users/matthew/Documents/rusoto/target/debug/deps --extern rustc_version=/Users/matthew/Documents/rusoto/target/debug/deps/librustc_version-444dfb6f7755262a.rlib --extern rayon=/Users/matthew/Documents/rusoto/target/debug/deps/librayon-ecad47f1e1ddbdc6.rlib --extern rusoto_codegen=/Users/matthew/Documents/rusoto/target/debug/deps/librusoto_codegen.rlib`
     
And the location of the script:     
Running `/Users/matthew/Documents/rusoto/target/debug/build/rusoto-741c7d923e3f0e37/build-script-build`

Run with valgrind.  OUT_DIR is usually set by cargo.

OUT_DIR="/tmp/" valgrind ./target/debug/build/rusoto-741c7d923e3f0e37/build-script-build

Output:

```
==63320== Memcheck, a memory error detector
==63320== Copyright (C) 2002-2015, and GNU GPL'd, by Julian Seward et al.
==63320== Using Valgrind-3.12.0 and LibVEX; rerun with -h for copyright info
==63320== Command: ./target/debug/build/rusoto-741c7d923e3f0e37/build-script-build
==63320== 

Generated 1 services.

cargo:rerun-if-changed=codegen
==63320== 
==63320== HEAP SUMMARY:
==63320==     in use at exit: 25,510 bytes in 204 blocks
==63320==   total heap usage: 284 allocs, 80 frees, 40,598 bytes allocated
==63320== 
==63320== LEAK SUMMARY:
==63320==    definitely lost: 64 bytes in 1 blocks
==63320==    indirectly lost: 0 bytes in 0 blocks
==63320==      possibly lost: 2,064 bytes in 1 blocks
==63320==    still reachable: 2,960 bytes in 10 blocks
==63320==         suppressed: 20,422 bytes in 192 blocks
==63320== Rerun with --leak-check=full to see details of leaked memory
==63320== 
==63320== For counts of detected and suppressed errors, rerun with: -v
==63320== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

Built the build script for ec2 and got this valgrind output:

```==63533== Memcheck, a memory error detector
==63533== Copyright (C) 2002-2015, and GNU GPL'd, by Julian Seward et al.
==63533== Using Valgrind-3.12.0 and LibVEX; rerun with -h for copyright info
==63533== Command: ./target/debug/build/rusoto-741c7d923e3f0e37/build-script-build
==63533== 
--63533-- run: /usr/bin/dsymutil "./target/debug/build/rusoto-741c7d923e3f0e37/build-script-build"
warning: (x86_64) /Users/matthew/Documents/rusoto/target/debug/build/rusoto-741c7d923e3f0e37/build_script_build.0.o unable to open object file: No such file or directory

Generated 1 services.

cargo:rerun-if-changed=codegen
==63533== 
==63533== HEAP SUMMARY:
==63533==     in use at exit: 25,510 bytes in 204 blocks
==63533==   total heap usage: 284 allocs, 80 frees, 40,598 bytes allocated
==63533== 
==63533== LEAK SUMMARY:
==63533==    definitely lost: 64 bytes in 1 blocks
==63533==    indirectly lost: 0 bytes in 0 blocks
==63533==      possibly lost: 2,064 bytes in 1 blocks
==63533==    still reachable: 2,960 bytes in 10 blocks
==63533==         suppressed: 20,422 bytes in 192 blocks
==63533== Rerun with --leak-check=full to see details of leaked memory
==63533== 
==63533== For counts of detected and suppressed errors, rerun with: -v
==63533== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```
