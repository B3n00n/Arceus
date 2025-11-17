pub mod dto;
pub mod entities;
pub mod value_objects;

pub use value_objects::{ClientId, GameId, GameVersionId, VersionStatus};
pub use entities::{Client, ClientGameAssignment, FileInfo, Game, GameManifest, GameVersion};
pub use dto::{
    AuthRequest, AuthResponse, Claims,
    CreateClientRequest, UpdateClientRequest, ClientResponse,
    CreateGameRequest, UpdateGameRequest, GameResponse,
    CreateGameVersionRequest, GameVersionResponse,
    CreateAssignmentRequest, UpdateAssignmentRequest, AssignmentResponse,
};
