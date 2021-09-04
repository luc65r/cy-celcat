//! # *Resources* and *entities*
//!
//! Celcat often require a *resource* type (like [`Student`]) in the request to
//! know what it must send back, and sometimes a *resource* ID (like [`StudentId`])
//! which identifies a particular resource.
//! Calcat sends back *entities*, identified by an *entity* type and an *entity* ID.
//!
//! An *entity* can be a *resource*, in which case it has an associated ID.
//! If it isn't a *resource*, is doesn't have an ID (`null` in JSON),
//! and we represent its type with [`Unknown`], and its ID with [`UnknownId`].

use std::{concat, stringify};

use paste::paste;
use serde::{Deserialize, Serialize};

/// A *resource* type.
///
/// This trait cannot be implemented outside of this crate.
pub trait ResourceType: Serialize + for<'de> Deserialize<'de> + private::Sealed {
    type Id: ResourceId;
}

/// A *resource* ID.
///
/// This trait cannot be implemented outside of this crate.
pub trait ResourceId: EntityId {}

/// An *entity* type.
///
/// This trait cannot be implemented outside of this crate.
pub trait EntityType: Serialize + for<'de> Deserialize<'de> + private::Sealed {
    type Id: EntityId;
}

/// An *entity* ID.
///
/// This trait cannot be implemented outside of this crate.
pub trait EntityId: Serialize + for<'de> Deserialize<'de> + private::Sealed {}

macro_rules! if_unknown {
    (
        if Unknown {
            $($i:item)*
        } else {
            $($_:item)*
        }
    ) => {
        $($i)*
    };
    (
        if $not_unknown:ident {
            $($_:item)*
        } else {
            $($i:item)*
        }
    ) => {
        $($i)*
    };
}

macro_rules! if_not_unknown {
    (
        if Unknown {
            $($_:item)*
        }
    ) => { };
    (
        if $not_unknown:ident {
            $($i:item)*
        }
    ) => {
        $($i)*
    };
}

macro_rules! entities {
    (
        $(
            $r:ident = $n:literal,
        )+
    ) => {
        $(
            paste! {
                // TODO: find a way to not write definitions 2 times
                if_unknown! {
                    if $r {
                        #[doc = "The unknown entity type."]
                        #[derive(Debug, Default, Clone, Copy, PartialEq)]
                        #[derive(Serialize, Deserialize)]
                        #[serde(try_from = "u8", into = "u8")]
                        pub struct $r;
                    } else {
                        #[doc = "The " $r:lower " resource type."]
                        #[derive(Debug, Default, Clone, Copy, PartialEq)]
                        #[derive(Serialize, Deserialize)]
                        #[serde(try_from = "u8", into = "u8")]
                        pub struct $r;
                    }
                }

                impl From<$r> for u8 {
                    fn from(_: $r) -> Self {
                        $n
                    }
                }

                impl TryFrom<u8> for $r {
                    type Error = &'static str;
                    fn try_from(n: u8) -> Result<Self, Self::Error> {
                        if n == $n {
                            Ok($r)
                        } else {
                            Err(concat!("expected ", $n, " (", stringify!($r), ")"))
                        }
                    }
                }

                impl private::Sealed for $r {}
                impl EntityType for $r {
                    type Id = [<$r Id>];
                }
                if_not_unknown! {
                    if $r {
                        impl ResourceType for $r {
                            type Id = [<$r Id>];
                        }
                    }
                }

                if_unknown! {
                    if $r {
                        #[derive(Debug, Default, Clone, PartialEq)]
                        #[derive(Serialize, Deserialize)]
                        #[serde(from = "()", into = "()")]
                        pub struct [<$r Id>];
                        impl From<[<$r Id>]> for () {
                            fn from(_: [<$r Id>]) -> Self {}
                        }
                        impl From<()> for [<$r Id>] {
                            fn from(_: ()) -> Self {
                                Self
                            }
                        }
                    } else {
                        #[derive(Debug, Default, Clone, PartialEq)]
                        #[derive(Serialize, Deserialize)]
                        #[repr(transparent)]
                        pub struct [<$r Id>](pub String);
                    }
                }

                impl private::Sealed for [<$r Id>] {}
                impl EntityId for [<$r Id>] {}
                if_not_unknown! {
                    if $r {
                        impl ResourceId for [<$r Id>] {}
                    }
                }
            }
        )+
    };
}

entities! {
    Unknown = 0,
    Module = 100,
    Staff = 101,
    Room = 102,
    Group = 103,
    Student = 104,
    Team = 105,
    Equipment = 106,
    Course = 107,
}

mod private {
    /// Empty trait that no struct/enum can implement outside of this crate.
    ///
    /// Used as a trait bound for traits that shouldn't be implemented outside of this crate.
    pub trait Sealed {}
}
