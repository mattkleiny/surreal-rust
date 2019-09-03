//! The lua scripting engine for Surreal.
//!
//! This is a very light-weight and opinionated version of how to implement Lua inside Rust.
//! We throw away a lot of unnecessary pandering and 'safety' in order to provide a competent
//! API for getting shit done.

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::*;

/// The Lua scripting engine backend.
///
/// Internally, we build and manage our own Lua FFI layer, and compile a custom version of Lua
/// for consumption by users of the library.
///
/// Lua itself is very small and extendable; using a pre-packaged build doesn't make much sense
/// as it prevents us from extending and optimizing where appropriate.
///
/// This interface is quite unsafe, as Lua itself is native C and we're a thin veneer on top.
/// Instead of trying to convert Lua into some safe beast from Rust's perspective, we're instead
/// just making it simple to work with and extend. Get the thing working first and something built
/// with it before we polish it up and make it 'clean'.
pub struct LuaScriptEngine {
  handle: LuaHandle,
}

impl LuaScriptEngine {
  pub fn new() -> Self {
    Self { handle: LuaHandle::new() }
  }

  pub fn open_libs(&mut self) { unsafe { ffi::luaL_openlibs(*self.handle.write_lock()) } }
  pub fn open_base(&mut self) { unsafe { ffi::luaopen_base(*self.handle.write_lock()) } }
  pub fn open_bit32(&mut self) { unsafe { ffi::luaopen_bit32(*self.handle.write_lock()) } }
  pub fn open_coroutine(&mut self) { unsafe { ffi::luaopen_coroutine(*self.handle.write_lock()) } }
  pub fn open_debug(&mut self) { unsafe { ffi::luaopen_debug(*self.handle.write_lock()) } }
  pub fn open_io(&mut self) { unsafe { ffi::luaopen_io(*self.handle.write_lock()) } }
  pub fn open_math(&mut self) { unsafe { ffi::luaopen_math(*self.handle.write_lock()) } }
  pub fn open_os(&mut self) { unsafe { ffi::luaopen_os(*self.handle.write_lock()) } }
  pub fn open_package(&mut self) { unsafe { ffi::luaopen_package(*self.handle.write_lock()) } }
  pub fn open_string(&mut self) { unsafe { ffi::luaopen_string(*self.handle.write_lock()) } }
  pub fn open_table(&mut self) { unsafe { ffi::luaopen_table(*self.handle.write_lock()) } }
}

/// The default script engine implementation for Lua.
impl ScriptEngine for LuaScriptEngine {
  type Error = LuaError;
  type Code = LuaCode<'static>;

  fn execute<C: AsRef<str>>(&mut self, code: C) -> Result<(), Self::Error> {
    unimplemented!()
  }
}

/// Encapsulates code that can be directly executed as Lua code.
#[derive(Debug)]
pub struct LuaCode<'a>(&'a str);

/// Error that can happen when executing Lua code.
#[derive(Debug)]
pub enum LuaError {
  /// There was a syntax error when parsing the Lua code.
  SyntaxError(String),
  /// There was an error during execution of the Lua code
  /// (for example not enough parameters for a function call).
  ExecutionError(String),
  /// The call to `execute` has requested the wrong type of data.
  WrongType,
}

/// The internal Lua handle, which is just a pointer back to the native Lua state.
///
/// We synchronize on access to lua state using a reader/writer lock.
#[derive(Debug)]
struct LuaHandle(RwLock<*mut ffi::lua_State>);

impl LuaHandle {
  pub fn new() -> Self {
    unsafe {
      // allocate the lua state, allow it to reach into the rust allocator
      let state = ffi::lua_newstate(alloc, std::ptr::null_mut());
      // wire up the panic callback; reach back into rust and unwind exceptions
      ffi::lua_atpanic(state, panic);

      Self(RwLock::new(state))
    }
  }

  /// Acquires a mutual read lock on the lua state.
  pub fn read_lock(&self) -> RwLockReadGuard<*mut ffi::lua_State> {
    self.0.read().unwrap()
  }

  /// Acquires an exclusive write lock on the lua state.
  pub fn write_lock(&mut self) -> RwLockWriteGuard<*mut ffi::lua_State> {
    self.0.write().unwrap()
  }
}

