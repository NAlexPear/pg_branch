use pgrx::{is_a, prelude::*};

/// All hooks needed to intercept and process CREATE DATABASE queries.
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
        // only block CREATE DATABASE, forwarding all others
        if unsafe { is_a(pstmt.utilityStmt, pg_sys::NodeTag_T_CreatedbStmt) } {
            // extract target and template databases from the statement
            let createdb =
                unsafe { PgBox::from_pg(pstmt.utilityStmt as *mut pg_sys::CreatedbStmt) };
            let target = match unsafe { core::ffi::CStr::from_ptr(createdb.dbname) }.to_str() {
                Ok(dbname) => dbname,
                Err(error) => error!("Invalid target database: {}", error),
            };

            // FIXME: traverse the List of options to determine the template
            // create the new branch using the top-level helper function
            pgrx::HookResult::new(crate::branch(target, None))
        } else {
            prev_hook(
                pstmt,
                query_string,
                read_only_tree,
                context,
                params,
                query_env,
                dest,
                completion_tag,
            )
        }
    }
}

static mut HOOKS: Hooks = Hooks;

/// initialize all of the hooks for use with _PG_init
pub unsafe fn init() {
    pgrx::register_hook(&mut HOOKS)
}
