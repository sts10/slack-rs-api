pub const ID_LENGTH: usize = 9;

macro_rules! make_id {
    ($name:ident, $firstchar:expr, $visname:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name {
            len: u8,
            buf: [u8; ID_LENGTH],
        }

        impl $name {
            pub fn as_str(&self) -> &str {
                ::std::str::from_utf8(&self.buf[..self.len as usize]).unwrap()
            }
        }

        impl<'a> From<&'a str> for $name {
            #[inline]
            fn from(input: &'a str) -> Self {
                assert!(input.len() <= ID_LENGTH);
                assert!(input.as_bytes()[0] == $firstchar);
                let mut output = Self {
                    len: input.len() as u8,
                    buf: [0; ID_LENGTH],
                };
                output.buf[..input.len()].copy_from_slice(&input.as_bytes());
                output
            }
        }

        struct $visname;

        impl<'de> ::serde::de::Visitor<'de> for $visname {
            type Value = $name;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str(&format!("a {}-byte str", ID_LENGTH))
            }

            fn visit_str<E>(self, value: &str) -> Result<$name, E>
            where
                E: ::serde::de::Error,
            {
                if value.len() <= ID_LENGTH && value.len() > 0 && value.as_bytes()[0] == $firstchar {
                    Ok($name::from(value))
                } else {
                    Err(E::custom(format!(
                        "{} must be a {}-byte string starting with {}, found {:?}",
                        stringify!($name),
                        ID_LENGTH,
                        $firstchar as char,
                        value,
                    )))
                }
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                deserializer.deserialize_str($visname)
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(::std::str::from_utf8(&self.buf[..self.len as usize]).unwrap())
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(
                    f,
                    "{}",
                    ::std::str::from_utf8(&self.buf[..self.len as usize]).unwrap()
                ).map(|_| ())
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self)
            }
        }
    };
}

make_id!(BotId, b'B', BotIdVisitor);
make_id!(UserId, b'U', UserIdVisitor);
make_id!(ChannelId, b'C', ChannelIdVisitor);
make_id!(GroupId, b'G', GroupIdVisitor);
make_id!(DmId, b'D', DmIdVisitor);
make_id!(TeamId, b'T', TeamIdVisitor);
make_id!(AppId, b'A', AppIdVisitor);
make_id!(FileId, b'F', FileIdVisitor);
make_id!(UsergroupId, b'S', UsergroupIdVisitor);
make_id!(ReminderId, b'R', ReminderIdVisitor);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ConversationId {
    Channel(ChannelId),
    Group(GroupId),
    DirectMessage(DmId),
}

impl ConversationId {
    pub fn as_str(&self) -> &str {
        match &self {
            ConversationId::Channel(id) => id.as_str(),
            ConversationId::Group(id) => id.as_str(),
            ConversationId::DirectMessage(id) => id.as_str(),
        }
    }
}

impl ::std::fmt::Display for ConversationId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match &self {
            ConversationId::Channel(c) => write!(f, "{}", c),
            ConversationId::Group(g) => write!(f, "{}", g),
            ConversationId::DirectMessage(d) => write!(f, "{}", d),
        }
    }
}

impl ::std::convert::From<ChannelId> for ConversationId {
    fn from(id: ChannelId) -> Self {
        ConversationId::Channel(id)
    }
}

impl ::std::convert::From<GroupId> for ConversationId {
    fn from(id: GroupId) -> Self {
        ConversationId::Group(id)
    }
}

impl ::std::convert::From<DmId> for ConversationId {
    fn from(id: DmId) -> Self {
        ConversationId::DirectMessage(id)
    }
}
