extern crate pkg_config;
extern crate cc;

use std::env;

fn main() {
    match pkg_config::find_library("lua5.2") {
        Ok(_) => return,
        Err(..) => {}
    };

    let mut build = cc::Build::new();

    if env::var("CARGO_CFG_TARGET_OS") == Ok("linux".to_string()) {
        // Enable `io.popen` support
        build.define("LUA_USE_LINUX", None);
    }

    build
        .file("third-party/lua/src/lapi.c")
        .file("third-party/lua/src/lcode.c")
        .file("third-party/lua/src/lctype.c")
        .file("third-party/lua/src/ldebug.c")
        .file("third-party/lua/src/ldo.c")
        .file("third-party/lua/src/ldump.c")
        .file("third-party/lua/src/lfunc.c")
        .file("third-party/lua/src/lgc.c")
        .file("third-party/lua/src/llex.c")
        .file("third-party/lua/src/lmem.c")
        .file("third-party/lua/src/lobject.c")
        .file("third-party/lua/src/lopcodes.c")
        .file("third-party/lua/src/lparser.c")
        .file("third-party/lua/src/lstate.c")
        .file("third-party/lua/src/lstring.c")
        .file("third-party/lua/src/ltable.c")
        .file("third-party/lua/src/ltm.c")
        .file("third-party/lua/src/lundump.c")
        .file("third-party/lua/src/lvm.c")
        .file("third-party/lua/src/lzio.c")
        .file("third-party/lua/src/lauxlib.c")
        .file("third-party/lua/src/lbaselib.c")
        .file("third-party/lua/src/lbitlib.c")
        .file("third-party/lua/src/lcorolib.c")
        .file("third-party/lua/src/ldblib.c")
        .file("third-party/lua/src/liolib.c")
        .file("third-party/lua/src/lmathlib.c")
        .file("third-party/lua/src/loslib.c")
        .file("third-party/lua/src/lstrlib.c")
        .file("third-party/lua/src/ltablib.c")
        .file("third-party/lua/src/loadlib.c")
        .file("third-party/lua/src/linit.c")
        .define("LUA_COMPAT_ALL", None)
        .include("third-party/lua/src")
        .compile("liblua.a");
}
