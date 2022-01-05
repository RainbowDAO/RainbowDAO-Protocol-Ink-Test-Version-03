#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use ink_lang as ink;

#[ink::contract]
mod dao {
    use alloc::string::String;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct dao {
        /// Stores a single `bool` value on the storage.
        value: bool,
        name: String,
        info: String,
        erc20: AccountId,
        owen: String,
        menberCLearTime: i32,
        addType: i8,
        addErc20: AccountId,
        addErc20Number: i8,
        addNft: String,
        addNftNumber: i8,
        addErc320: String,
        addErc320Number: i8,
        isUnion: bool,
    }

    impl dao {
        #[ink(constructor)]
        pub fn new(
            _name: String,
            _info: String,
            _erc20: AccountId,
            _owen: String,
            _menberCLearTime: i32,
            _addType: i8,
            _addErc20: AccountId,
            _addErc20Number: i8,
            _addNft: String,
            _addNftNumber: i8,
            _addErc320: String,
            _addErc320Number: i8,
            _isUnion: bool,
        ) -> Self {
            Self {
                value: false,
                name: _name,
                info: _info,
                erc20: _erc20,
                owen: _owen,
                menberCLearTime: _menberCLearTime,
                addType: _addType,
                addErc20: _addErc20,
                addErc20Number: _addErc20Number,
                addNft: _addNft,
                addNftNumber: _addNftNumber,
                addErc320: _addErc320,
                addErc320Number: _addErc320Number,
                isUnion: _isUnion,
            }
        }


        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }
        #[ink(message)]
        pub fn setUnion(&mut self, isUnion: bool) {
            self.isUnion = isUnion;
        }
        #[ink(message)]
        pub fn joinDao(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn isUnion(&self) -> bool {
            self.isUnion
        }
        #[ink(message)]
        pub fn getName(&self) -> bool {
            self.name
        }
        #[ink(message)]
        pub fn getInfo(&self) -> bool {
            self.info
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

            let mut dao = dao::new(
                "test Dao".to_string(),
                "test Dao info".to_string(),
                accounts.bob,
                "1".to_string(),
                0,
                1,
                accounts.bob,
                3,
                "2".to_string(),
                4,
                "3".to_string(),
                5,
                false,
            );
            assert_eq!(dao.isUnion(), false);
            dao.setUnion(true);
            assert_eq!(dao.isUnion(), true);
            println!("Dao name :{}", dao.name)
        }
    }
}
