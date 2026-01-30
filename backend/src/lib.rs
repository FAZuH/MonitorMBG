//! MonitorMBG Backend Library
//!
//! This library contains the core logic for the MonitorMBG backend, including
//! authentication, database access, middleware, routes, and services.

pub mod auth;
pub mod config;
pub mod database;
pub mod error;
pub mod logging;
pub mod middleware;
pub mod routes;
pub mod service;
