#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod baseDao {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct BaseDao {
       member:mapping,
       father:address,
       memberList:array,
        departimentSize:u8,
        departimentList:array,
        motions:array,
        montionsize:u8,

    }

    impl BaseDao {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(
            _father:address,
            _member:mapping,
                   _memberList:array,
                   _departimentSize:u8,
                   _departimentList:array,
                   _motions:array,
                   _montionsize:u8, ) -> Self {
            Self {
                father:_father
                member:_member:_,
                memberList:_memberList:_,
                departimentSize:_departimentSize:_,
                departimentList:_departimentList:_,
                motions:_motions:_,
                montionsize:_montionsize:_,
            }
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn getFather(&mut self) {
            self.father
        }
        #[ink(message)]
        pub fn setFather(&mut self,_father:address) {
            self.father=_father;
        }
        #[ink(message)]
        pub fn createDepartment(&mut self,_father:address) {
            self.father=_father;
        }
        #[ink(message)]
        pub fn createMotion(&mut self,_father:address) {
            self.father=_father;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
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

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {

        }


    }
}
