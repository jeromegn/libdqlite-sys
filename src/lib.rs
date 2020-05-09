use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use thiserror::Error;

pub mod sys;
use sys::*;

#[derive(Debug)]
pub struct Node(dqlite_node);

impl Node {
    pub fn new(addr: &str, data_dir: &str) -> Result<Node, DQliteError> {
        std::fs::create_dir_all(data_dir).unwrap();
        let caddr = CString::new(addr).unwrap();
        // let node_id = unsafe { dqlite_generate_node_id(caddr.as_ptr()) };
        let node_id = 1;
        println!("node id: {}", node_id);

        let cdata_dir = CString::new(data_dir).unwrap();
        let mut node = MaybeUninit::<dqlite_node>::uninit();
        let rc = unsafe {
            dqlite_node_create(
                node_id,
                caddr.as_ptr(),
                cdata_dir.as_ptr(),
                &mut node.as_mut_ptr(),
            )
        } as u32;
        match rc {
            0 => Ok(Node(unsafe { node.assume_init() })),
            DQLITE_ERROR => Err(DQliteError::CreateError(
                CString::from(unsafe { CStr::from_ptr(dqlite_node_errmsg(node.as_mut_ptr())) })
                    .into_string()
                    .ok()
                    .unwrap_or_else(|| String::new()),
            )),
            DQLITE_MISUSE => Err(DQliteError::Misuse),
            DQLITE_NOMEM => Err(DQliteError::NoMem),
            n => Err(DQliteError::Unknown(n)),
        }
    }

    pub fn set_bind_address(&mut self, addr: &str) -> Result<(), DQliteError> {
        let caddr = CString::new(addr).unwrap();
        // println!("caddr: {:?}", caddr);
        let rc = unsafe {
            dqlite_node_set_bind_address(&mut self.0, caddr.as_bytes().as_ptr() as *const i8)
        } as u32;
        // println!("set bind addr rc: {}", rc);
        match rc {
            0 => Ok(()),
            DQLITE_ERROR => Err(DQliteError::Error(
                CString::from(unsafe { CStr::from_ptr(dqlite_node_errmsg(&mut self.0)) })
                    .into_string()
                    .ok()
                    .unwrap_or_else(|| String::new()),
            )),
            DQLITE_MISUSE => Err(DQliteError::Misuse),
            DQLITE_NOMEM => Err(DQliteError::NoMem),
            n => Err(DQliteError::Unknown(n)),
        }
    }
}

#[derive(Error, Debug)]
pub enum DQliteError {
    #[error("unknown error code {0}")]
    Unknown(u32),
    #[error("create error `{0}`")]
    CreateError(String),
    #[error("error `{0}`")]
    Error(String),
    #[error("dqlite misuse")]
    Misuse,
    #[error("dqlite no mem")]
    NoMem,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn e_to_e() {
        let mut node = Node::new("127.0.0.1:5005", "/tmp/dqlite-rs").unwrap();
        println!("got node: {:?}", node);

        node.set_bind_address("127.0.0.1:5005").unwrap();
        println!("successfully set bind address");
    }
}
