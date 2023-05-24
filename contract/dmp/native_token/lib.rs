#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use self::native_token::{NativeTokenRef};

#[openbrush::contract]
pub mod native_token {
    // imports from openbrush
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::extensions::{ burnable::*, mintable::*, wrapper::*, metadata::* },
        },
        traits::{ Storage, String },
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct NativeToken {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        wrapper: wrapper::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP22 for NativeToken {}
    impl Ownable for NativeToken {}
    impl PSP22Wrapper for NativeToken {}
    impl PSP22Metadata for NativeToken {}

    impl PSP22Burnable for NativeToken {
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._burn_from(account, amount)
        }
    }
    impl PSP22Mintable for NativeToken {
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._mint_to(account, amount)
        }
    }

    impl NativeToken {
        #[ink(constructor)]
        pub fn new(
            initial_supply: Balance,
            name: Option<String>,
            symbol: Option<String>,
            decimal: u8
        ) -> Self {
            let mut _instance = Self::default();
            let caller = Self::env().caller();
            _instance._mint_to(caller, initial_supply).expect("Should mint");
            _instance._init_with_owner(caller);
            _instance.metadata.name = name;
            _instance.metadata.symbol = symbol;
            _instance.metadata.decimals = decimal;
            _instance
        }
    }
}