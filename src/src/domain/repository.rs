use async_trait::async_trait;
use crate::error::Result;
use crate::domain::event::Event;

#[async_trait]
pub trait EventRepository: Send + Sync + 'static {
    async fn save(&self, event: &Event) -> Result<()>;
    async fn find_by_id(&self, id: &str) -> Result<Vec<Event>>;
} 