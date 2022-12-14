use std::ops::Div;

use ext_rainbow::RainbowExt;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{env, near_bindgen, ext_contract,Gas,log, PromiseError,Promise, };
use serde::{Serialize,Deserialize};

pub const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(ext_lts)]
pub trait Lts {
    fn ft_transfer (&mut self, receiver_id:String, amount:String, memo:String);
}

//Define Rainbow Bridge contract
#[ext_contract(ext_rainbow)]
pub trait Rainbow {
    fn migrate_to_ethereum (&mut self,eth_recipient:String);
}



// VOTE
// Vote structor 
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
 pub struct Vote{
    pub address: String,
    pub vote:u8,
    pub time_of_vote:u64,
 }

  // Vote implementation 
  impl Vote {
    // Initialise a new vote
    pub fn new() -> Self{
        Self {
            address: String::new(),
            vote:0,
            time_of_vote:0,
        }
    }
 }

 // Council Proposal
// Proposal structor
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct CouncilProposal{
    pub id:String,
    pub proposal_type: u8,
    pub proposal_name: String,
    pub description: String,
    pub amount: u128,
    pub proposal_creator: String,
    pub votes_for: u32,
    pub votes_against: u32,
    pub time_of_creation:u64,
    pub duration_days:u64,
    pub duration_hours:u64,
    pub duration_min:u64,
    pub list_voters:Vec<String>,
    pub votes:Vec<Vote>,
    pub receiver:String
}

impl CouncilProposal{
    pub fn new() -> Self{
        Self{
            id:"".to_string(),
            proposal_type:0,
            proposal_name: String::new(),
            description: String::new(),
            amount:0,
            proposal_creator: String::new(),
            votes_for: 0,
            votes_against: 0,
            time_of_creation:0,
            duration_days:0,
            duration_hours:0,
            duration_min:0,
            list_voters:Vec::new(),
            votes:Vec::new(),
            receiver: String::new(),
        }
    }

    // Create a new vote 
    // Returns a propsal contains the new vote 
    pub fn create_vote(&mut self, vote:u8) -> Self{
        for i in self.list_voters.clone(){
            assert!(
                env::signer_account_id().to_string() != i,
                "You already voted"
            );
        }
        let v = Vote{
            address: env::signer_account_id().to_string(),
            vote:vote,
            time_of_vote:env::block_timestamp(),
        };
        self.votes.push(v);
        if vote==0 {
            self.votes_against=self.votes_against+1;
        }else{
            self.votes_for=self.votes_for+1;
        }
        self.list_voters.push(env::signer_account_id().to_string());
        Self { 
            id:self.id.clone(),
            proposal_type:self.proposal_type,
            proposal_name: self.proposal_name.clone(), 
            description: self.description.clone(),
            amount: self.amount,
            proposal_creator: self.proposal_creator.clone(),
            votes_for: self.votes_for, 
            votes_against: self.votes_against, 
            time_of_creation: self.time_of_creation, 
            duration_days: self.duration_days, 
            duration_hours: self.duration_hours, 
            duration_min: self.duration_min, 
            list_voters: self.list_voters.clone(),
            votes: self.votes.clone(),
            receiver: self.receiver.clone()
        }
    }

    // Get the end time of a proposal 
    pub fn end_time(&self) -> u64 {
        log!("{}",self.time_of_creation+(self.duration_days*86400000000000+self.duration_hours*3600000000000+self.duration_min*60000000000));
        self.time_of_creation+(self.duration_days*86400000000000+self.duration_hours*3600000000000+self.duration_min*60000000000)
    }

    // Check if the time of a proposal is end or not 
    pub fn check_proposal(&self)->bool{
        if (env::block_timestamp() > self.end_time()) && (self.votes_for > self.votes_against){
            return true;
        }
        return false;
    } 

}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TreasuryDao {
    stakers: Vec<String>,
    members: UnorderedMap<String,u8>,
    proposals: Vec<CouncilProposal>,
}

// Define the default, which automatically initializes the contract
impl Default for TreasuryDao {
    fn default() -> Self {
        panic!("Contract is not initialized yet")
    }
}

// Make sure that the caller of the function is the owner
fn assert_self() {
    assert_eq!(
        env::current_account_id(),
        env::predecessor_account_id(),
        "Can only be called by owner"
    );
}

