#[cfg(not(target_family = "wasm"))]
mod app;
#[cfg(not(target_family = "wasm"))]
mod calendar;
#[cfg(not(target_family = "wasm"))]
mod db;

mod dates;
mod error;

#[cfg(any(target_os = "ios", target_os = "android"))]
uniffi::setup_scaffolding!();
