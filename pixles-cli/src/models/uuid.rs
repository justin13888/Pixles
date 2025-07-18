use uuid::{Timestamp, Uuid};
pub fn generate_uuid() -> Uuid {
    Uuid::new_v7(Timestamp::now(uuid::timestamp::context::NoContext))
}