// Implement the contract structure
// To be implemented in the front end
#[near_bindgen]
impl TreasuryDao {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            stakers: Vec::new(),
            members : UnorderedMap::new(b"m"),
            proposals : Vec::new(),
        }
    }

    pub fn init(&mut self) {
        assert_self();
        self.members.insert(&env::current_account_id().to_string(), &0);
    }

    // delete all members 
    pub fn delete_all (&mut self) {
        assert_self();
        self.members.clear();
    }

    // get all councils
    pub fn get_councils(&self) -> Vec<String> {
        let mut vec = Vec::new();
        for i in self.members.keys() {
            if self.members.get(&i).unwrap() == 0 {
                vec.push(i);
            }
        }
        vec
    }

    // get all communities
    pub fn get_communities(&self) -> Vec<String> {
        let mut vec = Vec::new();
        for i in self.members.keys() {
            if self.members.get(&i).unwrap() == 1 {
                vec.push(i);
            }
        }
        vec
    }

    pub fn check_member(&self, account:String) -> bool {
        let mut result=false;
        for i in 0..self.members.keys_as_vector().len() {
            if self.members.keys_as_vector().get(i).unwrap() == account {
                result = true;
                break;
            } 
        }
        result
    }

    pub fn check_council (&self, account:String) -> bool {
        if self.check_member(account.clone()) == false {
            return false ;
        }else {
            if self.members.get(&account).unwrap() == 0 {
                return true;
            }else {
                return false;
            }
        }
    }

    // Create a new proposal 
    pub fn create_proposal (
        &mut self,
        id:String,
        proposal_type:u8,
        proposal_name: String,
        description: String,
        amount:u128,
        duration_days: u64,
        duration_hours: u64,
        duration_min: u64,
        receiver:String,
    ){
        assert_eq!(
            self.check_council(env::signer_account_id().to_string()),
            true,
            "Proposals can be created only by the councils"
        );
        let proposal=CouncilProposal{
            id:id,
            proposal_type:proposal_type,
            proposal_name: proposal_name,
            description: description,
            amount:amount,
            proposal_creator: env::signer_account_id().to_string(),
            votes_for: 0,
            votes_against: 0,
            time_of_creation:env::block_timestamp(),
            duration_days:duration_days,
            duration_hours:duration_hours,
            duration_min:duration_min,
            list_voters:Vec::new(),
            votes:Vec::new(),
            receiver:receiver
        };
        self.proposals.push(proposal);
    }

    // Replace a proposal whith a new one 
    pub fn replace_proposal(&mut self, proposal: CouncilProposal){
        assert_eq!(
            self.check_member(env::signer_account_id().to_string()),
            true,
            "Proposals can be created only by members"
        );
        let mut index =0;
        for i in 0..self.proposals.len(){
            match self.proposals.get(i){
                Some(p) => if p.id==proposal.id {
                    index=i;
                },
                None => panic!("There is no PROPOSALs"),
            }
        }
        self.proposals.swap_remove(index);
        self.proposals.insert(index, proposal);
    }

    // Get all proposals 
    pub fn get_proposals(&self) -> Vec<CouncilProposal>{
        self.proposals.clone()
    }

    // Get a spsific proposal 
    pub fn get_specific_proposal(&self, id: String) -> CouncilProposal{
        let mut proposal= CouncilProposal::new();
        for i in 0..self.proposals.len() {
            match self.proposals.get(i){
                Some(p) => if p.id==id {
                    proposal=p.clone();
                },
                None => panic!("There is no DAOs"),
            }
        }
        proposal
    }

    // add a vote 
    pub fn add_vote(
        &mut self,
        id: String,
        vote: u8
    ){
        if env::block_timestamp() < self.get_specific_proposal(id.clone()).end_time() {
            assert_eq!(
                self.check_member(env::signer_account_id().to_string()),
                true,
                "You must be one of the dao members to vote"
            );
            let proposal =self.get_specific_proposal(id.clone()).create_vote(vote);
            self.replace_proposal(proposal);
        }else {
            panic!("Proposal has been expired");
        }
    }

    pub fn get_end_time(&self , id: String) -> u64{
        self.get_specific_proposal(id.clone()).end_time()
    }

    // add a council
    pub fn add_council(&mut self, account:String){
        assert_eq!(
            self.check_council(env::signer_account_id().to_string()),
            true,
            "To add a council you must be one of the councils"
        );
        self.members.insert(&account, &0);
    }

    // delete all stakers 
    pub fn delete_stakers (&mut self) {
        assert_self();
        self.stakers.clear();
    }

    // delete specific staker 
    pub fn delete_specific_staker (&mut self, account:String) {
        assert_self();
        for i in 0..self.stakers.len(){
            if self.stakers.get(i).unwrap() == &account {
                self.stakers.swap_remove(i);
                break;
            }
        }
    }

    // get list of stakers
    pub fn get_stakers (&self) -> Vec<String> {
        self.stakers.clone()
    }

    // Check staker
    pub fn check_staker (&self, account:String) -> bool{
        let mut existance = false;
        for i in 0..self.stakers.len() {
            if self.stakers.get(i).unwrap() == &account {
                existance = true;
            }
        }
        existance
    }

    // add a staker
    pub fn add_staker (&mut self, account:String) {
        assert_eq!(
            env::predecessor_account_id().to_string(),
            "rewarder_contract.testnet".to_string(),
            "You are not authorized to execute this function"
        );
        if self.check_staker(account.clone()) == false{
            self.stakers.push(account);
        }
    }

    // add community
    pub fn add_community (&mut self,account:String) {
        if self.check_staker(account.clone()) == true {
            self.members.insert(&account, &1);
        }else {
            panic!("You must be a staker to join community");
        }
    }

    // check the proposal and return a message
    pub fn check_the_proposal(&self,id: String) -> String{
        let proposal=self.get_specific_proposal(id);
        let check= proposal.check_proposal();
        if check==true {
            let msg="Proposal accepted".to_string();
            msg
        }else{
            let msg="Proposal refused".to_string();
            msg
        }
    }

    // fund function 
    pub fn fund (&mut self,account:String,amount:u128){
        assert_eq!(
            env::signer_account_id().to_string(),
            "alach.testnet".to_string(),
            "You are not authorized to execute this function"
        );
        let account_lts= "light-token.testnet".to_string().try_into().unwrap();
        ext_lts::ext(account_lts)
        .with_static_gas(Gas(2 * TGAS))
        .with_attached_deposit(1)
        .ft_transfer(account,(amount*100000000).to_string(),"".to_string());
    }

    #[payable]
    pub fn process_borrow(&mut self,eth_recipient:String)->Promise{
        
        let mut eth_addr=eth_recipient.clone();
        if(eth_addr.len()==42){
            eth_addr.remove(0);
            eth_addr.remove(0);
        }
        let rainbow_account= "enear.goerli.testnet".to_string().try_into().unwrap();
        let promise =ext_rainbow::ext(rainbow_account)
        .with_static_gas(Gas(2 * TGAS))
        .with_attached_deposit(12000000000000000000000000)
        .migrate_to_ethereum(eth_addr);
        return promise.then( // Create a promise to callback withdraw_callback
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(10 * TGAS))
            .process_callback()
            )


        
    }

        
    #[private] // Public - but only callable by env::current_account_id()
    pub fn process_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError> ) {
            if call_result.is_err() {
            panic!("There was an error contacting the pool contract");
            }
        }
}
#[cfg(test)]
mod tests {
    use std::{ptr::null, arch::x86_64::_mm_undefined_pd};

