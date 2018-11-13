#![no_std]

extern crate tiny_keccak;
use tiny_keccak::Keccak;

static TOTAL_SUPPLY_KEY: H256 = H256([2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
static OWNER_KEY: H256 = H256([3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);

// Reads balance by address
fn read_balance_of(owner: &Address) -> U256 {
    owasm_ethereum::read(&balance_key(owner)).into()
}

// Reads allowance value using key
// Key generated by allowance_key function
fn read_allowance(key: &H256) -> U256 {
    owasm_ethereum::read(key).into()
}

// Writes allowance value
// Key generated by allowance_key function
fn write_allowance(key: &H256, value: U256) {
    owasm_ethereum::write(key, &value.into())
}

// Generates the "allowance" storage key to map owner and spender
fn allowance_key(owner: &Address, spender: &Address) -> H256 {
    let mut keccak = Keccak::new_keccak256();
    let mut res = H256::new();
    keccak.update("allowance_key".as_ref());
    keccak.update(owner.as_ref());
    keccak.update(spender.as_ref());
    keccak.finalize(&mut res);
    res
}

// Generates a balance key for some address.
// Used to map balances with their owners.
fn balance_key(address: &Address) -> H256 {
    let mut key = H256::from(address);
    key[0] = 1; // just a naiive "namespace";
    key
}

#[owasm_abi_derive::contract]
trait TokenContract {
    fn constructor(&mut self, total_supply: U256) {
        let sender = owasm_ethereum::sender();
        // Set up the total supply for the token
        owasm_ethereum::write(&TOTAL_SUPPLY_KEY, &total_supply.into());
        // Give all tokens to the contract owner
        owasm_ethereum::write(&balance_key(&sender), &total_supply.into());
        // Set the contract owner
        owasm_ethereum::write(&OWNER_KEY, &H256::from(sender).into());
    }

    #[constant]
    fn balanceOf(&mut self, owner: Address) -> U256 {
        read_balance_of(&owner)
    }

    #[constant]
    fn totalSupply(&mut self) -> U256 {
        owasm_ethereum::read(&TOTAL_SUPPLY_KEY).into()
    }

    fn transfer(&mut self, to: Address, amount: U256) -> bool {
        let sender = owasm_ethereum::sender();
        let senderBalance = read_balance_of(&sender);
        let recipientBalance = read_balance_of(&to);
        if amount == 0.into() || senderBalance < amount || to == sender {
            false
        } else {
            let new_sender_balance = senderBalance - amount;
            let new_recipient_balance = recipientBalance + amount;
            // TODO: impl From<U256> for H256 makes convertion to big endian. Could be optimized
            owasm_ethereum::write(&balance_key(&sender), &new_sender_balance.into());
            owasm_ethereum::write(&balance_key(&to), &new_recipient_balance.into());
            true
        }
    }

    fn approve(&mut self, spender: Address, value: U256) -> bool {
        write_allowance(&allowance_key(&owasm_ethereum::sender(), &spender), value);
        true
    }

    fn allowance(&mut self, owner: Address, spender: Address) -> U256 {
        read_allowance(&allowance_key(&owner, &spender))
    }

    fn transferFrom(&mut self, from: Address, to: Address, amount: U256) -> bool {
        let fromBalance = read_balance_of(&from);
        let recipientBalance = read_balance_of(&to);
        let a_key = allowance_key(&from, &owasm_ethereum::sender());
        let allowed = read_allowance(&a_key);
        if  allowed < amount || amount == 0.into() || fromBalance < amount  || to == from {
            false
        } else {
            let new_allowed = allowed - amount;
            let new_from_balance = fromBalance - amount;
            let new_recipient_balance = recipientBalance + amount;
            owasm_ethereum::write(&a_key, &new_allowed.into());
            owasm_ethereum::write(&balance_key(&from), &new_from_balance.into());
            owasm_ethereum::write(&balance_key(&to), &new_recipient_balance.into());
            true
        }
    }
}
