#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 3: Connect your DAO to the Super DAO with registration and voting
//
// - **Difficulty**: Mid
// - **Submission Criteria:** ink! contract must
//     - Import the Super DAO trait>
//     - Store Super DAO contract address.
//     - Register contract as member of Super DAO - using trait-based contract calling.
//     - Vote on proposals in the Super DAO - using trait-based contract calling.
//     - Create proposals to call another contract - using trait-based contract calling.
//     - E2E test for cross-contract call.
// - **Submission Guidelines:**
//     - Verify with R0GUE DevRel, and post on X.
// - **Prize:** Sub0 Merch & ink! sports towel

#[ink::contract]
mod dao {
    use ink::{
        contract_ref,
        prelude::{string::String, vec},
        storage::StorageVec,
    };
    use minidao_common::*;
    use superdao_traits::{Call, ContractCall, SuperDao, Vote};

    #[ink(storage)]
    pub struct Dao {
        superdao: contract_ref!(SuperDao),
        voters: StorageVec<AccountId>,
        name: String,
    }

    impl Dao {
        // Constructor that initializes the values for the contract.
        #[ink(constructor)]
        pub fn new(name: String, superdao: AccountId) -> Self {
            // Register your Dao as a member of the Superdao.
            let mut instance = Self {
                name,
                superdao: superdao.into(),
                voters: StorageVec::new(),
            };
            instance
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
            if self.has_voter(self.env().caller()) {
                return Err(DaoError::VoterAlreadyRegistered);
            }
            let caller = self.env().caller();
            self.voters.push(&caller);

            self.superdao.register_member()?;


            
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
            self.superdao.deregister_member();

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
        pub fn create_superdao_contract_call_proposal(
            &mut self,
            call: ContractCall,
        ) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Create a SuperDao proposal to call a contract method.
            if !self.has_voter(self.env().caller()) {
                return Err(DaoError::VoterNotRegistered);
            }
            self.superdao.create_proposal(Call::Contract(call))?;

            Ok(())
        }

        #[ink(message)]
        pub fn vote_proposal(&mut self, proposal_id: u32, vote: bool) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Vote a SuperDao proposal.
            if !self.has_voter(self.env().caller()) {
                return Err(DaoError::VoterNotRegistered);
            }
            if vote {
                self.superdao.vote(proposal_id, Vote::Aye)?;
            } else {
                self.superdao.vote(proposal_id, Vote::Nay)?;
            }

            Ok(())
        }
    }

    // #[cfg(test)]
    // mod tests {
    //     use super::*;

    //     #[ink::test]
    //     fn test_create_superdao_contract_call_proposal() {
    //         todo!("Challenge 3");
    //     }

    //     #[ink::test]
    //     fn test_vote_superdao_proposal() {
    //         todo!("Challenge 3");
    //     }
    // }



    // #[cfg(all(test, feature = "e2e-tests"))]
    // mod e2e_tests {

    //     use super::*;
    //     use ink_e2e::ContractsBackend;
    //     // use ink_e2e::{test, DefaultEnvironment};
    //     type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
        
    //     #[ink_e2e::test]
    //     async fn create_superdao_contract_call_proposal<Client: E2EBackend>(mut client: ink_e2e::Client) -> E2EResult<()> {
        
    //         // let mut constructor = SuperDao::new();
    //         // let contract = client
    //         //     .instantiate("superdao", &ink_e2e::alice(), &mut constructor)
    //         //     .submit()
    //         //     .await
    //         //     .expect("SuperDao instantiate failed");
    //         // let mut call_builder = contract.call_builder::<SuperDao>();
    //         // let call = call_builder.flip_and_get_v1();
        
    //         // // when
    //         // let result = client
    //         //     .call(&ink_e2e::alice(), &call)
    //         //     .submit()
    //         //     .await
    //         //     .expect("Calling `flip_and_get` failed")
    //         //     .return_value();
        
    //         // assert!(!result);
        
    //         Ok(())
    //     }

    // }
}




