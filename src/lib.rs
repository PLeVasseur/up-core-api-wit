#[allow(warnings)]
mod bindings;

pub mod up_core_api {
    use crate::bindings::exports::uprotocol::basic::{
        uuid::Uuid,
        uuri::Uuri,
        utransport::Guest,
    };
}


