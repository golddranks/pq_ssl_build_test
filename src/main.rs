extern crate native_tls;
extern crate pq_sys;

use native_tls::TlsConnector;


pub fn main() {
    let connector = TlsConnector::new().unwrap();
    let result = unsafe { pq_sys::PQconnectStart(&*b"test" as *const u8 as *const i8)};
}
