#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(clippy::arithmetic_side_effects)]

pub use self::token::TokenRef;

mod data;
mod errors;
mod testing;
mod traits;

pub use data::{PSP22Data, PSP22Event};
pub use errors::PSP22Error;
pub use traits::{PSP22Burnable, PSP22Metadata, PSP22Mintable, PSP22};

#[ink::contract]
mod token {
    use crate::{PSP22Data, PSP22Error, PSP22Event, PSP22Metadata, PSP22};
    use ink::prelude::{string::String, vec::Vec};
    use crate::PSP22Error::Custom;

    #[ink(storage)]
    pub struct Token {
        data: PSP22Data,
        name: Option<String>,
        symbol: Option<String>,
        bond: AccountId,
        decimals: u8,
    }

    impl Token {
        #[ink(constructor)]
        pub fn new(
            name: Option<String>,
            symbol: Option<String>,
            bond: AccountId,
        ) -> Self {
            Self {
                data: PSP22Data::default(),
                name,
                symbol,
                bond,
                decimals: 12,
            }
        }

        fn emit_events(&self, events: Vec<PSP22Event>) {
            for event in events {
                match event {
                    PSP22Event::Transfer { from, to, value } => {
                        self.env().emit_event(Transfer { from, to, value })
                    }
                    PSP22Event::Approval {
                        owner,
                        spender,
                        amount,
                    } => self.env().emit_event(Approval {
                        owner,
                        spender,
                        amount,
                    }),
                }
            }
        }

        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, value: u128) -> Result<(), PSP22Error> {
            self.only_bond()?;
            self.data.mint(to, value)?;
            Ok(())
        }

        #[ink(message)]
        pub fn burn(&mut self, from: AccountId, value: u128) -> Result<(), PSP22Error> {
            self.only_bond()?;
            self.data.burn(from, value)?;
            Ok(())
        }


        fn only_bond(&self) -> Result<(), PSP22Error> {
            if self.env().caller() != self.bond {
                return Err(Custom("Not Permission".parse().unwrap()));
            }
            Ok(())
        }
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        amount: u128,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: u128,
    }

    impl PSP22 for Token {
        #[ink(message)]
        fn total_supply(&self) -> u128 {
            self.data.total_supply()
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u128 {
            self.data.balance_of(owner)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
            self.data.allowance(owner, spender)
        }

        #[ink(message)]
        fn transfer(
            &mut self,
            to: AccountId,
            value: u128,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let events = self.data.transfer(self.env().caller(), to, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: u128,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .transfer_from(self.env().caller(), from, to, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: u128) -> Result<(), PSP22Error> {
            let events = self.data.approve(self.env().caller(), spender, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn increase_allowance(
            &mut self,
            spender: AccountId,
            delta_value: u128,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .increase_allowance(self.env().caller(), spender, delta_value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn decrease_allowance(
            &mut self,
            spender: AccountId,
            delta_value: u128,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .decrease_allowance(self.env().caller(), spender, delta_value)?;
            self.emit_events(events);
            Ok(())
        }
    }

    impl PSP22Metadata for Token {
        #[ink(message)]
        fn token_name(&self) -> Option<String> {
            self.name.clone()
        }
        #[ink(message)]
        fn token_symbol(&self) -> Option<String> {
            self.symbol.clone()
        }
        #[ink(message)]
        fn token_decimals(&self) -> u8 {
            self.decimals
        }
    }

    // // (7)
    // #[cfg(test)]
    // mod tests {
    //     crate::tests!(Token, (|supply| Token::new(None, None, 0)));
    // }
}
