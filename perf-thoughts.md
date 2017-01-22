Resources:
* https://llogiq.github.io/2015/07/15/profiling.html
* https://blog.mozilla.org/nnethercote/category/rust/

Using `cargo build --features sqs -vv` as a test:

See output for build script compilation:

Running `rustc build.rs --crate-name build_script_build --crate-type bin -g --cfg feature=\"rusoto_codegen\" --cfg feature=\"serde_codegen\" --cfg feature=\"default\" --cfg feature=\"sqs\" -C metadata=741c7d923e3f0e37 --out-dir /Users/matthew/Documents/rusoto/target/debug/build/rusoto-741c7d923e3f0e37 --emit=dep-info,link -L dependency=/Users/matthew/Documents/rusoto/target/debug/deps --extern rustc_version=/Users/matthew/Documents/rusoto/target/debug/deps/librustc_version-444dfb6f7755262a.rlib --extern rayon=/Users/matthew/Documents/rusoto/target/debug/deps/librayon-ecad47f1e1ddbdc6.rlib --extern rusoto_codegen=/Users/matthew/Documents/rusoto/target/debug/deps/librusoto_codegen.rlib`
     
And the location of the script:     
```
Running `/Users/matthew/Documents/rusoto/target/debug/build/rusoto-741c7d923e3f0e37/build-script-build`
```

Run with valgrind.  OUT_DIR is usually set by cargo.

OUT_DIR="/tmp/" valgrind ./target/debug/build/rusoto-741c7d923e3f0e37/build-script-build

Massif: `OUT_DIR="/tmp/" valgrind --tool=massif --time-unit=B ./target/debug/build/rusoto-741c7d923e3f0e37/build-script-build`

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

Running on rustc for the crate:

```
OUT_DIR="/tmp/" CARGO_PKG_VERSION="0.0.1" time valgrind rustc src/lib.rs --crate-name rusoto --crate-type lib -g --cfg feature=\"serde_codegen\" --cfg feature=\"ec2\" --cfg feature=\"default\" --cfg feature=\"rusoto_codegen\" -C metadata=741c7d923e3f0e37 --out-dir /Users/matthew/Documents/rusoto/target/debug/deps --emit=dep-info,link -L dependency=/Users/matthew/Documents/rusoto/target/debug/deps --extern chrono=/Users/matthew/Documents/rusoto/target/debug/deps/libchrono-7342810e34d1c30d.rlib --extern rusoto_credential=/Users/matthew/Documents/rusoto/target/debug/deps/librusoto_credential.rlib --extern url=/Users/matthew/Documents/rusoto/target/debug/deps/liburl-05b7563c76614a9f.rlib --extern md5=/Users/matthew/Documents/rusoto/target/debug/deps/libmd5-cf231219a5a4e0e7.rlib --extern hyper=/Users/matthew/Documents/rusoto/target/debug/deps/libhyper-7d4e5896feabec06.rlib --extern xml=/Users/matthew/Documents/rusoto/target/debug/deps/libxml-f3cd53f8ceff1f6a.rlib --extern time=/Users/matthew/Documents/rusoto/target/debug/deps/libtime-bcc5484ee52ec8cd.rlib --extern serde=/Users/matthew/Documents/rusoto/target/debug/deps/libserde-5c748cb46e1cb43c.rlib --extern ring=/Users/matthew/Documents/rusoto/target/debug/deps/libring-9371b8a8952a4f9d.rlib --extern regex=/Users/matthew/Documents/rusoto/target/debug/deps/libregex-59f42a2a673f3a9d.rlib --extern lazy_static=/Users/matthew/Documents/rusoto/target/debug/deps/liblazy_static-7f1b96a3a3eb529d.rlib --extern rustc_serialize=/Users/matthew/Documents/rusoto/target/debug/deps/librustc_serialize-6b938435173797f7.rlib --extern log=/Users/matthew/Documents/rusoto/target/debug/deps/liblog-bf16bb9a4912b11d.rlib --extern hyper_native_tls=/Users/matthew/Documents/rusoto/target/debug/deps/libhyper_native_tls-a083bcc9fd348c89.rlib --extern serde_json=/Users/matthew/Documents/rusoto/target/debug/deps/libserde_json-752420654e3d2fa6.rlib --extern serde_codegen=/Users/matthew/Documents/rusoto/target/debug/deps/libserde_codegen-fd0b140eeddde770.rlib -L native=/Users/matthew/Documents/rusoto/target/debug/build/ring-5badc5ca0c9a3718/out/lib
```

