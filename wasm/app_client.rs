// Code generated by sails-client-gen. DO NOT EDIT.
#[allow(unused_imports)]
use sails_rs::collections::BTreeMap;
#[allow(unused_imports)]
use sails_rs::{
    calls::{Activation, Call, Query, Remoting, RemotingAction},
    prelude::*,
    String,
};
pub struct AppFactory<R> {
    #[allow(dead_code)]
    remoting: R,
}
impl<R> AppFactory<R> {
    #[allow(unused)]
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::AppFactory for AppFactory<R> {
    type Args = R::Args;
    fn new(&self) -> impl Activation<Args = R::Args> {
        RemotingAction::<_, app_factory::io::New>::new(self.remoting.clone(), ())
    }
}

pub mod app_factory {
    use super::*;
    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct New(());
        impl New {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <New as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for New {
            const ROUTE: &'static [u8] = &[12, 78, 101, 119];
            type Params = ();
            type Reply = ();
        }
    }
}
pub struct Service<R> {
    remoting: R,
}
impl<R> Service<R> {
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::Service for Service<R> {
    type Args = R::Args;
    fn add_expense(
        &mut self,
        group_id: u32,
        expense: Expense,
    ) -> impl Call<Output = Events, Args = R::Args> {
        RemotingAction::<_, service::io::AddExpense>::new(
            self.remoting.clone(),
            (group_id, expense),
        )
    }
    fn create_group(&mut self, group_id: u32) -> impl Call<Output = Events, Args = R::Args> {
        RemotingAction::<_, service::io::CreateGroup>::new(self.remoting.clone(), group_id)
    }
    fn join_group(
        &mut self,
        group_id: u32,
        user_id: ActorId,
    ) -> impl Call<Output = Events, Args = R::Args> {
        RemotingAction::<_, service::io::JoinGroup>::new(self.remoting.clone(), (group_id, user_id))
    }
    fn query(&self) -> impl Query<Output = IoState, Args = R::Args> {
        RemotingAction::<_, service::io::Query>::new(self.remoting.clone(), ())
    }
    fn query_admins(&self) -> impl Query<Output = Vec<ActorId>, Args = R::Args> {
        RemotingAction::<_, service::io::QueryAdmins>::new(self.remoting.clone(), ())
    }
    fn query_expenses(
        &self,
        group_id: u32,
    ) -> impl Query<Output = Option<Vec<Expense>>, Args = R::Args> {
        RemotingAction::<_, service::io::QueryExpenses>::new(self.remoting.clone(), group_id)
    }
    fn query_group_members(
        &self,
        group_id: u32,
    ) -> impl Query<Output = Option<Vec<ActorId>>, Args = R::Args> {
        RemotingAction::<_, service::io::QueryGroupMembers>::new(self.remoting.clone(), group_id)
    }
}

pub mod service {
    use super::*;

    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct AddExpense(());
        impl AddExpense {
            #[allow(dead_code)]
            pub fn encode_call(group_id: u32, expense: super::Expense) -> Vec<u8> {
                <AddExpense as ActionIo>::encode_call(&(group_id, expense))
            }
        }
        impl ActionIo for AddExpense {
            const ROUTE: &'static [u8] = &[
                28, 83, 101, 114, 118, 105, 99, 101, 40, 65, 100, 100, 69, 120, 112, 101, 110, 115,
                101,
            ];
            type Params = (u32, super::Expense);
            type Reply = super::Events;
        }
        pub struct CreateGroup(());
        impl CreateGroup {
            #[allow(dead_code)]
            pub fn encode_call(group_id: u32) -> Vec<u8> {
                <CreateGroup as ActionIo>::encode_call(&group_id)
            }
        }
        impl ActionIo for CreateGroup {
            const ROUTE: &'static [u8] = &[
                28, 83, 101, 114, 118, 105, 99, 101, 44, 67, 114, 101, 97, 116, 101, 71, 114, 111,
                117, 112,
            ];
            type Params = u32;
            type Reply = super::Events;
        }
        pub struct JoinGroup(());
        impl JoinGroup {
            #[allow(dead_code)]
            pub fn encode_call(group_id: u32, user_id: ActorId) -> Vec<u8> {
                <JoinGroup as ActionIo>::encode_call(&(group_id, user_id))
            }
        }
        impl ActionIo for JoinGroup {
            const ROUTE: &'static [u8] = &[
                28, 83, 101, 114, 118, 105, 99, 101, 36, 74, 111, 105, 110, 71, 114, 111, 117, 112,
            ];
            type Params = (u32, ActorId);
            type Reply = super::Events;
        }
        pub struct Query(());
        impl Query {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <Query as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for Query {
            const ROUTE: &'static [u8] = &[
                28, 83, 101, 114, 118, 105, 99, 101, 20, 81, 117, 101, 114, 121,
            ];
            type Params = ();
            type Reply = super::IoState;
        }
        pub struct QueryAdmins(());
        impl QueryAdmins {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <QueryAdmins as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for QueryAdmins {
            const ROUTE: &'static [u8] = &[
                28, 83, 101, 114, 118, 105, 99, 101, 44, 81, 117, 101, 114, 121, 65, 100, 109, 105,
                110, 115,
            ];
            type Params = ();
            type Reply = Vec<ActorId>;
        }
        pub struct QueryExpenses(());
        impl QueryExpenses {
            #[allow(dead_code)]
            pub fn encode_call(group_id: u32) -> Vec<u8> {
                <QueryExpenses as ActionIo>::encode_call(&group_id)
            }
        }
        impl ActionIo for QueryExpenses {
            const ROUTE: &'static [u8] = &[
                28, 83, 101, 114, 118, 105, 99, 101, 52, 81, 117, 101, 114, 121, 69, 120, 112, 101,
                110, 115, 101, 115,
            ];
            type Params = u32;
            type Reply = Option<Vec<super::Expense>>;
        }
        pub struct QueryGroupMembers(());
        impl QueryGroupMembers {
            #[allow(dead_code)]
            pub fn encode_call(group_id: u32) -> Vec<u8> {
                <QueryGroupMembers as ActionIo>::encode_call(&group_id)
            }
        }
        impl ActionIo for QueryGroupMembers {
            const ROUTE: &'static [u8] = &[
                28, 83, 101, 114, 118, 105, 99, 101, 68, 81, 117, 101, 114, 121, 71, 114, 111, 117,
                112, 77, 101, 109, 98, 101, 114, 115,
            ];
            type Params = u32;
            type Reply = Option<Vec<ActorId>>;
        }
    }
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Expense {
    pub id: u32,
    pub description: String,
    pub amount: u128,
    pub currency: String,
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Events {
    GroupCreated(u32),
    UserJoined((ActorId, u32)),
    ExpenseAdded((u32, u32)),
    Error(String),
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoState {
    pub admins: Vec<ActorId>,
    pub groups: Vec<Group>,
}
#[derive(PartialEq, Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Group {
    pub id: u32,
    pub members: Vec<ActorId>,
    pub expenses: Vec<Expense>,
}

pub mod traits {
    use super::*;
    #[allow(dead_code)]
    pub trait AppFactory {
        type Args;
        #[allow(clippy::new_ret_no_self)]
        #[allow(clippy::wrong_self_convention)]
        fn new(&self) -> impl Activation<Args = Self::Args>;
    }

    #[allow(clippy::type_complexity)]
    pub trait Service {
        type Args;
        fn add_expense(
            &mut self,
            group_id: u32,
            expense: Expense,
        ) -> impl Call<Output = Events, Args = Self::Args>;
        fn create_group(&mut self, group_id: u32) -> impl Call<Output = Events, Args = Self::Args>;
        fn join_group(
            &mut self,
            group_id: u32,
            user_id: ActorId,
        ) -> impl Call<Output = Events, Args = Self::Args>;
        fn query(&self) -> impl Query<Output = IoState, Args = Self::Args>;
        fn query_admins(&self) -> impl Query<Output = Vec<ActorId>, Args = Self::Args>;
        fn query_expenses(
            &self,
            group_id: u32,
        ) -> impl Query<Output = Option<Vec<Expense>>, Args = Self::Args>;
        fn query_group_members(
            &self,
            group_id: u32,
        ) -> impl Query<Output = Option<Vec<ActorId>>, Args = Self::Args>;
    }
}
