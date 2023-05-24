#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use self::nft_token::{ NftTokenRef, PSP37MintableRef };

#[openbrush::contract]
pub mod nft_token {
    use ink::prelude::vec::Vec;
    pub use openbrush::{
        contracts::{
            ownable::*,
            psp37::extensions::{ burnable::*, mintable::*, enumerable::*, batch::*, metadata::* },
        },
        modifiers,
        traits::{ Storage, String },
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct NftToken {
        #[storage_field]
        psp37: psp37::Data<Balances>,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP37 for NftToken {}
    impl Ownable for NftToken {}
    impl PSP37Batch for NftToken {}
    impl PSP37Enumerable for NftToken {}
    impl PSP37Metadata for NftToken {}

    impl PSP37Burnable for NftToken {
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        fn burn(
            &mut self,
            account: AccountId,
            ids_amounts: Vec<(Id, Balance)>
        ) -> Result<(), PSP37Error> {
            self._burn_from(account, ids_amounts)
        }
    }

    impl PSP37Mintable for NftToken {
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        fn mint(
            &mut self,
            account: AccountId,
            ids_amounts: Vec<(Id, Balance)>
        ) -> Result<(), PSP37Error> {
            self._mint_to(account, ids_amounts)
        }
    }

    impl NftToken {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
            let caller = _instance.env().caller();
            _instance._init_with_owner(caller);
            _instance
        }

        #[ink(message)]
        pub fn set_attribute(
            &mut self,
            id: Id,
            key: String,
            data: String
        ) -> Result<(), PSP37Error> {
            self._set_attribute(&id, &key, &data)
        }

        #[ink(message)]
        pub fn get_attribute(&self, id: Id, key: String) -> Option<Vec<u8>> {
            self._get_attribute(&id, &key)
        }

        // #[ink(message)]
        // pub fn transfer_nft(
        //     &mut self,
        //     to: AccountId,
        //     id: Id,
        //     value: Balance,
        //     data: Vec<u8>
        // ) -> Result<(), PSP37Error> {
        //     self.transfer(to, id, value, data)
        // }
    }
}