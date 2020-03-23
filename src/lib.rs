// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(non_camel_case_types)]

/*! Load DLLs/EXEs from memory on Windows.

This crate declares a library interface to https://github.com/fancycode/MemoryModule.

The initial developer of the original code is Joachim Bauch. See the copyright
in the vendored .c/.h files for a more detailed copyright notice.
*/

use libc::size_t;
use std::ffi::c_void;
use winapi::ctypes::c_int;
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{BOOL, DWORD, FARPROC, LPVOID, UINT, WORD};
use winapi::shared::ntdef::{CHAR, LPCSTR, LPTSTR};

type LPCTSTR = *const CHAR;

pub type HMEMORYMODULE = *const c_void;
pub type HMEMORYRSRC = *const c_void;
pub type HCUSTOMMODULE = *const c_void;

pub type custom_alloc_fn = extern "C" fn(LPVOID, SIZE_T, DWORD, DWORD, *mut c_void) -> LPVOID;
pub type custom_free_fn = extern "C" fn(LPVOID, SIZE_T, DWORD, *mut c_void) -> BOOL;
pub type custom_load_library_fn = extern "C" fn(LPCSTR, *mut c_void) -> HCUSTOMMODULE;
pub type custom_get_proc_address_fn =
    extern "C" fn(HCUSTOMMODULE, LPCSTR, *mut c_void) -> FARPROC;
pub type custom_free_library_fn = extern "C" fn(HCUSTOMMODULE, *mut c_void);

extern "C" {
    /// Load exe/dll from memory location with the given size.
    ///
    /// All dependencies are resolved using default LoadLibrary/GetProcAddress
    /// calls through the Windows API.
    pub fn MemoryLoadLibrary(data: *const c_void, size: size_t) -> HMEMORYMODULE;

    /// Load EXE/DLL from memory location with the given size using custom
    /// dependency resolvers.
    ///
    /// Dependencies will be resolved using passed callback methods.
    pub fn MemoryLoadLibraryEx(
        data: *const c_void,
        size: size_t,
        alloc: custom_alloc_fn,
        free: custom_free_fn,
        load_library: custom_load_library_fn,
        get_process_address: custom_get_proc_address_fn,
        free_library: custom_free_library_fn,
        user_data: *mut c_void,
    ) -> HMEMORYMODULE;

    /// Get address of exported method. Supports loading both by name and by ordinal
    /// value.
    pub fn MemoryGetProcAddress(module: HMEMORYMODULE, name: LPCSTR) -> FARPROC;

    /// Free previously loaded exe/dll.
    pub fn MemoryFreeLibrary(module: HMEMORYMODULE);

    /// Execute entry point (EXE only).
    ///
    /// The entry point can only be executed if the EXE has been loaded to the correct base address
    /// or it could be relocated (i.e. relocation information have not been stripped
    /// by the linker).
    ///
    /// Important: calling this function will not return, i.e. once the loaded
    /// EXE finished running, the process will terminate.
    ///
    ///  Returns a negative value if the entry point could not be executed.
    pub fn MemoryCallEntryPoint(module: HMEMORYMODULE) -> c_int;

    /// Find the location of a resource with the specified type and name.
    pub fn MemoryFindResource(module: HMEMORYMODULE, name: LPCTSTR, typ: LPCTSTR) -> HMEMORYRSRC;

    /// Find the location of a resource with the specified type, name and language.
    pub fn MemoryFindResourceEx(
        module: HMEMORYMODULE,
        name: LPCTSTR,
        typ: LPCTSTR,
        language: WORD,
    ) -> HMEMORYRSRC;

    /// Get the size of the resource in bytes.
    pub fn MemorySizeofResource(module: HMEMORYMODULE, resource: HMEMORYRSRC) -> DWORD;

    /// Get a pointer to the contents of the resource.
    pub fn MemoryLoadResource(module: HMEMORYMODULE, resource: HMEMORYRSRC) -> LPVOID;

    /// Load a string resource.
    pub fn MemoryLoadString(
        module: HMEMORYMODULE,
        id: UINT,
        buffer: LPTSTR,
        max_size: c_int,
    ) -> c_int;

    /// Load a string resource with a given language.
    pub fn MemoryLoadStringEx(
        module: HMEMORYMODULE,
        id: UINT,
        buffer: LPTSTR,
        max_size: c_int,
        language: WORD,
    ) -> c_int;

    /// Default implementation of custom_alloc_fn that calls VirtualAlloc()
    /// internally to allocate memory for a library.
    ///
    /// This is the default as used by MemoryLoadLibrary().
    pub fn MemoryDefaultAlloc(
        address: LPVOID,
        size: SIZE_T,
        allocation_type: DWORD,
        protect: DWORD,
        user_data: *mut c_void,
    ) -> LPVOID;

    /// Default implementation of CustomFreeFunc that calls VirtualFree
    /// internally to free the memory used by a library.
    ///
    /// This is the default as used by MemoryLoadLibrary.
    pub fn MemoryDefaultFree(
        address: LPVOID,
        size: SIZE_T,
        free_type: DWORD,
        user_data: *mut c_void,
    ) -> BOOL;

    /// Default implementation of CustomLoadLibraryFunc that calls LoadLibraryA
    /// internally to load an additional libary.
    ///
    /// This is the default as used by MemoryLoadLibrary.
    pub fn MemoryDefaultLoadLibrary(filename: LPCSTR, user_data: *mut c_void) -> HCUSTOMMODULE;

    /// Default implementation of CustomGetProcAddressFunc that calls GetProcAddress
    /// internally to get the address of an exported function.
    ///
    /// This is the default as used by MemoryLoadLibrary.
    pub fn MemoryDefaultGetProcAddress(
        module: HCUSTOMMODULE,
        name: LPCSTR,
        user_data: *mut c_void,
    ) -> FARPROC;

    /// Default implementation of CustomFreeLibraryFunc that calls FreeLibrary
    /// internally to release an additional library.
    ///
    /// This is the default as used by MemoryLoadLibrary().
    pub fn MemoryDefaultFreeLibrary(module: HCUSTOMMODULE, user_data: *mut c_void);
}
