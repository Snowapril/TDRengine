use lock_api::{
    MappedMutexGuard, MappedRwLockReadGuard, MappedRwLockWriteGuard, Mutex, MutexGuard, RwLock,
    RwLockReadGuard, RwLockUpgradableReadGuard, RwLockWriteGuard,
};
use log::{log, Level};
use once_cell::sync::{Lazy, OnceCell};
use parking_lot::{RawMutex, RawRwLock, RawThreadId};
use static_cell::StaticCell;
use std::process::ExitCode;

pub struct TdrLog(Level, &'static str);

impl TdrLog {}

pub struct Logger {}

pub fn logger() -> &'static RwLock<RawRwLock, Option<Logger>> {
    static LOGGER_INSTANCE: OnceCell<RwLock<RawRwLock, Option<Logger>>> = OnceCell::new();
    LOGGER_INSTANCE.get_or_init(|| RwLock::new(None))
}
