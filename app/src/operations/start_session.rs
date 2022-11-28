use crate::state::ResourceStorage;
use crate::State;
use aws_smithy_http_server::Extension;
use liwimean_ims_core_server_sdk::error::{InvalidTransactionDirectory, StartSessionError};
use liwimean_ims_core_server_sdk::input::StartSessionInput;
use liwimean_ims_core_server_sdk::output::StartSessionOutput;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use uuid::Uuid;

pub async fn do_start_session(
    input: StartSessionInput,
    state: Extension<Arc<State>>,
) -> Result<StartSessionOutput, StartSessionError> {
    match get_transaction_path(input.transactions_directory_path()) {
        Some(value) => {
            let session_id = Uuid::new_v4().to_string();
            state.resources_map.insert(
                input.clone().tlr().to_string(),
                ResourceStorage {
                    transaction_path: value,
                },
            );
            Ok(StartSessionOutput { session_id })
        }
        None => Err(StartSessionError::InvalidTransactionDirectory(
            InvalidTransactionDirectory {},
        )),
    }
}

fn get_transaction_path(path: &str) -> Option<PathBuf> {
    let path = Path::new(path);

    if path.is_dir() {
        Option::Some(path.to_path_buf())
    } else {
        Option::None
    }
}
