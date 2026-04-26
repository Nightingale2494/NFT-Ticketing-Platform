#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Map, String, Symbol, Vec, symbol_short,
};

// ─── Storage Key Types ────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Event(u64),         // event_id → EventData
    Ticket(u64),        // ticket_id → TicketData
    EventTickets(u64),  // event_id → Vec<ticket_id>
    OwnerTickets(Address), // owner → Vec<ticket_id>
    EventCounter,       // global event id counter
    TicketCounter,      // global ticket id counter
    Admin,              // contract admin
}

// ─── Data Structures ─────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct EventData {
    pub id: u64,
    pub name: String,
    pub date: u64,          // Unix timestamp
    pub description: String,
    pub organizer: Address,
    pub total_supply: u64,
    pub minted_count: u64,
    pub ticket_price: i128,  // in stroops (1 XLM = 10_000_000 stroops)
    pub max_resale_price: i128, // 0 = no resale restriction
    pub is_active: bool,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct TicketData {
    pub id: u64,
    pub event_id: u64,
    pub owner: Address,
    pub original_owner: Address,
    pub purchase_price: i128,
    pub is_for_sale: bool,
    pub resale_price: i128,
    pub metadata_uri: String,   // IPFS or data URI for ticket image
}

// ─── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct NFTTicketContract;

#[contractimpl]
impl NFTTicketContract {

    // ─── Initialization ───────────────────────────────────────────────────────

