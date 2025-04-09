#[allow(warnings)]
mod bindings;

pub mod up_core_api {
    use crate::bindings::exports::uprotocol::basic::{
        uuid::Uuid,
        uuri::Uuri,
    };
}

pub mod up_l1 {
    use crate::bindings::exports::uprotocol::basic::{
        utransport::Guest as UTransport,
    };
}
