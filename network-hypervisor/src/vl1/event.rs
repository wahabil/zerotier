// (c) 2020-2022 ZeroTier, Inc. -- currently proprietary pending actual release and licensing. See LICENSE.md.

use crate::vl1::identity::Identity;

#[derive(Clone)]
pub enum Event {
    // Change in node online status.
    Online(bool),

    // Debug event: source file, line number, message (only if feature 'debug_events' is enabled).
    Debug(&'static str, u32, String),

    // An anomalous event has been encountered that could indicate a possible security problem.
    SecurityWarning(String),

    // A fatal error has occurred.
    FatalError(String),

    // This node has automatically generated an identity.
    IdentityAutoGenerated(Identity),

    // This node's identity has automatically been upgraded, contains old and new.
    IdentityAutoUpgraded(Identity, Identity),

    // The list of roots has been updated, contains old and new.
    UpdatedRoots(Vec<Identity>, Vec<Identity>),
}

impl ToString for Event {
    fn to_string(&self) -> String {
        match self {
            Event::Online(online) => format!("[vl1] online == {}", online),
            Event::Debug(l, f, m) => format!("[debug] {}:{} {}", l.split("/").last().unwrap(), f, m),
            Event::SecurityWarning(w) => format!("[global] security warning: {}", w),
            Event::FatalError(e) => format!("[global] FATAL: {}", e),
            Event::IdentityAutoGenerated(id) => format!("[vl1] identity auto-generated: {}", id.to_string()),
            Event::IdentityAutoUpgraded(oldid, newid) => format!("[vl1] identity upgrade: {} => {}", oldid.to_string(), newid.to_string()),
            Event::UpdatedRoots(_, newroots) => {
                let mut tmp = String::with_capacity(128);
                tmp.push_str("[vl1] updated root set:");
                for r in newroots.iter() {
                    tmp.push(' ');
                    tmp.push_str(r.address.to_string().as_str());
                }
                tmp
            }
        }
    }
}
