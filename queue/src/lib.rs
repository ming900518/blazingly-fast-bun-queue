#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::perf)]
#![allow(non_snake_case, clippy::missing_panics_doc, clippy::missing_safety_doc)]

use chrono::Utc;
use parking_lot::{const_rwlock, Condvar, Mutex, RwLock};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    ffi::{c_char, CStr, CString},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, OnceLock,
    },
    thread::spawn,
};

type OnceMutexWithCondvar = OnceLock<Arc<(Mutex<Vec<Option<(i64, String)>>>, Condvar)>>;

static QUEUE: OnceMutexWithCondvar = OnceLock::new();
static RESULT: RwLock<Vec<(i64, String, String)>> = const_rwlock(Vec::new());
static COUNT: AtomicU64 = AtomicU64::new(0);

#[no_mangle]
pub extern "C" fn init() {
    QUEUE.get_or_init(|| Arc::new((Mutex::new(Vec::new()), Condvar::new())));
    spawn(move || {
        let queue = QUEUE.get().unwrap().clone();
        let (lock, condvar) = &*queue;
        let queue_value = &mut lock.lock();
        loop {
            condvar.wait(queue_value);
            queue_value.par_iter_mut().for_each(|data| {
                if let Some(data) = data.take() {
                    COUNT.fetch_add(1, Ordering::Release);
                    process(data);
                }
            });
            queue_value.retain(Option::is_some);
        }
    });
}

fn process((req_number, data): (i64, String)) {
    RESULT
        .write()
        .push((req_number, format!("{}", Utc::now()), data));
}

#[no_mangle]
pub unsafe extern "C" fn addQueue(data: *const c_char) -> i64 {
    let queue = QUEUE.get().unwrap().clone();
    let (lock, condvar) = &*queue;
    let req_number = Utc::now().timestamp_millis();
    lock.lock().push(Some((
        req_number,
        CStr::from_ptr(data).to_str().unwrap_or_default().to_owned(),
    )));
    condvar.notify_one();
    req_number
}

#[derive(Serialize)]
struct Data {
    processedCount: u64,
    dataProcessTime: String,
    inputData: String,
}

#[no_mangle]
pub unsafe extern "C" fn fetchResult(req_number: i64) -> *const c_char {
    let data = RESULT
        .read()
        .par_iter()
        .find_any(|(key, _, _)| key == &req_number)
        .as_ref()
        .map_or_else(
            || Data {
                processedCount: COUNT.load(Ordering::Relaxed),
                dataProcessTime: "not yet processed.".to_owned(),
                inputData: "not yet processed".to_owned(),
            },
            |result| Data {
                processedCount: COUNT.load(Ordering::Relaxed),
                dataProcessTime: result.1.clone(),
                inputData: result.2.clone(),
            },
        );
    CString::new(serde_json::to_string(&data).unwrap_or_default())
        .unwrap_or_default()
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn fetchInputVec() -> *const c_char {
    CString::new(serde_json::to_string(&*QUEUE.get().unwrap().0.lock()).unwrap_or_default())
        .unwrap_or_default()
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn fetchResultVec() -> *const c_char {
    CString::new(serde_json::to_string(&*RESULT.read()).unwrap_or_default())
        .unwrap_or_default()
        .into_raw()
}
