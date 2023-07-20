#![allow(unused)] // FIXME: remove this

/// All hooks needed to intercept and process CREATE DATABASE queries.
use pgrx::prelude::*;

struct Hooks;
impl pgrx::PgHooks for Hooks {
    /// hook into the ProcessUtility hook to intercept CREATE DATABASE calls
    fn process_utility_hook(
        &mut self,
        pstmt: PgBox<pg_sys::PlannedStmt>,
        query_string: &core::ffi::CStr,
        read_only_tree: Option<bool>,
        context: pg_sys::ProcessUtilityContext,
        params: PgBox<pg_sys::ParamListInfoData>,
        query_env: PgBox<pg_sys::QueryEnvironment>,
        dest: PgBox<pg_sys::DestReceiver>,
        completion_tag: *mut pg_sys::QueryCompletion,
        prev_hook: fn(
            pstmt: PgBox<pg_sys::PlannedStmt>,
            query_string: &core::ffi::CStr,
            read_only_tree: Option<bool>,
            context: pg_sys::ProcessUtilityContext,
            params: PgBox<pg_sys::ParamListInfoData>,
            query_env: PgBox<pg_sys::QueryEnvironment>,
            dest: PgBox<pg_sys::DestReceiver>,
            completion_tag: *mut pg_sys::QueryCompletion,
        ) -> pgrx::HookResult<()>,
    ) -> pgrx::HookResult<()> {
        // TODO: only block CREATE DATABASE, forwarding all others
        // TODO: extract target and template databases from the query itself
        // TODO: pass the values to branch()
        pgrx::HookResult::new(())
    }
}

static mut HOOKS: Hooks = Hooks;

/// initialize all of the hooks for use with _PG_init
pub unsafe fn init() {
    pgrx::register_hook(&mut HOOKS)
}
