// State.rs
// Use necessary crates
use sails_rs::{
    prelude::*,
};

// Static mut variable (contract's state)
pub static mut STATE: Option<State> = None;

// Create a struct for the state
#[derive(Clone, Default)]
pub struct State {
    pub groups: Vec<Group>,
}

// Struct to represent a group
#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Group {
    pub id: u32,
    pub members: Vec<ActorId>,
    pub expenses: Vec<Expense>,
}

#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct GroupDTO {
    pub members: Vec<u32>,
    pub expenses: Vec<ExpenseDTO>,
}

// Struct to represent an expense
#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Expense {
    pub id: u32,
    pub description: String,
    pub amount: u128,
    pub currency: String,
    pub actor_id: ActorId
}

impl Expense {
    pub fn new(id: u32, description: String, amount: u128, currency: String, actor_id:ActorId) -> Self {
        Expense {
            id,
            description,
            amount,
            currency,
            actor_id
        }
    }
}

#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct ExpenseDTO {
    pub description: String,
    pub amount: u128,
    pub currency: String,
}

// Impl to set methods or related functions
impl State {
    // Method to create a new instance
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    // Related function to init the state
    pub fn init_state() {
        unsafe {
            STATE = Some(Self::new());
        }
    }

    // Related function to get the state as mutable
    pub fn state_mut() -> &'static mut State {
        let state = unsafe { STATE.as_mut() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }

    // Related function to get the state as reference
    pub fn state_ref() -> &'static State {
        let state = unsafe { STATE.as_ref() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }

    // Service to create a group 
    pub fn create_group(&mut self, _group_id: u32, wallet: ActorId) {
        self.groups.push(Group {
            id: _group_id,
            members: vec![wallet],
            ..Default::default()
        });
    }

    // Service for a user to join a specific group
    pub fn join_group(&mut self, _group_id: u32, _user_id: ActorId) {
        if let Some(group) = self.groups.iter_mut().find(|g| g.id == _group_id) {
            group.members.push(_user_id);
        }
    }

    // Service to add an expense to the group
    pub fn add_expense(&mut self, _group_id: u32, _expense: Expense) {
        if let Some(group) = self.groups.iter_mut().find(|g| g.id == _group_id) {
            group.expenses.push(_expense);
        }
    }

    // Service to edit an existing expense
    pub fn edit_expense(&mut self, _group_id: u32, _expense_id: u32, _new_expense: Expense) {
        if let Some(group) = self.groups.iter_mut().find(|g| g.id == _group_id) {
            if let Some(expense) = group.expenses.iter_mut().find(|e| e.id == _expense_id) {
                *expense = _new_expense;
            }
        }
    }

    // Service to delete an existing expense
    pub fn delete_expense(&mut self, _group_id: u32, _expense_id: u32) {
        if let Some(group) = self.groups.iter_mut().find(|g| g.id == _group_id) {
            group.expenses.retain(|e| e.id != _expense_id);
        }
    }
}

// Create a struct that can be sent to the user who reads state
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoState {
    pub groups: Vec<Group>,
}

impl From<State> for IoState {
    fn from(value: State) -> Self {
        let State { groups } = value;
        Self { groups }
    }
}