From the big dump, the memory usage:

```
==64208== HEAP SUMMARY:
==64208==     in use at exit: 410,880 bytes in 871 blocks
==64208==   total heap usage: 1,424 allocs, 553 frees, 717,696 bytes allocated
==64208== 
==64208== LEAK SUMMARY:
==64208==    definitely lost: 12,320 bytes in 187 blocks
==64208==    indirectly lost: 1,200 bytes in 27 blocks
==64208==      possibly lost: 2,120 bytes in 54 blocks
==64208==    still reachable: 27,178 bytes in 272 blocks
==64208==         suppressed: 368,062 bytes in 331 blocks
==64208== Rerun with --leak-check=full to see details of leaked memory
==64208== 
==64208== For counts of detected and suppressed errors, rerun with: -v
==64208== Use --track-origins=yes to see where uninitialised values come from
==64208== ERROR SUMMARY: 12 errors from 8 contexts (suppressed: 0 from 0)
```

With massif:


OUT_DIR="/tmp/" CARGO_PKG_VERSION="0.0.1" time valgrind --tool=massif --time-unit=B rustc --crate-name rusoto src/lib.rs --crate-type lib -g --cfg feature=\"ec2\" --cfg feature=\"default\" --cfg feature=\"sqs\" --cfg feature=\"serde_codegen\" --cfg feature=\"s3\" --cfg feature=\"rusoto_codegen\" -C metadata=698e50f6af7b8401 -C extra-filename=-698e50f6af7b8401 --out-dir /Users/matthew/Documents/rusoto/target/debug/deps --emit=dep-info,link -L dependency=/Users/matthew/Documents/rusoto/target/debug/deps --extern rusoto_credential=/Users/matthew/Documents/rusoto/target/debug/deps/librusoto_credential-fd17c3c489346198.rlib --extern lazy_static=/Users/matthew/Documents/rusoto/target/debug/deps/liblazy_static-13daae1d9a07c252.rlib --extern log=/Users/matthew/Documents/rusoto/target/debug/deps/liblog-1ce22d3a92f37841.rlib --extern hyper_native_tls=/Users/matthew/Documents/rusoto/target/debug/deps/libhyper_native_tls-8adb5dedbc236d78.rlib --extern serde_codegen=/Users/matthew/Documents/rusoto/target/debug/deps/libserde_codegen-ff022c3038b211d6.rlib --extern serde=/Users/matthew/Documents/rusoto/target/debug/deps/libserde-1978de041ce68cda.rlib --extern ring=/Users/matthew/Documents/rusoto/target/debug/deps/libring-82824b7189f879d8.rlib --extern url=/Users/matthew/Documents/rusoto/target/debug/deps/liburl-df2b4fe65acab452.rlib --extern hyper=/Users/matthew/Documents/rusoto/target/debug/deps/libhyper-edcb94879da8a107.rlib --extern chrono=/Users/matthew/Documents/rusoto/target/debug/deps/libchrono-512b039bb0c9bc86.rlib --extern time=/Users/matthew/Documents/rusoto/target/debug/deps/libtime-53238cb92943b947.rlib --extern regex=/Users/matthew/Documents/rusoto/target/debug/deps/libregex-85104e2a1249fae2.rlib --extern xml=/Users/matthew/Documents/rusoto/target/debug/deps/libxml-79a5d5c47edce985.rlib --extern serde_json=/Users/matthew/Documents/rusoto/target/debug/deps/libserde_json-11ce0c0a6bd2aa8c.rlib --extern rustc_serialize=/Users/matthew/Documents/rusoto/target/debug/deps/librustc_serialize-03b62901d97343ea.rlib --extern md5=/Users/matthew/Documents/rusoto/target/debug/deps/libmd5-e03ad2cfa427c7c8.rlib -L native=/Users/matthew/Documents/rusoto/target/debug/build/ring-6d6547894d2a7043/out/lib