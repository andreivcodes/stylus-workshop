#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, U256},
    msg,
    prelude::*,
};

sol_storage! {
    #[entrypoint]
    pub struct VisitorBook {
        // Dynamic array to store visitor addresses
        address[] visitors;
        // Mapping to track if an address has already visited
        mapping(address => bool) has_visited;
        // Counter for total unique visitors
        uint256 total_visitors;
    }
}

#[public]
impl VisitorBook {
    // Function to record a new visitor
    pub fn sign_guestbook(&mut self) {
        let visitor = msg::sender();

        // Check if the address has already visited
        if !self.has_visited.get(visitor) {
            // Add to visitors array
            self.visitors.push(visitor);
            // Mark as visited
            self.has_visited.setter(visitor).set(true);
            // Increment total visitors
            let current_count = self.total_visitors.get();
            self.total_visitors.set(current_count + U256::from(1));
        }
    }

    // Get total number of unique visitors
    pub fn get_total_visitors(&self) -> U256 {
        self.total_visitors.get()
    }

    // Get visitor at specific index
    pub fn get_visitor_at_index(&self, index: U256) -> Address {
        self.visitors.get(index).unwrap()
    }

    // Check if an address has visited
    pub fn has_address_visited(&self, address: Address) -> bool {
        self.has_visited.get(address)
    }

    // Get all visitors (returns array length)
    pub fn get_all_visitors_length(&self) -> U256 {
        U256::from(self.visitors.len())
    }
}
