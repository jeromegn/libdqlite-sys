use std::ffi::{CStr, CString};

use libdqlite_sys::sys;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // unsafe {
    //     let addr = "127.0.0.1:5005";
    //     // println!("hello world {}", unsafe {
    //     //     libsqlite3_sys::sqlite3_libversion_number()
    //     // });

    //     let caddr = CString::new(addr)?;
    //     let node_id = sys::dqlite_generate_node_id(caddr.as_c_str().as_ptr());

    //     println!("node id: {}", node_id);

    //     let data_dir = CString::new("/tmp/dqlite-rs")?;

    //     let mut node = {
    //         let mut node = std::mem::MaybeUninit::<sys::dqlite_node>::uninit();
    //         if sys::dqlite_node_create(
    //             node_id,
    //             caddr.as_ptr(),
    //             data_dir.as_c_str().as_ptr(),
    //             &mut node.as_mut_ptr(),
    //         ) != 0
    //         {
    //             let errmsg = CStr::from_ptr(sys::dqlite_node_errmsg(node.as_mut_ptr()));
    //             println!("error creating node: {:?}", errmsg);
    //             return Err(std::io::Error::new(
    //                 std::io::ErrorKind::Other,
    //                 errmsg.to_str().unwrap(),
    //             )
    //             .into());
    //         }
    //         node.assume_init()
    //         // node
    //     };

    //     {
    //         // let caddr = CString::new(addr)?;
    //         println!("passing a string: {:?}", caddr);
    //         let rc = sys::dqlite_node_set_bind_address(&mut node, caddr.as_ptr());
    //         println!("set bind addr rc: {}", rc);
    //         if rc != 0 {
    //             return Err(std::io::Error::new(
    //                 std::io::ErrorKind::Other,
    //                 "failed to set bind address",
    //             )
    //             .into());
    //         }
    //     }

    //     // let rc =
    //     sys::dqlite_node_start(&mut node);
    //     // println!("start rc: {}", rc2);

    //     // {
    //     //     // let bindaddr = ;
    //     //     // println!("got bind addr");
    //     //     println!(
    //     //         "bind addr: {:?}",
    //     //         CStr::from_ptr(sys::dqlite_node_get_bind_address(&mut node))
    //     //     );
    //     // }
    // }

    // let mut conn = std::net::TcpStream::connect(addr)?;

    let mut node = libdqlite_sys::Node::new("127.0.0.1:5005", "/tmp/dqlite-rs")?;
    println!("got node: {:?}", node);

    node.set_bind_address("127.0.0.1:5005")?;
    println!("successfully set bind address");

    Ok(())
}
