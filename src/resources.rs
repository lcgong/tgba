
use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "resources/"]
pub struct EmbededResources;


#[derive(RustEmbed)]
#[folder = "requirements/"]
pub struct EmbededRequirements;

