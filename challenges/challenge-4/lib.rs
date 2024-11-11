#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 4: Support creating cross-chain proposals to the Super DAO

// - **Difficulty**: Advanced
// - **Submission Criteria:** ink! contract must
//     - Support creating cross-chain proposals to the Super DAO (XCM)
//     - A deployed contract on Pop Network Testnet
//     - Have a cross-chain proposal successfully executed
// - **Submission Guidelines:**
//     - Verify with R0GUE DevRel, and post on X.
// - **Prize:** Sub0 merch

#[ink::contract]
mod dao {
    use ink::{contract_ref, prelude::string::String, storage::StorageVec, xcm::prelude::*};
    use minidao_common::*;
    use superdao_traits::{Call, ChainCall, SuperDao, Vote};

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
            instance.superdao.register_member();
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
        pub fn create_superdao_contract_call_proposal(
            &mut self,
            call: ChainCall,
        ) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Create a SuperDao proposal to call a contract method.
            if !self.has_voter(self.env().caller()) {
                return Err(DaoError::VoterNotRegistered);
            }
            self.superdao.create_proposal(Call::Chain(call))?;

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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn xcm_encoded_calls_helper() {
            let location = Location::here();

            let accounts = ink::env::test::default_accounts::<Environment>();

            let value: Balance = 10000000000;
            let asset: Asset = (Location::parent(), value).into();
            let beneficiary = AccountId32 {
                network: None,
                id: *accounts.alice.as_ref(),
            };

            let msg: Xcm<()> = Xcm::builder()
                .withdraw_asset(asset.clone().into())
                .buy_execution(asset.clone(), Unlimited)
                .deposit_asset(asset.into(), beneficiary.into())
                .build();

            let chain_call = ChainCall::new(&location, &msg);

            ink::env::debug_println!("dest: {:?}", hex::encode(chain_call.get_encoded_dest()));
            ink::env::debug_println!("msg: {:?}", hex::encode(chain_call.get_encoded_msg()));
        }
        
    }
}
