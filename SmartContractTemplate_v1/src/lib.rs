
#![no_std]
use gstd::{errors::Result, msg , prelude::*,ActorId};
use gmeta::{Metadata};
use hashbrown::HashMap;
use io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));



static mut STATE:Option<HashMap<ActorId, u128>> = None;



fn state_mut() -> &'static mut HashMap<ActorId,u128> {

    let state = unsafe { STATE.as_mut()};

    unsafe { state.unwrap_unchecked() }


}


#[no_mangle]
extern "C" fn init () {

   unsafe { STATE = Some(HashMap::new())}


}

#[no_mangle]
extern "C" fn handle(){

    let input_business: InputBusiness = msg::load().expect("Error while loading the payload");

    match input_business.event {
       
        Action::Comprar => {

            msg::reply(u16::from(input_business.amount), 0)
            .expect("Error in sending a reply message");
        }, 
       
        Action::Pagar => {

            msg::reply(u16::from(input_business.amount), 0)
            .expect("Error in sending a reply message");

           
        },
        Action::Total => {

            msg::reply(input_business.amount, 0)
            .expect("Error in sending a reply message");

           
        }
    }

 

}



    #[no_mangle]
    extern "C" fn state() {

        // We create a state variable.
        let state: <ContractMetadata as Metadata>::State =
            state_mut().iter().map(|(k, v)| (*k, *v)).collect();
         
        // Generate response message
        msg::reply(state, 0).expect("failed to encode or reply from `state()`");
    }
