use std::{concat, stringify};

use paste::paste;
use serde::{Deserialize, Serialize};

pub trait ResourceType: Serialize + for<'de> Deserialize<'de> + private::Sealed {
    type Id: ResourceId;
}

pub trait ResourceId: EntityId {}

pub trait EntityType: Serialize + for<'de> Deserialize<'de> + private::Sealed {
    type Id: EntityId;
}

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
                #[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
                #[serde(try_from = "u8", into = "u8")]
                pub struct $r;

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
                        #[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
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
                        #[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
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
