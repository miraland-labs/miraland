pub(crate) mod miraland {
    pub(crate) mod wen_restart_proto {
        include!(concat!(env!("OUT_DIR"), "/miraland.wen_restart_proto.rs"));
    }
}

pub(crate) mod last_voted_fork_slots_aggregate;
pub mod wen_restart;
