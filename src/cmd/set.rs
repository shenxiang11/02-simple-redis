use crate::cmd::{CommandError, CommandExecutor, extract_args, SAdd, SIsMember, SMembers, validate_command};
use crate::{Backend, RespArray, RespFrame, SimpleString};

impl CommandExecutor for SAdd {
    fn execute(self, backend: &Backend) -> RespFrame {
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

impl CommandExecutor for SIsMember {
    fn execute(self, backend: &Backend) -> RespFrame {
        let res = backend.sismember(self.key, self.member);

        if res {
            SimpleString::new("1").into()
        } else {
            SimpleString::new("0").into()
        }
    }
}

impl TryFrom<RespArray> for SIsMember{
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["sismember"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(member)) ) => Ok(SIsMember {
                key: String::from_utf8(key.0)?,
                member: String::from_utf8(member.0)?,
            }),
            _ => Err(CommandError::InvalidArgument("Invalid key or member".to_string())),
        }
    }
}

impl CommandExecutor for SMembers {
    fn execute(self, backend: &Backend) -> RespFrame {
        let members = backend.smembers(self.key);

        let mut data = Vec::new();

        for member in members {
            data.push(RespFrame::BulkString(member.into()));
        }

        RespArray::new(data).into()
    }
}

impl TryFrom<RespArray> for SMembers {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["smembers"], 1)?;

        let mut args = extract_args(value, 1)?.into_iter();

        let key = match args.next() {
            Some(RespFrame::BulkString(key)) => String::from_utf8(key.0)?,
            _ => return Err(CommandError::InvalidArgument("Invalid key".to_string())),
        };

        Ok(SMembers {
            key,
        })
    }
}

