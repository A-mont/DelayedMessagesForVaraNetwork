
#![no_std]
use gmeta::Metadata;
use hashbrown::HashMap;
use io::*;
use gstd::{async_main, msg, prelude::*, ActorId};



#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));


#[derive(Debug, Clone, Default)]
struct Actors {  
    actors: HashMap<ActorId, u128>,
}


impl Actors {

    async fn delayed_message_0s( &mut self, amount_tokens: u128){
       
        let currentstate = state_mut();
        let address_ft = addresft_state_mut(); 
        let payload = FTAction::Mint(amount_tokens);
        let delay = 0;     
        let _delete_message =msg::send_delayed(address_ft.ft_program_id, payload, 0, delay);     
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

    }


    async fn delayed_message_10s( &mut self, amount_tokens: u128){
       
        let currentstate = state_mut();
        let address_ft = addresft_state_mut(); 
        let payload = FTAction::Mint(amount_tokens);
        let delay = 3;     
        let _delete_message =msg::send_delayed(address_ft.ft_program_id, payload, 0, delay);
        
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

    }

    async fn delayed_message_20s( &mut self, amount_tokens: u128){
       
        let currentstate = state_mut();
        let address_ft = addresft_state_mut(); 
        let payload = FTAction::Mint(amount_tokens);
        let delay = 6;     
        let _delete_message =msg::send_delayed(address_ft.ft_program_id, payload, 0, delay);
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

        
    }

    async fn delayed_message_30s( &mut self, amount_tokens: u128){
       
        let currentstate = state_mut();
        let address_ft = addresft_state_mut(); 
        let payload = FTAction::Mint(amount_tokens);
        let delay = 10;     
        let _delete_message =msg::send_delayed(address_ft.ft_program_id, payload, 0, delay);
        
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

        
    }


    async fn delayed_message_1m( &mut self, amount_tokens: u128){
       
        let currentstate = state_mut();
        let address_ft = addresft_state_mut(); 
        let payload = FTAction::Mint(amount_tokens);
        let delay = 20;     
        let _delete_message =msg::send_delayed(address_ft.ft_program_id, payload, 0, delay);
        
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

        
    }

    async fn delayed_message_3m( &mut self, amount_tokens: u128){
       
        let currentstate = state_mut();
        let address_ft = addresft_state_mut(); 
        let payload = FTAction::Mint(amount_tokens);
        let delay = 60;     
        let _delete_message =msg::send_delayed(address_ft.ft_program_id, payload, 0, delay);
        
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

        
    }


    async fn delayed_message_5m( &mut self, amount_tokens: u128){
       
        let currentstate = state_mut();
        let address_ft = addresft_state_mut(); 
        let payload = FTAction::Mint(amount_tokens);
        let delay = 100;     
        let _delete_message =msg::send_delayed(address_ft.ft_program_id, payload, 0, delay);
        
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

        
    }

    

  
   
}


static mut ACTORS:Option<Actors> = None;

static mut STATE:Option<HashMap<ActorId, u128>> = None;

static mut ADDRESSFT:Option<InitFT> = None;



fn actors_state_mut() -> &'static mut Actors  {

    unsafe { ACTORS.get_or_insert(Default::default()) }


}




fn state_mut() -> &'static mut HashMap<ActorId,u128> {

    let state = unsafe { STATE.as_mut()};

    unsafe { state.unwrap_unchecked() }


}

fn addresft_state_mut() -> &'static mut InitFT {


    let addressft = unsafe { ADDRESSFT.as_mut()};

    unsafe { addressft.unwrap_unchecked() }


}


#[no_mangle]
extern "C" fn init () {

    let config: InitFT = msg::load().expect("Unable to decode InitFT");

    let _actors = Actors {
        ..Default::default()
    };

    if config.ft_program_id.is_zero() {
        panic!("FT program address can't be 0");
    }

    let initft = InitFT {
        ft_program_id: config.ft_program_id
    };

    unsafe {
        ADDRESSFT = Some(initft);
    }

   unsafe { STATE = Some(HashMap::new())}

}

#[async_main]
async fn main(){

    let action: Action = msg::load().expect("Could not load Action");

    let actors = unsafe { ACTORS.get_or_insert(Actors::default()) };

    match action {


        Action::FTDelayedMessage0s(amount) =>  {
         
            actors.delayed_message_0s(amount).await;   
        },
        Action::FTDelayedMessage10s(amount) =>  {
        
                actors.delayed_message_10s(amount).await;
               
            },
        Action::FTDelayedMessage20s(amount) => {

                actors.delayed_message_20s(amount).await;             
            },

        Action::FTDelayedMessage30s(amount) => {
     
                actors.delayed_message_30s(amount).await;
            },
        Action::FTDelayedMessage1m(amount) => {
     
                actors.delayed_message_1m(amount).await;
            },
            Action::FTDelayedMessage3m(amount) => {
     
                actors.delayed_message_3m(amount).await;
            },
            Action::FTDelayedMessage5m(amount) => {
     
                actors.delayed_message_5m(amount).await;
            },
            Action::FTAllDelayedMessages(amount) => {
    

                actors.delayed_message_0s(amount).await; 
                actors.delayed_message_10s(amount).await;
                actors.delayed_message_30s(amount).await;
                actors.delayed_message_1m(amount).await; 
                actors.delayed_message_3m(amount).await;         
                actors.delayed_message_5m(amount).await; 
            },
           
            };

}

    

    #[no_mangle]
    extern "C" fn state() {
     
        let state: <ContractMetadata as Metadata>::State =
            state_mut().iter().map(|(k, v)| (*k, *v)).collect();
         
        msg::reply(state, 0).expect("failed to encode or reply from `state()`");
    }


#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitFT {
   
    pub ft_program_id: ActorId,
}



