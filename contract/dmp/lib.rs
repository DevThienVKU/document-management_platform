#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod doc_manage_platform {
    use ink_prelude::vec;
    use scale::{ Decode, Encode };

    use native_token::native_token::NativeTokenRef;
    use nft_token::nft_token::{
        NftTokenRef,
        Id,
        PSP37Error,
        psp37_external::PSP37,
        psp37mintable_external::PSP37Mintable,
    };
    use ink_prelude::vec::Vec;
    // use ink::storage::Mapping as Map;
    use openbrush::{ traits::{ String } };

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotApproved,
        TokenExists,
        TokenNotFound,
        CannotInsert,
        CannotFetchValue,
        NotAllowed,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum UserRole {
        Developer,
        CoAuthor,
        Partner,
        Owner,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Version {
        pub contributor: AccountId,
        pub ipfs_hash: String,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct PostOwner {
        pub user: AccountId,
        pub user_role: UserRole,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct NftDocument {
        pub id: u32,
        pub owner: AccountId,
        pub title: String,
        pub members: Vec<PostOwner>,
        pub versions: Version,
    }

    #[ink(storage)]
    pub struct NftCollection {
        docs: Vec<NftDocument>,
        native_token_ref: NativeTokenRef,
        nft_token_ref: NftTokenRef,
    }

    #[ink(event)]
    pub struct DocumentCreated {
        #[ink(topic)]
        doc_id: Id,
        #[ink(topic)]
        title: String,
        #[ink(topic)]
        hash: String,
    }

    #[ink(event)]
    pub struct Joined {
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        doc_id: u32,
        #[ink(topic)]
        balance: Balance,
    }

    #[ink(event)]
    pub struct VersionCreated {
        #[ink(topic)]
        contributor: AccountId,
        #[ink(topic)]
        doc_id: u32,
        #[ink(topic)]
        hash: String,
    }

    impl NftCollection {
        #[ink(constructor)]
        pub fn new(
            native_token_code_hash: Hash,
            nft_token_code_hash: Hash,
            initial_supply: Balance,
            name: Option<String>,
            symbol: Option<String>,
            decimal: u8
        ) -> Self {
            let native_token_contract = NativeTokenRef::new(initial_supply, name, symbol, decimal)
                .code_hash(native_token_code_hash)
                .endowment(0)
                .salt_bytes([0xde, 0xad, 0xbe, 0xef])
                .instantiate();

            let nft_token_contract = NftTokenRef::new()
                .code_hash(nft_token_code_hash)
                .endowment(0)
                .salt_bytes([0xde, 0xad, 0xbe, 0xef])
                .instantiate();

            Self {
                docs: Vec::new(),
                native_token_ref: native_token_contract,
                nft_token_ref: nft_token_contract,
            }
        }

        #[ink(message)]
        pub fn get_nfts(&self) -> Vec<NftDocument> {
            self.docs.clone()
        }

        #[ink(message)]
        pub fn get_nfts_length(&self) -> u32 {
            self.docs.len() as u32
        }

        #[ink(message)]
        pub fn add_nft(&mut self, nft: NftDocument) {
            self.docs.push(nft);
        }

        //get document by ID
        #[ink(message)]
        pub fn get_document_by_id(&self, doc_id: u32) -> Option<NftDocument> {
            self.docs.get(doc_id as usize).cloned()
        }

        #[ink(message)]
        pub fn get_version(&self, doc_id: u32) -> Version {
            self.docs[doc_id as usize].versions.clone()
        }

        #[ink(message)]
        pub fn get_ipfs_hash_version(&self, doc_id: u32) -> String {
            self.docs[doc_id as usize].versions.ipfs_hash.clone()
        }

        #[ink(message)]
        pub fn get_contributors(&self, doc_id: u32) -> Vec<PostOwner> {
            self.docs[doc_id as usize].members.clone()
        }

        #[ink(message)]
        pub fn get_total_supply_nft_doc(&self, token_id: Option<Id>) -> Balance {
            self.nft_token_ref.total_supply(token_id)
        }

        #[ink(message)]
        pub fn get_balance_of_nft_doc(&self, token_id: Option<Id>) -> Balance {
            let caller = self.env().caller();
            self.nft_token_ref.balance_of(caller, token_id)
        }

        #[ink(message)]
        pub fn get_allowance_nft_doc(&self, operator: AccountId, token_id: Option<Id>) -> Balance {
            let caller = self.env().caller();
            self.nft_token_ref.allowance(caller, operator, token_id)
        }

        #[ink(message)]
        pub fn approve_nft_doc(
            &mut self,
            operator: AccountId,
            token_id: Option<Id>,
            value: Balance
        ) -> Result<(), PSP37Error> {
            self.nft_token_ref.approve(operator, token_id, value)
        }

        #[ink(message)]
        pub fn get_attribute_version_nft_doc(
            &self,
            token_id: Id,
            version_id: String
        ) -> Option<String> {
            self.nft_token_ref.get_attribute(token_id, version_id)
        }

        #[ink(message)]
        pub fn transfer_nft_doc(&mut self, to: AccountId, token_id: Id, value: u128, data: Vec<u8>) -> Result<(), PSP37Error> {
            // let caller = self.env().caller();
            self.nft_token_ref.transfer(to, token_id, value, data)
        }

        #[ink(message)]
        pub fn create_document(
            &mut self,
            title: String,
            ipfs_hash: String,
            co_author: Vec<(AccountId, u128)>
        ) -> Result<(), Error> {
            let doc_id = self.get_nfts_length();
            let caller = self.env().caller();

            let token_id: Id = Id::U32(doc_id);
            let token_balance: u128 = 100000;
            let version_id = "0";
            // let version_id = vec![init_version_id];
            let ids_amount: Vec<(Id, Balance)> = vec![(token_id.clone(), token_balance)];

            let _ = self.nft_token_ref.mint(caller, ids_amount);
            // key with String value, fix version_id to string to get value in key
            let _ = self.nft_token_ref.set_attribute(token_id.clone(), version_id.into(), ipfs_hash.clone());

            let mut arr_owner: Vec<PostOwner> = Vec::new();
            let author = PostOwner {
                user: caller,
                user_role: UserRole::Owner,
            };
            arr_owner.push(author);

            for (account, _balance) in co_author.iter() {
                let new_author = PostOwner {
                    user: *account,
                    user_role: UserRole::CoAuthor,
                };
                let _ = self.nft_token_ref.transfer(*account, token_id.clone(), *_balance, ipfs_hash.clone());
                arr_owner.push(new_author);

            }

            let new_version = Version {
                contributor: caller,
                ipfs_hash: ipfs_hash.clone(),
            };

            let new_doc = NftDocument {
                id: doc_id,
                owner: caller,
                title: title.clone(),
                members: arr_owner,
                versions: new_version.clone(),
            };
            self.add_nft(new_doc);

            Self::env().emit_event(DocumentCreated {
                doc_id: token_id,
                title,
                hash: ipfs_hash,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn create_version_document(
            &mut self,
            doc_id: u32,
            ipfs_hash: String
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            let _doc_id_convert = Id::U32(doc_id);

            let update_version = Version {
                contributor: caller,
                ipfs_hash: ipfs_hash.clone(),
            };

            let _version_id: String = "0".into();
            let _ = self.nft_token_ref.set_attribute(
                _doc_id_convert,
                _version_id,
                ipfs_hash.clone()
            );

            let _ = self.docs[doc_id as usize].versions = update_version;

            Self::env().emit_event(VersionCreated {
                contributor: caller,
                doc_id: doc_id,
                hash: ipfs_hash,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn reward_native_token(
            &self,
            status: bool,
            voter: Vec<(AccountId, bool)>,
            _doc_id: u32
        ) -> Result<(), Error> {
            let _caller = self.env().caller();
            if status {
                for each in voter {
                    if each.1 {
                        // reward native token for voter yes and authors
                        // transfer nft token for contributor
                        //3
                    }
                }
            } else {
                for each in voter {
                    if each.1 {
                        // reward native token for voter no
                        //
                    }
                }
            }
            Ok(())
        }
    }

    // #[cfg(all(test, feature = "e2e-tests"))]
    // mod e2e_tests {
    //     use super::NftCollectionRef;
    //     use ink_e2e::build_message;

    //     type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
        
    //     #[ink_e2e::test(additional_contracts = "token/psp22/Cargo.toml token/psp37/Cargo.toml")]
    //     async fn e2e_multi_contract_caller(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    //         let psp22_hash = client
    //             .upload("psp22", &ink_e2e::alice(), None).await
    //             .expect("uploading 'psp22' failed").code_hash;

    //         let psp37_hash = client
    //             .upload("psp37", &ink_e2e::alice(), None).await
    //             .expect("uploading 'psp37' failed").code_hash;

    //         let constructor = NftCollectionRef::new(
    //             psp22_hash,
    //             psp37_hash,
    //             10000,
    //             Some(vec![5u8]),
    //             Some(vec![5u8]),
    //             9u8
    //         );

    //         let document_platform_call_acc_id = client
    //             .instantiate("DocManagePlatform", &ink_e2e::alice(), constructor, 0, None).await
    //             .expect("instantiate failed").account_id;

    //         //create document
    //         let create_document = build_message::<NftCollectionRef>(
    //             document_platform_call_acc_id.clone()
    //         ).call(|contract| contract.create_document("Document #0".into(), "0x123456789".into()));
    //         let value = client
    //             .call(&ink_e2e::alice(), create_document, 0, None).await
    //             .expect("create document fail")
    //             .return_value();

    //         // assert_eq!(value, );

    //         let get = build_message::<NftCollectionRef>(document_platform_call_acc_id.clone()).call(
    //             |contract| contract.get_nfts_length()
    //         );

    //         let length = client
    //             .call(&ink_e2e::alice(), get, 0, None).await
    //             .expect("get length document fail")
    //             .return_value();
    //         assert_eq!(length, 1);

    //         let get_document_item = build_message::<NftCollectionRef>(
    //             document_platform_call_acc_id.clone()
    //         ).call(|contract| contract.get_document_by_id(0u32));
    //         let value_document_item = client
    //             .call(&ink_e2e::alice(), get_document_item, 0, None).await
    //             .expect("get value document fail")
    //             .return_value();

    //         // assert_eq!(value, value_document_item);
    //         Ok(())
    //     }
    // }
}