impl Drop for LuaHandle {
  fn drop(&mut self) {
    unsafe { ffi::lua_close(*self.write_lock()); }
  }
}
/// Allows external lua code to allocate state via the Rust allocator.
extern "C" fn alloc(_ud: *mut libc::c_void,
                    ptr: *mut libc::c_void,
                    osize: usize,
                    nsize: usize)
                    -> *mut libc::c_void {
  unsafe {
    if nsize == 0 {
      libc::free(ptr as *mut libc::c_void);
      std::ptr::null_mut()
    } else {
      libc::realloc(ptr, nsize)
    }
  }
}

/// Called whenever lua encounters an unexpected error.
extern "C" fn panic(lua: *mut ffi::lua_State) -> std::os::raw::c_int {
  let err = unsafe { ffi::lua_tostring(lua, -1) };
  let err = unsafe { std::ffi::CStr::from_ptr(err) };
  let err = String::from_utf8(err.to_bytes().to_vec()).unwrap();

  panic!("PANIC: unprotected error in call to Lua API ({})\n", err);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn engine_should_open_base_library() {
    let mut engine = LuaScriptEngine::new();

    engine.open_base();
  }

  #[test]
  fn engine_should_open_all_libraries() {
    let mut engine = LuaScriptEngine::new();

    engine.open_libs();
  }

  #[test]
  fn engine_should_execute_basic_lua_code() {
    let mut engine = LuaScriptEngine::new();

    engine.open_base();
    engine.execute("print 'Hello, World!'").unwrap();
  }
}

mod ffi {
  //! Foreign function interface for the native Lua module.

  #![allow(non_camel_case_types)]
  #![allow(non_snake_case)]
  #![allow(dead_code)]
  #![allow(improper_ctypes)]

  use std::{default, ptr};
  use std::os::raw::{c_char, c_double, c_int, c_uchar, c_ulong, c_void};

  pub const MULTRET: c_int = -1;

  pub const LUAI_MAXSTACK: c_int = 1000000;
  pub const LUAI_FIRSTPSEUDOIDX: c_int = (-LUAI_MAXSTACK - 1000);
  pub const LUA_REGISTRYINDEX: c_int = LUAI_FIRSTPSEUDOIDX;

  pub const LUA_OK: c_int = 0;
  pub const LUA_YIELD: c_int = 1;
  pub const LUA_ERRRUN: c_int = 2;
  pub const LUA_ERRSYNTAX: c_int = 3;
  pub const LUA_ERRMEM: c_int = 4;
  pub const LUA_ERRGCMM: c_int = 5;
  pub const LUA_ERRERR: c_int = 6;

  #[repr(C)]
  #[allow(missing_copy_implementations)]
  pub struct lua_State;

  pub type lua_CFunction = extern "C" fn(L: *mut lua_State) -> c_int;
  pub type lua_Reader = extern "C" fn(L: *mut lua_State, ud: *mut c_void, sz: *mut usize) -> *const c_char;
  pub type lua_Writer = extern "C" fn(L: *mut lua_State, p: *const c_void, sz: usize, ud: *mut c_void) -> c_int;
  pub type lua_Alloc = extern "C" fn(ud: *mut c_void, ptr: *mut c_void, osize: usize, nsize: usize) -> *mut c_void;
  pub type lua_Hook = extern "C" fn(L: *mut lua_State, ar: *mut lua_Debug);

  pub const LUA_TNONE: c_int = -1;

  pub const LUA_TNIL: c_int = 0;
  pub const LUA_TBOOLEAN: c_int = 1;
  pub const LUA_TLIGHTUSERDATA: c_int = 2;
  pub const LUA_TNUMBER: c_int = 3;
  pub const LUA_TSTRING: c_int = 4;
  pub const LUA_TTABLE: c_int = 5;
  pub const LUA_TFUNCTION: c_int = 6;
  pub const LUA_TUSERDATA: c_int = 7;
  pub const LUA_TTHREAD: c_int = 8;

  pub const LUA_MINSTACK: c_int = 20;

  pub const LUA_RIDX_MAINTHREAD: c_int = 1;
  pub const LUA_RIDX_GLOBALS: c_int = 2;

  pub const LUA_REFNIL: c_int = -1;
  pub const LUA_NOREF: c_int = -2;

  pub type lua_Number = c_double;
  pub type lua_Integer = usize;
  pub type lua_Unsigned = c_ulong;

  pub const LUA_OPADD: c_int = 0;
  pub const LUA_OPSUB: c_int = 1;
  pub const LUA_OPMUL: c_int = 2;
  pub const LUA_OPDIV: c_int = 3;
  pub const LUA_OPMOD: c_int = 4;
  pub const LUA_OPPOW: c_int = 5;
  pub const LUA_OPUNM: c_int = 6;

