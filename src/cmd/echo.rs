use crate::{Backend, BulkString, RespArray, RespFrame};
use crate::cmd::{CommandError, CommandExecutor, Echo, extract_args, validate_command};

impl CommandExecutor for Echo {
    fn execute(self, _backend: &Backend) -> RespFrame {
        RespFrame::BulkString(BulkString(self.message.as_bytes().to_vec()))
    }
}

impl TryFrom<RespArray> for Echo {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["echo"], 1)?;

        let mut  args = extract_args(value, 1)?.into_iter();

        let message = match args.next() {
            Some(RespFrame::BulkString(message)) => String::from_utf8(message.0)?,
            _ => return Err(CommandError::InvalidArgument("Invalid message".to_string())),
        };

        Ok(Echo {
            message: message.to_string(),
        })
    }
}
