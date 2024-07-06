use serenity::all::{Mentionable, UserId};

use crate::relationships::Relationship;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // region common
    Zayden,
    Bot,
    InvalidUserId,
    AlreadyRelated {
        target: UserId,
        relationship: Relationship,
    },
    UnauthorisedUser,
    NoMentionedUser,
    NoInteraction,
    SameUser(UserId),
    // endregion

    // region adopt
    UserSelfAdopt,
    AlreadyAdopted(UserId),
    AdoptCancelled,
    // endregion

    // region block
    UserSelfBlock,
    // endregion

    //region children
    SelfNoChildren,
    NoChildren(UserId),
    // endregion

    // region marry
    UserSelfMarry,
    MaxPartners,
    // endregion

    // region external
    Serenity(serenity::Error),
    SerenityTimestamp(serenity::model::timestamp::InvalidTimestamp),
    Sqlx(sqlx::Error),
    EnvVar(std::env::VarError),
    // Reqwest(reqwest::Error),
    // Cron(cron::error::Error),
    ParseIntError(std::num::ParseIntError),
    ReactionConversionError(serenity::all::ReactionConversionError),
    // JoinError(tokio::task::JoinError),
    // CharmingError(charming::EchartsError),
    // endregion
}

impl Error {
    pub fn as_response(&self) -> String {
        match self {
            Self::UserSelfMarry => String::from("You can't marry yourself!"),
            Self::Bot => String::from("Can robots even love?"),
            Self::Zayden => String::from("Please... I can do better than you."),
            Self::AlreadyRelated {
                target,
                relationship,
            } => {
                format!(
                    "You guys are already related! {} is your {relationship}.",
                    target.mention()
                )
            }
            Self::MaxPartners => String::from(
                "You're already at your partner limit! Use `/divorce` to break up with someone.",
            ),
            Self::UnauthorisedUser => String::from("You can't respond to this interaction."),
            Self::SameUser(user_id) => format!(
                "Would you look at that... {0} is very closely related to {0}",
                user_id.mention()
            ),
            Self::UserSelfAdopt => String::from("You can't adopt yourself!"),
            Self::AlreadyAdopted(user_id) => {
                format!("It looks like {} already has a parent.", user_id.mention())
            }
            _ => String::new(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<serenity::Error> for Error {
    fn from(e: serenity::Error) -> Self {
        Error::Serenity(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Sqlx(e)
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::EnvVar(e)
    }
}

// impl From<reqwest::Error> for Error {
//     fn from(e: reqwest::Error) -> Self {
//         Error::Reqwest(e)
//     }
// }

// impl From<cron::error::Error> for Error {
//     fn from(e: cron::error::Error) -> Self {
//         Error::Cron(e)
//     }
// }

impl From<serenity::model::timestamp::InvalidTimestamp> for Error {
    fn from(e: serenity::model::timestamp::InvalidTimestamp) -> Self {
        Error::SerenityTimestamp(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseIntError(e)
    }
}

impl From<serenity::all::ReactionConversionError> for Error {
    fn from(e: serenity::all::ReactionConversionError) -> Self {
        Error::ReactionConversionError(e)
    }
}

// impl From<tokio::task::JoinError> for Error {
//     fn from(e: tokio::task::JoinError) -> Self {
//         Error::JoinError(e)
//     }
// }

// impl From<charming::EchartsError> for Error {
//     fn from(e: charming::EchartsError) -> Self {
//         Error::CharmingError(e)
//     }
// }
