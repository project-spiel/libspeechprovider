#![cfg_attr(docsrs, feature(doc_cfg))]

/// No-op.
macro_rules! skip_assert_initialized {
  () => {};
}

/// XXX: We don't actually care what thread we are on.
macro_rules! assert_initialized_main_thread {
  () => {};
}

pub use auto::*;
mod auto;
