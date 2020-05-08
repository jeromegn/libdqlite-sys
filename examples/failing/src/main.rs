use std::ffi::CString;

use libdqlite_sys as sys;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("hello world {}", unsafe {
        libsqlite3_sys::sqlite3_libversion_number()
    });

    let s = CString::new("127.0.0.1:5005")?;
    let node_id = unsafe { sys::dqlite_generate_node_id(s.into_raw()) };

    println!("node id: {}", node_id);

    Ok(())
}
