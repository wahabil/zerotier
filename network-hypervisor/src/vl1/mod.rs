// (c) 2020-2022 ZeroTier, Inc. -- currently proprietary pending actual release and licensing. See LICENSE.md.

mod address;
mod api;
mod endpoint;
mod event;
mod mac;
mod node;
mod path;
mod peer;
mod peermap;
mod rootset;
mod whois;

pub mod identity;
pub mod inetaddress;

pub use address::{Address, PartialAddress};
pub use api::{ApplicationLayer, InnerProtocolLayer, PacketHandlerResult};
pub use endpoint::Endpoint;
pub use event::Event;
pub use inetaddress::InetAddress;
pub use mac::MAC;
pub use node::Node;
pub use path::Path;
pub use peer::Peer;
pub use rootset::{Root, RootSet};

pub use zerotier_crypto::typestate::Valid;

#[cfg(feature = "debug_events")]
#[allow(unused_macros)]
#[macro_export]
macro_rules! debug_event {
    ($si:expr, $fmt:expr $(, $($arg:tt)*)?) => {
        use $crate::vl1::Event;
        $si.event(Event::Debug(file!(), line!(), format!($fmt, $($($arg)*)?)));
    }
}

#[cfg(not(feature = "debug_events"))]
#[allow(unused_macros)]
#[macro_export]
macro_rules! debug_event {
    ($si:expr, $fmt:expr $(, $($arg:tt)*)?) => {};
}

#[allow(unused_imports)]
pub use debug_event;
