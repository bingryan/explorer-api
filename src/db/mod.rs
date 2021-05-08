use actix::prelude::{Actor, SyncContext};

pub type MeiliPool = String;

pub struct DocumentExecutor(pub MeiliPool);

impl Actor for DocumentExecutor {
    type Context = SyncContext<Self>;
}