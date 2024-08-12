use serde::{Deserialize, Serialize};

use crate::Text;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "action", content = "contents", rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum HoverEvent {
    /// Displays a tooltip with the given text.
    ShowText(Text),
    /// Shows an item.
    ShowItem {
        /// Resource identifier of the item
        id: String,
        /// Number of the items in the stack
        count: Option<i32>,
        /// NBT information about the item (sNBT format)
        tag: String,
    },
    /// Shows an entity.
    ShowEntity {
        /// The entity's UUID
        id: uuid::Uuid,
        /// Resource identifier of the entity
        #[serde(rename = "type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        kind: Option<String>,
        /// Optional custom name for the entity
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<Text>,
    },
}
