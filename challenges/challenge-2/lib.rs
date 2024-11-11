#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 2: Implement voter registration, proposal management, and voting in your Dao.
//
// - **Difficulty**: Mid
// - **Submission Criteria:** ink! contract must
//     - Use a storage-optimized data-structure `Mapping` or `StorageVec`
//     - Store registered members, member votes, and proposals to vote on.
//     - Implement methods to register and de-register members.
//     - Implement methods to create proposals and a method to vote on proposals.
//     - Unit tests for adding members, votes, and proposals.
// - **Submission Guidelines:**
//     - Verify with R0GUE DevRel, and post on X.
// - **Prize:** sub0 merch

#[ink::contract]
mod dao {
    use ink::{
        prelude::string::String,
        storage::{Mapping, StorageVec},
    };
    use minidao_common::*;

    #[derive(Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct BasicProposal {
        pub vote_count: u32,
    }

    #[ink(storage)]
    pub struct Dao {
        name: String,
        proposals: Mapping<u32, BasicProposal>,
        proposal_count: u32,
        voters: StorageVec<AccountId>,
        voter_count: Mapping<AccountId, u32>,
    }

    impl Dao {
        // Constructor that initializes the values for the contract.
        #[ink(constructor)]
        pub fn new(name: String) -> Self {
            Self { name, proposals: Mapping::new(), proposal_count: 0 , voters: StorageVec::new(), voter_count: Mapping::new()}
        }

        // Constructor that initializes the default values for the contract.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn get_name(&self) -> String {
            // - Returns the name of the Dao
            self.name.clone()
        }

        #[ink(message)]
        pub fn register_voter(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterAlreadyRegistered` if the voter is registered
            // - Success: Register a new `voter` to the Dao

            let caller = self.env().caller();
            self.voters.push(&caller);
            
            Ok(())
        }

        #[ink(message)]
        pub fn deregister_voter(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Deregister a new `voter` from the Dao
            let caller = self.env().caller();
            if !self.has_voter(caller) {
                return Err(DaoError::VoterNotRegistered);
            }
            for i in 0..self.voters.len() {
                if let Some(_) = self.voters.get(i) {
                    self.voters.clear_at(i);
                }
            }

            Ok(())
        }

        #[ink(message)]
        pub fn has_voter(&self, voter: AccountId) -> bool {
            // - Success: Return if the voter is registered.
            for i in 0..self.voters.len() {
                if let Some(registered_voter) = self.voters.get(i) {
                    if registered_voter == voter {
                        return true;
                    }
                }
            }
            false

        }

        #[ink(message)]
        pub fn create_proposal(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Create a new proposal that stores `votes` from `voters`
            let caller = self.env().caller();
            if !self.has_voter(caller) {
                return Err(DaoError::VoterNotRegistered);
            }
            let current_proposal_count = self.proposal_count;

            let proposal = BasicProposal {
                vote_count: 0
            };
            self.proposals.insert(current_proposal_count, &proposal);
            self.proposal_count += 1;
        
            Ok(())
        }

        #[ink(message)]
        pub fn remove_proposal(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Error: Throw error `DaoError::ProposalDoesNotExist` if the proposal is not created
            // - Success: Create a new proposal that stores `votes` from `voters`

            let caller = self.env().caller();
            if !self.has_voter(caller) {
                return Err(DaoError::VoterNotRegistered);
            }

            let proposal = self.proposals.get(proposal_id);
            if let Some(_) = proposal {
                self.proposals.remove(proposal_id);
                self.proposal_count -= 1;
            } else {
                return Err(DaoError::ProposalDoesNotExist);
            }

            Ok(())
        }

        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u32) -> Option<BasicProposal> {
            // - Success: Returns the proposal detail
            let proposal = self.proposals.get(proposal_id);
            proposal
        }

        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Error: Throw error `Error::ProposalDoesNotExist` if the proposal is not created
            // - Success: Vote on the proposal
            let caller = self.env().caller();
            if !self.has_voter(caller) {
                return Err(DaoError::VoterNotRegistered);
            }
            let voter_count = self.voter_count.get(caller).unwrap_or(0);
            self.voter_count.insert(caller, &(voter_count + 1));

            if let Some(mut proposal) = self.proposals.get(proposal_id) { 
                proposal.vote_count += 1;
                self.proposals.insert(proposal_id, &proposal);

            } else {
                return Err(DaoError::ProposalDoesNotExist);
            }


            Ok(())
        }

        #[ink(message)]
        pub fn vote_count(&self, voter: AccountId) -> u32 {
            // - Returns the number of `votes` a Dao `voter` voted
            let count = self.voter_count.get(voter).unwrap_or(0);
            count
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::dao::Dao;

        fn default_accounts(
        ) -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<Environment>()
        }

        #[ink::test]
        fn test_voter_registration() {
            let mut dao = Dao::new("DAO1".to_string());
            let default_accounts = default_accounts();
            let result = dao.register_voter();
            assert_eq!(result.is_ok(), true);
            assert_eq!(dao.has_voter(default_accounts.alice), true);
        }

        #[ink::test]
        fn test_proposal_management() {
            let mut dao = Dao::new("DAO1".to_string());
            let default_accounts = default_accounts();
            let result = dao.register_voter();
            assert_eq!(result.is_ok(), true);
            assert_eq!(dao.has_voter(default_accounts.alice), true);
            
            let result = dao.create_proposal();
            assert_eq!(result.is_ok(), true);
            assert_eq!(dao.proposal_count, 1);

        }

        #[ink::test]
        fn test_vote() {
            let mut dao = Dao::new("DAO1".to_string());
            let default_accounts = default_accounts();
            let result = dao.register_voter();
            assert_eq!(result.is_ok(), true);
            let result = dao.create_proposal();
            assert_eq!(result.is_ok(), true);
            assert_eq!(dao.proposal_count, 1);
            let result = dao.vote(0);
            assert_eq!(result.is_ok(), true);
            assert_eq!(dao.vote_count(default_accounts.alice), 1);
        }
    }
}
