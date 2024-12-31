macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }

        impl std::convert::TryFrom<i64> for $name {
            type Error = ();

            fn try_from(v: i64) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i64 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

back_to_enum!(
    pub(crate) enum ChatCommand {
        Ping = 0,
        Pong = 10000,
        Connect = 100,
        Connected = 10100,
        RequestRecentChat = 5101,
        RecentChat = 15101,
        Event = 93006,
        Chat = 93101,
        Donation = 93102,
        Kick = 94005,
        Block = 94006,
        Blind = 94008,
        Notice = 94010,
        Penalty = 94015,
        SendChat = 3101,
    }
);

back_to_enum!(
    pub(crate) enum ChatType {
        Text = 1,
        Image = 2,
        Sticker = 3,
        Video = 4,
        Rich = 5,
        Donation = 10,
        Subscription = 11,
        SystemMessage = 30,
    }
);
