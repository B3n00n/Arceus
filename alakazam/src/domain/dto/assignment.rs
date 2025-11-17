use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{ClientId, GameId, GameVersionId};
use crate::domain::entities::ClientGameAssignment;

/// Request to create a game assignment
#[derive(Debug, Clone, Deserialize)]
pub struct CreateAssignmentRequest {
    pub client_id: ClientId,
    pub game_id: GameId,
    pub target_version_id: Option<GameVersionId>,
}

/// Request to update an assignment
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAssignmentRequest {
    pub target_version_id: Option<GameVersionId>,
}

/// Assignment response DTO
#[derive(Debug, Clone, Serialize)]
pub struct AssignmentResponse {
    pub client_id: ClientId,
    pub game_id: GameId,
    pub target_version_id: Option<GameVersionId>,
    pub created_at: DateTime<Utc>,
}

impl From<ClientGameAssignment> for AssignmentResponse {
    fn from(assignment: ClientGameAssignment) -> Self {
        Self {
            client_id: assignment.client_id,
            game_id: assignment.game_id,
            target_version_id: assignment.target_version_id,
            created_at: assignment.created_at,
        }
    }
}
