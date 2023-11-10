
#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{InOut,Metadata};


#[derive(Default, Encode, Decode, Clone, TypeInfo)]
pub struct CustomStruct {
    example: String,
   
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Action {
    
    // Add Actions
    Comprar,
    Pagar,
    Total
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum  Event {
    
    // Add Events
    ExampleEvent
}

pub struct ContractMetadata;

impl Metadata for ContractMetadata{
     type Init = ();
     type Handle = InOut<InputBusiness,u16>; // Acciones como entrada y  eventos como salida.
     type Others = ();
     type Reply=();
     type Signal = ();
     type State = Vec<(ActorId, u128)>; // Estado 

}

#[derive(Encode, Decode, TypeInfo)]

pub struct InputBusiness{
    pub event: Action, 
    pub amount: u16,
}