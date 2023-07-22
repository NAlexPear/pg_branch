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
        // FIXME: check CREATEDB privilege of the user
        if unsafe { is_a(pstmt.utilityStmt, pg_sys::NodeTag_T_CreatedbStmt) } {
            let createdb =
                unsafe { PgBox::from_pg(pstmt.utilityStmt as *mut pg_sys::CreatedbStmt) };

            // extract the target from the statement's dbname
            let target = unsafe { core::ffi::CStr::from_ptr(createdb.dbname) }
                .to_str()
                .expect("Invalid dbname in CREATE DATABASE");

            // parse and handle relevant options
            let mut template = None;
            if !createdb.options.is_null() {
                let options = unsafe { PgBox::from_pg(createdb.options as *mut pg_sys::List) };
                for index in 0..options.length {
                    let list_cell = unsafe { pg_sys::pgrx_list_nth(options.as_ptr(), index) };
                    let element = unsafe { PgBox::from_pg(list_cell as *mut pg_sys::DefElem) };
                    let defname = unsafe { core::ffi::CStr::from_ptr(element.defname) }
                        .to_str()
                        .expect("Invalid template name in CREATE DATABASE");
                    let arg = unsafe { PgBox::from_pg(element.arg as *mut pg_sys::Node) }
                        .to_string()
                        .replace("\"", "");

                    match defname {
                        "template" => {
                            template = Some(arg);
                        }
                        "strategy" if arg.to_lowercase() != "snapshot" => {
                            // if a strategy is explicitly defined as something other than
                            // "snapshot", forward the call to prev_hook instead
                            return prev_hook(
                                pstmt,
                                query_string,
                                read_only_tree,
                                context,
                                params,
                                query_env,
                                dest,
                                completion_tag,
                            );
                        }
                        // FIXME: support more of the CREATE DATABASE options
                        _ => (),
                    }
                }
            };

            // create the new branch using the top-level helper function
            pgrx::HookResult::new(crate::branch(target, template.as_deref()))
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
