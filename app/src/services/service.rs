// service.rs
// necesary crates
use sails_rs::{
    prelude::*,
    gstd::msg,
};

// import the state
use crate::states::*;
use crate::services::service::state::*;
use crate::utils::Utils;

#[derive(Default)]
pub struct Service;

// Impl for seed related function to init the state
impl Service {
    // Related function to init the service state (call only once)
    pub fn seed() {
        State::init_state();
    }
}

#[service]
impl Service {
    // Service constructor
    pub fn new() -> Self {
        Self
    }

    // Service to create a group
    pub fn create_group(&mut self, group_name: String) -> Events {

        let wallet = msg::source();

        let state = State::state_mut();

        let new_group_id = Utils::generate_group_id();

        // Logic to create a group
        state.create_group(new_group_id, group_name, wallet);

        // Change State and return event
        Events::GroupCreated(new_group_id)
    }

    // Service for a user to join a specific group
    pub fn join_group(&mut self, group_id: u32) -> Events {
        let user_id = msg::source();
        // Validation - check if the group exists
        let state = State::state_mut();
        if let Some(group) = state.groups.iter().find(|g| g.id == group_id) {
            if !group.members.contains(&user_id) {
                // Logic to add a member to the group
                state.join_group(group_id, user_id);

                // Return successful event
                return Events::UserJoined(user_id, group_id);
            }
            return Events::Error("User already in group".to_owned());
        }
        Events::Error("Group not found".to_owned())
    }

    // Service to add an expense to the group
    pub fn add_expense(&mut self, group_id: u32, expenseDTO: ExpenseDTO) -> Events {
        let actor_id = msg::source();
        // Validation - check if the group exists
        let expense = Expense::new(
            Utils::generate_group_id(),
            expenseDTO.description, 
            expenseDTO.amount, 
            expenseDTO.currency,
            actor_id
        );

        let state = State::state_mut();

        if state.groups.iter().any(|g| g.id == group_id) {
            // Logic to add an expense
            state.add_expense(group_id, expense.clone());

            // Return successful event
            return Events::ExpenseAdded(group_id, expense.id);
        }
        Events::Error("Group not found".to_owned())
    }

    pub fn add_payment(&mut self, group_id: u32, amount: u32, to: ActorId)->Events{
        let actor_id = msg::source();

        let state = State::state_mut();

        if let Some(group) = state.groups.iter_mut().find(|g| g.id == group_id) {
            group.payments.push(Payment::new(
                Utils::generate_group_id(),
                actor_id,
                to,
                amount,
            ));
            return Events::PaymentAdded(group_id, amount);
        }
        Events::Error("Group not found".to_owned())
        
    }

    pub fn query_group(&self, groupId: u32)->Group{
        State::state_ref()
        .groups
        .iter()
        .find(|x| x.id == groupId) // Correct closure
        .cloned() // Convert &Group to Group
        .expect("Group not found") 
    }

    pub fn query_actor_groups(&self) -> Vec<Group> {
        let actor_id = msg::source(); 

        State::state_ref()
            .groups
            .iter()
            .filter(|group| group.members.contains(&actor_id)) // Filter groups where actor_id is in the members list
            .cloned() // Clone the groups so they can be returned
            .collect() // Collect the results into a Vec<Group>
    }

    // Queried function to get group members by group id
    pub fn query_group_members(&self, group_id: u32) -> Option<Vec<ActorId>> {
        State::state_ref()
            .groups
            .iter()
            .find(|g| g.id == group_id)
            .map(|g| g.members.clone())
    }

    // Queried function to get expenses for a group
    pub fn query_expenses(&self, group_id: u32) -> Option<Vec<Expense>> {
        State::state_ref()
            .groups
            .iter()
            .find(|g| g.id == group_id)
            .map(|g| g.expenses.clone())
    }

    // Returns a struct that will be sent as a response to the user
    pub fn query(&self) -> IoState {
        State::state_ref()
            .to_owned()
            .into()
    }
}

// struct to use as a response to the user
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Events {
    GroupCreated(u32),
    UserJoined(ActorId, u32),
    ExpenseAdded(u32, u32),
    PaymentAdded(u32, u32),
    Error(String),
}

//This code adds services for creating a group, joining a group, and adding an expense, along with query methods to retrieve admins, group members, and expenses. It includes validation checks to ensure logical consistency, such as checking if a group already exists or if a user is already part of a group before joining.  m 