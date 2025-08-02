//! This module handles connection and communication with our databases. For now, it only contains
//! `redis` module, used for caching daily bonus information.
//!
//! Later, it should include a `mongo` module, which will access a mongodb database, and keep track
//! of player achievement progress.

pub mod redis;