    use super::*;
    //testing init function to initialize the smart contract after deployemnt
    #[test]
    fn test_init(){
        let mut contract = TreasuryDao::new();
        contract.init();
        assert_eq!(contract.check_council(env::current_account_id().to_string()), true);
    }
    // testing delete all members function 
    #[test]
    fn test_delete_all(){
        let mut contract = TreasuryDao::new();
        contract.init();
        contract.delete_all();
        assert_eq!(contract.check_member(env::current_account_id().to_string()), false);

    }

    //testing create proposal function
    #[test]
    fn test_create_proposal(){
        let mut contract = TreasuryDao::new();
        contract.init();
        contract.create_proposal("id".to_string(), 0,"azerty".to_string(), "description".to_string(), 1, 0, 0, 1, env::current_account_id());
        assert_ne!(contract.get_specific_proposal("id".to_string()),null());
    }
    
    //testing replace proposal function

    #[test]
    fn test_replace_proposal(){
        let mut contract = TreasuryDao::new();
        contract.create_proposal("azerty".to_string(), 1,"qwerty".to_string(), "description".to_string(), 1, 0, 0, 1, env::current_account_id());
        let mut proposal = contract::CouncilProposal{
            id:"azerty".to_string(),
            proposal_type:0,
            proposal_name: String::new(),
            description: String::new(),
            amount:0,
            proposal_creator: String::new(),
            votes_for: 0,
            votes_against: 0,
            time_of_creation:0,
            duration_days:0,
            duration_hours:0,
            duration_min:0,
            list_voters:Vec::new(),
            votes:Vec::new(),
            receiver: String::new(),

        };
        contract.replace_proposal(proposal);
        assert_eq!(contract.get_specific_proposal("azerty".to_string()), proposal);
    }

    //testing add vote function
    #[test]
    fn test_add_vote(){
        let mut contract = TreasuryDao::new();
        contract.init();
        contract.create_proposal("azerty".to_string(), 1,"qwerty".to_string(), "description".to_string(), 1, 0, 0, 1, env::current_account_id());
        let proposal = contract.get_specific_proposal("azerty".to_string());
        contract.vote(&proposal.id, 1);
        assert_eq!(&proposal.votes.len(), 1);
    }
    //testing add council function 
    #[test]
    fn test_add_council(){
        let mut contract = TreasuryDao::new();
        contract.init();
        contract.add_council("oussema.testnet".to_string);
        assert!(contract.check_council("oussema.testnet".to_string());)
    }

}