  pub const LUA_OPEQ: c_int = 0;
  pub const LUA_OPLT: c_int = 1;
  pub const LUA_OPLE: c_int = 2;

  pub const LUA_GCSTOP: c_int = 0;
  pub const LUA_GCRESTART: c_int = 1;
  pub const LUA_GCCOLLECT: c_int = 2;
  pub const LUA_GCCOUNT: c_int = 3;
  pub const LUA_GCCOUNTB: c_int = 4;
  pub const LUA_GCSTEP: c_int = 5;
  pub const LUA_GCSETPAUSE: c_int = 6;
  pub const LUA_GCSETSTEPMUL: c_int = 7;
  pub const LUA_GCSETMAJORINC: c_int = 8;
  pub const LUA_GCISRUNNING: c_int = 9;
  pub const LUA_GCGEN: c_int = 10;
  pub const LUA_GCINC: c_int = 11;

  pub const LUA_HOOKCALL: c_int = 0;
  pub const LUA_HOOKRET: c_int = 1;
  pub const LUA_HOOKLINE: c_int = 2;
  pub const LUA_HOOKCOUNT: c_int = 3;
  pub const LUA_HOOKTAILRET: c_int = 4;

  pub const LUA_MASKCALL: c_int = 1 << LUA_HOOKCALL as c_int;
  pub const LUA_MASKRET: c_int = 1 << LUA_HOOKRET as c_int;
  pub const LUA_MASKLINE: c_int = 1 << LUA_HOOKLINE as c_int;
  pub const LUA_MASKCOUNT: c_int = 1 << LUA_HOOKCOUNT as c_int;

  #[repr(C)]
  #[allow(missing_copy_implementations)]
  pub struct lua_Debug {
    pub event: c_int,
    pub name: *const c_char,
    pub namewhat: *const c_char,
    pub what: *const c_char,
    pub source: *const c_char,
    pub currentline: c_int,
    pub linedefined: c_int,
    pub lastlinedefined: c_int,
    pub nups: c_uchar,
    pub nparams: c_uchar,
    pub isvararg: c_char,
    pub istailcall: c_char,
    pub short_src: [c_char; 60],
    //i_ci: *CallInfo
  }

  impl default::Default for lua_Debug {
    fn default() -> lua_Debug {
      lua_Debug {
        event: 0,
        name: ptr::null(),
        namewhat: ptr::null(),
        what: ptr::null(),
        source: ptr::null(),
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
      }
    }
  }

  #[inline(always)]
  pub fn lua_upvalueindex(i: c_int) -> c_int {
    LUA_REGISTRYINDEX - i
  }

  #[inline(always)]
  pub unsafe fn lua_call(L: *mut lua_State, nargs: c_int, nresults: c_int) {
    lua_callk(L, nargs, nresults, 0, None)
  }

  #[inline(always)]
  pub unsafe fn lua_pcall(L: *mut lua_State, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int {
    lua_pcallk(L, nargs, nresults, errfunc, 0, None)
  }

  #[inline(always)]
  pub unsafe fn lua_yield(L: *mut lua_State, nresults: c_int) -> c_int {
    lua_yieldk(L, nresults, 0, None)
  }

  #[inline(always)]
  pub unsafe fn lua_pop(L: *mut lua_State, n: c_int) {
    lua_settop(L, -n - 1)
  }

  #[inline(always)]
  pub unsafe fn lua_newtable(L: *mut lua_State) {
    lua_createtable(L, 0, 0)
  }

  #[inline(always)]
  pub unsafe fn lua_register(L: *mut lua_State, name: *const c_char, f: lua_CFunction) {
    lua_pushcfunction(L, f);
    lua_setglobal(L, name)
  }

  #[inline(always)]
  pub unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(L, f, 0)
  }

