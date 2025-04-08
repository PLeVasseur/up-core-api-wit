#[allow(warnings)]
mod bindings;

use crate::bindings::exports::name::Guest;
use crate::bindings::exports::name::FlagType;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn dummy(_name: wit_bindgen_rt::async_support::FutureReader<FlagType>) -> wit_bindgen_rt::async_support::FutureReader<FlagType> {

        todo!()
    }
}

bindings::export!(Component with_types_in bindings);
