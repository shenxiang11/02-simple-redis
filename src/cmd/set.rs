use crate::cmd::{CommandError, CommandExecutor, extract_args, SAdd};
use crate::{RespArray, RespFrame, SimpleString};

impl CommandExecutor for SAdd {
    fn execute(self, backend: &crate::Backend) -> RespFrame {
        let count = backend.sadd(self.key, self.members);
        SimpleString::new(count.to_string()).into()
    }
}

impl TryFrom<RespArray> for SAdd {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        let mut args = extract_args(value, 1)?.into_iter();

        let key = match args.next() {
            Some(RespFrame::BulkString(key)) => String::from_utf8(key.0)?,
            _ => return Err(CommandError::InvalidArgument("Invalid key".to_string())),
        };

        let members = args.map(|frame| {
            match frame {
                RespFrame::BulkString(member) => Ok(String::from_utf8(member.0)?),
                _ => Err(CommandError::InvalidArgument("Invalid member".to_string())),
            }
        }).collect::<Result<Vec<String>, CommandError>>()?;

        Ok(SAdd {
            key,
            members,
        })
    }
}

