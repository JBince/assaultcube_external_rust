use winapi::um::memoryapi::{ ReadProcessMemory, WriteProcessMemory };
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::shared::minwindef::{ LPVOID, LPCVOID };

// Gets the handle of a process from a PID
pub fn get_process_handle(pid: u32) -> HANDLE {
    let handle = unsafe {
        let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);
        if handle.is_null() {
            println!("Failed to get handle");
            return handle;
        }
        handle
    };
    handle
}

// Resolve a multilevel pointer to get the final address the pointer points to, without dereferencing it
pub fn resolve_pointer(handle: HANDLE, offsets: Vec<u32>, mut pointer: u32) -> u32 {
    if offsets.len() == 1 {
        return pointer + offsets[0];
    } else {
        for (_i, &offset) in offsets
            .iter()
            .enumerate()
            .take(offsets.len() - 1) {
            pointer += offset;
            pointer = read_memory(handle, pointer);
        }
        pointer += offsets[offsets.len() - 1];
        pointer
    }
}

// Reads a value in memory at a given address
pub fn read_memory(handle: HANDLE, address: u32) -> u32 {
    let value: [u32; 4] = [0; 4];
    unsafe {
        ReadProcessMemory(handle, address as LPVOID, value.as_ptr() as LPVOID, 4, 0x0 as *mut _);
        // Returns the value at the given address
        *(value.as_ptr() as *const u32)
    }
}

pub fn write_memory(handle: HANDLE, source: &[u8], destination: u32) {
    unsafe {
        WriteProcessMemory(
            handle,
            destination as LPVOID,
            source.as_ptr() as LPCVOID,
            source.len(),
            0x0 as *mut _
        );
    }
}
