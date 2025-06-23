use async_trait::async_trait;

#[async_trait]
pub trait AudioPlayer {
    async fn play_sound(&self, name: &str);
}