  #[inline(always)]
  pub unsafe fn lua_isfunction(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) == LUA_TFUNCTION
  }

  #[inline(always)]
  pub unsafe fn lua_istable(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) == LUA_TTABLE
  }

  #[inline(always)]
  pub unsafe fn lua_islightuserdata(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) == LUA_TLIGHTUSERDATA
  }

  #[inline(always)]
  pub unsafe fn lua_isnil(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) == LUA_TNIL
  }

  #[inline(always)]
  pub unsafe fn lua_isboolean(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) == LUA_TBOOLEAN
  }

  #[inline(always)]
  pub unsafe fn lua_isthread(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) == LUA_TTHREAD
  }

  #[inline(always)]
  pub unsafe fn lua_isnone(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) == LUA_TNONE
  }

  #[inline(always)]
  pub unsafe fn lua_isnoneornil(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) <= 0
  }

  // TODO: lua_pushliteral

  #[inline(always)]
  pub unsafe fn lua_pushglobaltable(L: *mut lua_State) {
    lua_rawgeti(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS)
  }

  #[inline(always)]
  pub unsafe fn lua_tostring(L: *mut lua_State, i: c_int) -> *const c_char {
    lua_tolstring(L, i, ptr::null_mut())
  }

  extern "C" {
    pub fn lua_newstate(f: lua_Alloc, ud: *mut c_void) -> *mut lua_State;
    pub fn lua_close(L: *mut lua_State);
    pub fn lua_newthread(L: *mut lua_State) -> *mut lua_State;

    pub fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;

    pub fn lua_version(L: *mut lua_State) -> *const lua_Number;

    pub fn lua_absindex(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_gettop(L: *mut lua_State) -> c_int;
    pub fn lua_settop(L: *mut lua_State, idx: c_int);
    pub fn lua_pushvalue(L: *mut lua_State, idx: c_int);
    pub fn lua_remove(L: *mut lua_State, idx: c_int);
    pub fn lua_insert(L: *mut lua_State, idx: c_int);
    pub fn lua_replace(L: *mut lua_State, idx: c_int);
    pub fn lua_copy(L: *mut lua_State, fromidx: c_int, toidx: c_int);
    pub fn lua_checkstack(L: *mut lua_State, sz: c_int) -> c_int;

    pub fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int);

    pub fn lua_isnumber(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_isstring(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_iscfunction(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_isuserdata(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_type(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_typename(L: *mut lua_State, tp: c_int) -> *const c_char;

    pub fn lua_tonumberx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Number;
    pub fn lua_tointegerx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Integer;
    pub fn lua_tounsignedx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Unsigned;
    pub fn lua_toboolean(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_tolstring(L: *mut lua_State, idx: c_int, len: *mut usize) -> *const c_char;
    pub fn lua_rawlen(L: *mut lua_State, idx: c_int) -> usize;
    pub fn lua_tocfunction(L: *mut lua_State, idx: c_int) -> Option<lua_CFunction>;
    pub fn lua_touserdata(L: *mut lua_State, idx: c_int) -> *mut c_void;
    pub fn lua_tothread(L: *mut lua_State, idx: c_int) -> *mut lua_State;
    pub fn lua_topointer(L: *mut lua_State, idx: c_int) -> *const c_void;

    pub fn lua_arith(L: *mut lua_State, op: c_int);
    pub fn lua_rawequal(L: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int;
    pub fn lua_compare(L: *mut lua_State, idx1: c_int, idx2: c_int, op: c_int) -> c_int;

    pub fn lua_pushnil(L: *mut lua_State);
    pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    pub fn lua_pushunsigned(L: *mut lua_State, n: lua_Unsigned);
    pub fn lua_pushlstring(L: *mut lua_State, s: *const c_char, l: usize);
    pub fn lua_pushstring(L: *mut lua_State, s: *const c_char);
    // TODO: lua_pushvfstring()
    pub fn lua_pushfstring(L: *mut lua_State, fmt: *const c_char, ...) -> *const c_char;
    pub fn lua_pushcclosure(L: *mut lua_State, f: lua_CFunction, n: c_int);
    pub fn lua_pushboolean(L: *mut lua_State, b: c_int);
    pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut c_void);
    pub fn lua_pushthread(L: *mut lua_State) -> c_int;

    pub fn lua_getglobal(L: *mut lua_State, var: *const c_char);
    pub fn lua_gettable(L: *mut lua_State, idx: c_int);
    pub fn lua_getfield(L: *mut lua_State, idx: c_int, k: *const c_char);
    pub fn lua_rawget(L: *mut lua_State, idx: c_int);
    pub fn lua_rawgeti(L: *mut lua_State, idx: c_int, n: c_int);
    pub fn lua_rawgetp(L: *mut lua_State, idx: c_int, p: *const c_char);
    pub fn lua_createtable(L: *mut lua_State, narr: c_int, nrec: c_int);
    pub fn lua_newuserdata(L: *mut lua_State, sz: usize) -> *mut c_void;
    pub fn lua_getmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
    pub fn lua_getfenv(L: *mut lua_State, idx: c_int);

    pub fn lua_setglobal(L: *mut lua_State, var: *const c_char);
    pub fn lua_settable(L: *mut lua_State, idx: c_int);
    pub fn lua_setfield(L: *mut lua_State, idx: c_int, k: *const c_char);
    pub fn lua_rawset(L: *mut lua_State, idx: c_int);
    pub fn lua_rawseti(L: *mut lua_State, idx: c_int, n: c_int);
    pub fn lua_rawsetp(L: *mut lua_State, idx: c_int, p: *const c_char);
    pub fn lua_setmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
    pub fn lua_setfenv(L: *mut lua_State, idx: c_int) -> c_int;

    pub fn lua_callk(L: *mut lua_State, nargs: c_int, nresults: c_int, ctx: c_int, k: Option<lua_CFunction>);
    pub fn lua_getctx(L: *mut lua_State, ctx: c_int) -> c_int;
    pub fn lua_pcallk(L: *mut lua_State, nargs: c_int, nresults: c_int, errfunc: c_int, ctx: c_int, k: Option<lua_CFunction>) -> c_int;
    pub fn lua_load(L: *mut lua_State, reader: lua_Reader, dt: *mut c_void, chunkname: *const c_char, mode: *const c_char) -> c_int;
    pub fn lua_dump(L: *mut lua_State, writer: lua_Writer, data: *mut c_void) -> c_int;

    pub fn lua_yieldk(L: *mut lua_State, nresults: c_int, ctx: c_int, k: Option<lua_CFunction>) -> c_int;
    pub fn lua_resume(L: *mut lua_State, from: *mut lua_State, narg: c_int) -> c_int;
    pub fn lua_status(L: *mut lua_State) -> c_int;

    pub fn lua_gc(L: *mut lua_State, what: c_int, data: c_int) -> c_int;

    pub fn lua_error(L: *mut lua_State) -> c_int;
    pub fn lua_next(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_concat(L: *mut lua_State, n: c_int);
    pub fn lua_len(L: *mut lua_State, idx: c_int);

    pub fn lua_getallocf(L: *mut lua_State, ud: *mut *mut c_void) -> lua_Alloc;
    pub fn lua_setallocf(L: *mut lua_State, f: lua_Alloc, ud: *mut c_void);

    pub fn lua_getstack(L: *mut lua_State, level: c_int, ar: *mut lua_Debug) -> c_int;
    pub fn lua_getinfo(L: *mut lua_State, what: *const c_char, ar: *mut lua_Debug) -> c_int;
    pub fn lua_getlocal(L: *mut lua_State, ar: *const lua_Debug, n: c_int) -> *const c_char;
    pub fn lua_setlocal(L: *mut lua_State, ar: *mut lua_Debug, n: c_int) -> *const c_char;
    pub fn lua_getupvalue(L: *mut lua_State, funcindex: c_int, n: c_int) -> *const c_char;
    pub fn lua_setupvalue(L: *mut lua_State, funcindex: c_int, n: c_int) -> *const c_char;

    pub fn lua_upvalueid(L: *mut lua_State, fidx: c_int, n: c_int) -> *const c_void;
    pub fn lua_upvaluejoin(L: *mut lua_State, fidx1: c_int, n1: c_int, fidx2: c_int, n2: c_int);

    pub fn lua_sethook(L: *mut lua_State, func: lua_Hook, mask: c_int, count: c_int) -> c_int;
    pub fn lua_gethook(L: *mut lua_State) -> lua_Hook;
    pub fn lua_gethookmask(L: *mut lua_State) -> c_int;
    pub fn lua_gethookcount(L: *mut lua_State) -> c_int;

    pub fn luaL_openlibs(L: *mut lua_State);
    pub fn luaL_ref(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn luaL_unref(L: *mut lua_State, idx: c_int, ref_id: c_int);

    pub fn luaopen_base(L: *mut lua_State);
    pub fn luaopen_bit32(L: *mut lua_State);
    pub fn luaopen_coroutine(L: *mut lua_State);
    pub fn luaopen_debug(L: *mut lua_State);
    pub fn luaopen_io(L: *mut lua_State);
    pub fn luaopen_math(L: *mut lua_State);
    pub fn luaopen_os(L: *mut lua_State);
    pub fn luaopen_package(L: *mut lua_State);
    pub fn luaopen_string(L: *mut lua_State);
    pub fn luaopen_table(L: *mut lua_State);
  }
}