    /// Initialize the contract with an admin address
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::EventCounter, &0u64);
        env.storage().instance().set(&DataKey::TicketCounter, &0u64);
    }

    // ─── Event Management ─────────────────────────────────────────────────────

    /// Create a new event. Only the caller becomes the organizer.
    pub fn create_event(
        env: Env,
        organizer: Address,
        name: String,
        date: u64,
        description: String,
        total_supply: u64,
        ticket_price: i128,
        max_resale_price: i128,
        metadata_uri: String,
    ) -> u64 {
        // Require organizer signature
        organizer.require_auth();

        // Validate inputs
        if total_supply == 0 {
            panic!("Supply must be greater than 0");
        }
        if ticket_price < 0 {
            panic!("Ticket price cannot be negative");
        }

        let event_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::EventCounter)
            .unwrap_or(0);

        let event = EventData {
            id: event_id,
            name,
            date,
            description,
            organizer: organizer.clone(),
            total_supply,
            minted_count: 0,
            ticket_price,
            max_resale_price,
            is_active: true,
        };

        // Store event
        env.storage().persistent().set(&DataKey::Event(event_id), &event);

        // Initialize empty ticket list for this event
        let empty_tickets: Vec<u64> = Vec::new(&env);
        env.storage().persistent().set(&DataKey::EventTickets(event_id), &empty_tickets);

        // Increment counter
        env.storage().instance().set(&DataKey::EventCounter, &(event_id + 1));

        // Emit event creation log
        env.events().publish(
            (symbol_short!("CREATE"), symbol_short!("EVENT")),
            (event_id, organizer),
        );

        event_id
    }

    /// Get event details by ID
    pub fn get_event(env: Env, event_id: u64) -> EventData {
        env.storage()
            .persistent()
            .get(&DataKey::Event(event_id))
            .expect("Event not found")
    }

    /// Get all events (returns a list of event IDs up to the counter)
    pub fn get_all_event_ids(env: Env) -> Vec<u64> {
        let count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::EventCounter)
            .unwrap_or(0);
        let mut ids = Vec::new(&env);
        for i in 0..count {
            ids.push_back(i);
        }
        ids
    }

    /// Organizer can deactivate their event
    pub fn deactivate_event(env: Env, organizer: Address, event_id: u64) {
        organizer.require_auth();
        let mut event: EventData = env
            .storage()
            .persistent()
            .get(&DataKey::Event(event_id))
            .expect("Event not found");

        if event.organizer != organizer {
            panic!("Only the organizer can deactivate this event");
        }
        event.is_active = false;
        env.storage().persistent().set(&DataKey::Event(event_id), &event);
    }

    // ─── Ticket Minting ───────────────────────────────────────────────────────

    /// Mint a ticket for a buyer. Must be called by the event organizer.
    /// In a production setting, payment would be verified via Stellar operations.
    pub fn mint_ticket(
        env: Env,
        organizer: Address,
        event_id: u64,
        buyer: Address,
        metadata_uri: String,
    ) -> u64 {
        organizer.require_auth();

        let mut event: EventData = env
            .storage()
            .persistent()
            .get(&DataKey::Event(event_id))
            .expect("Event not found");

        // Authorization: only the event organizer can mint
        if event.organizer != organizer {
            panic!("Only the event organizer can mint tickets");
        }

        // Check event is active
        if !event.is_active {
            panic!("Event is not active");
        }

        // Check supply
        if event.minted_count >= event.total_supply {
            panic!("All tickets have been minted");
        }

        let ticket_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::TicketCounter)
            .unwrap_or(0);

        let ticket = TicketData {
            id: ticket_id,
            event_id,
            owner: buyer.clone(),
            original_owner: buyer.clone(),
            purchase_price: event.ticket_price,
            is_for_sale: false,
            resale_price: 0,
            metadata_uri,
        };

        // Persist ticket
        env.storage().persistent().set(&DataKey::Ticket(ticket_id), &ticket);

        // Append to event's ticket list
        let mut event_tickets: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::EventTickets(event_id))
            .unwrap_or(Vec::new(&env));
        event_tickets.push_back(ticket_id);
        env.storage().persistent().set(&DataKey::EventTickets(event_id), &event_tickets);

        // Append to owner's ticket list
        let mut owner_tickets: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::OwnerTickets(buyer.clone()))
            .unwrap_or(Vec::new(&env));
        owner_tickets.push_back(ticket_id);
        env.storage().persistent().set(&DataKey::OwnerTickets(buyer.clone()), &owner_tickets);

        // Update minted count
        event.minted_count += 1;
        env.storage().persistent().set(&DataKey::Event(event_id), &event);

        // Increment ticket counter
        env.storage().instance().set(&DataKey::TicketCounter, &(ticket_id + 1));

        // Emit mint event
        env.events().publish(
            (symbol_short!("MINT"), symbol_short!("TICKET")),
            (ticket_id, event_id, buyer),
        );

        ticket_id
    }

    // ─── Ticket Transfer / Resale ─────────────────────────────────────────────

    /// Transfer a ticket from one address to another.
    /// Enforces max resale price if set on the event.
    pub fn transfer_ticket(
        env: Env,
        from: Address,
        to: Address,
        ticket_id: u64,
        sale_price: i128,
    ) {
        from.require_auth();

        let mut ticket: TicketData = env
            .storage()
            .persistent()
            .get(&DataKey::Ticket(ticket_id))
            .expect("Ticket not found");

        // Verify ownership
        if ticket.owner != from {
            panic!("Caller does not own this ticket");
        }

        // Fetch event to check resale restrictions
        let event: EventData = env
            .storage()
            .persistent()
            .get(&DataKey::Event(ticket.event_id))
            .expect("Event not found");

        // Enforce max resale price (if restriction set and it's a secondary sale)
        if event.max_resale_price > 0 && sale_price > event.max_resale_price {
            panic!("Sale price exceeds maximum allowed resale price");
        }

        // Remove ticket from `from` owner list
        let mut from_tickets: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::OwnerTickets(from.clone()))
            .unwrap_or(Vec::new(&env));

        let mut new_from_tickets: Vec<u64> = Vec::new(&env);
        for i in 0..from_tickets.len() {
            let tid = from_tickets.get(i).unwrap();
            if tid != ticket_id {
                new_from_tickets.push_back(tid);
            }
        }
        env.storage().persistent().set(&DataKey::OwnerTickets(from.clone()), &new_from_tickets);

        // Add ticket to `to` owner list
        let mut to_tickets: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::OwnerTickets(to.clone()))
            .unwrap_or(Vec::new(&env));
        to_tickets.push_back(ticket_id);
        env.storage().persistent().set(&DataKey::OwnerTickets(to.clone()), &to_tickets);

        // Update ticket ownership
        ticket.owner = to.clone();
        ticket.purchase_price = sale_price;
        ticket.is_for_sale = false;
        ticket.resale_price = 0;
        env.storage().persistent().set(&DataKey::Ticket(ticket_id), &ticket);

        // Emit transfer event
        env.events().publish(
            (symbol_short!("TRANSFER"), symbol_short!("TICKET")),
            (ticket_id, from, to),
        );
    }

    /// List a ticket for resale at a specified price
    pub fn list_for_sale(env: Env, owner: Address, ticket_id: u64, resale_price: i128) {
        owner.require_auth();

        let mut ticket: TicketData = env
            .storage()
            .persistent()
            .get(&DataKey::Ticket(ticket_id))
            .expect("Ticket not found");

        if ticket.owner != owner {
            panic!("Caller does not own this ticket");
        }

        // Check resale price restriction
        let event: EventData = env
            .storage()
            .persistent()
            .get(&DataKey::Event(ticket.event_id))
            .expect("Event not found");

        if event.max_resale_price > 0 && resale_price > event.max_resale_price {
            panic!("Resale price exceeds maximum allowed");
        }

        ticket.is_for_sale = true;
        ticket.resale_price = resale_price;
        env.storage().persistent().set(&DataKey::Ticket(ticket_id), &ticket);
    }

    /// Remove a ticket from the resale market
    pub fn delist_from_sale(env: Env, owner: Address, ticket_id: u64) {
        owner.require_auth();

        let mut ticket: TicketData = env
            .storage()
            .persistent()
            .get(&DataKey::Ticket(ticket_id))
            .expect("Ticket not found");

        if ticket.owner != owner {
            panic!("Caller does not own this ticket");
        }

        ticket.is_for_sale = false;
        ticket.resale_price = 0;
        env.storage().persistent().set(&DataKey::Ticket(ticket_id), &ticket);
    }

    // ─── Queries ──────────────────────────────────────────────────────────────

    /// Get all ticket IDs owned by an address
    pub fn get_tickets(env: Env, owner: Address) -> Vec<u64> {
        env.storage()
            .persistent()
            .get(&DataKey::OwnerTickets(owner))
            .unwrap_or(Vec::new(&env))
    }

    /// Get a specific ticket's data
    pub fn get_ticket(env: Env, ticket_id: u64) -> TicketData {
        env.storage()
            .persistent()
            .get(&DataKey::Ticket(ticket_id))
            .expect("Ticket not found")
    }

    /// Get all ticket IDs for a specific event
    pub fn get_event_tickets(env: Env, event_id: u64) -> Vec<u64> {
        env.storage()
            .persistent()
            .get(&DataKey::EventTickets(event_id))
            .unwrap_or(Vec::new(&env))
    }

    // ─── Verification ─────────────────────────────────────────────────────────

    /// Verify that an address owns at least one ticket for a given event.
    /// Returns the ticket ID if found, panics otherwise.
    pub fn verify_ticket(env: Env, owner: Address, event_id: u64) -> bool {
        let owner_tickets: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::OwnerTickets(owner.clone()))
            .unwrap_or(Vec::new(&env));

        for i in 0..owner_tickets.len() {
            let ticket_id = owner_tickets.get(i).unwrap();
            let ticket: TicketData = env
                .storage()
                .persistent()
                .get(&DataKey::Ticket(ticket_id))
                .expect("Ticket data corrupted");

            if ticket.event_id == event_id && ticket.owner == owner {
                return true;
            }
        }
        false
    }

    /// Get total number of events created
    pub fn get_event_count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::EventCounter).unwrap_or(0)
    }

    /// Get total number of tickets minted globally
    pub fn get_ticket_count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::TicketCounter).unwrap_or(0)
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_create_event_and_mint_ticket() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, NFTTicketContract);
        let client = NFTTicketContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let organizer = Address::generate(&env);
        let buyer = Address::generate(&env);

        client.initialize(&admin);

        let event_id = client.create_event(
            &organizer,
            &String::from_str(&env, "Stellar Summit 2025"),
            &1750000000u64,
            &String::from_str(&env, "Annual Stellar developer conference"),
            &100u64,
            &10_000_000i128,  // 1 XLM
            &20_000_000i128,  // 2 XLM max resale
            &String::from_str(&env, "ipfs://QmExample"),
        );

        assert_eq!(event_id, 0);

        let ticket_id = client.mint_ticket(
            &organizer,
            &event_id,
            &buyer,
            &String::from_str(&env, "ipfs://QmTicket1"),
        );

        assert_eq!(ticket_id, 0);

        let ticket = client.get_ticket(&ticket_id);
        assert_eq!(ticket.owner, buyer);
        assert_eq!(ticket.event_id, 0);

        // Verify ownership
        assert!(client.verify_ticket(&buyer, &event_id));

        let tickets = client.get_tickets(&buyer);
        assert_eq!(tickets.len(), 1);
    }

    #[test]
    fn test_transfer_ticket() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, NFTTicketContract);
        let client = NFTTicketContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let organizer = Address::generate(&env);
        let buyer = Address::generate(&env);
        let new_owner = Address::generate(&env);

        client.initialize(&admin);

        let event_id = client.create_event(
            &organizer,
            &String::from_str(&env, "Test Event"),
            &1750000000u64,
            &String::from_str(&env, "Test"),
            &10u64,
            &10_000_000i128,
            &15_000_000i128,
            &String::from_str(&env, "ipfs://test"),
        );

        let ticket_id = client.mint_ticket(
            &organizer,
            &event_id,
            &buyer,
            &String::from_str(&env, "ipfs://ticket"),
        );

        // Transfer within max resale price
        client.transfer_ticket(&buyer, &new_owner, &ticket_id, &12_000_000i128);

        let ticket = client.get_ticket(&ticket_id);
        assert_eq!(ticket.owner, new_owner);

        // Verify new ownership
        assert!(client.verify_ticket(&new_owner, &event_id));
        assert!(!client.verify_ticket(&buyer, &event_id));
    }

    #[test]
    #[should_panic(expected = "All tickets have been minted")]
    fn test_supply_exceeded() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, NFTTicketContract);
        let client = NFTTicketContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let organizer = Address::generate(&env);
        let buyer = Address::generate(&env);

        client.initialize(&admin);

        let event_id = client.create_event(
            &organizer,
            &String::from_str(&env, "Tiny Event"),
            &1750000000u64,
            &String::from_str(&env, "Only 1 ticket"),
            &1u64, // supply of 1
            &10_000_000i128,
            &0i128,
            &String::from_str(&env, "ipfs://tiny"),
        );

        client.mint_ticket(&organizer, &event_id, &buyer, &String::from_str(&env, "ipfs://t1"));
        // This should panic
        client.mint_ticket(&organizer, &event_id, &buyer, &String::from_str(&env, "ipfs://t2"));
    }
}