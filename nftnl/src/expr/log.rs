use super::{Expression, Rule};
use nftnl_sys::{
    self as sys,
    libc::c_char,
};

/// A log expression.
pub struct Log {
    group: Option<LogGroup>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum LogGroup {
    LogGroupZero,
    LogGroupOne,
    LogGroupTwo,
    LogGroupThree,
    LogGroupFour,
    LogGroupFive,
    LogGroupSix,
    LogGroupSeven,
}


impl Log {
    pub fn new(group: Option<LogGroup>) -> Self {
        Log { group }
    }
}

impl Expression for Log {
    fn to_expr(&self, _rule: &Rule) -> *mut sys::nftnl_expr {
        unsafe {
            let expr = try_alloc!(sys::nftnl_expr_alloc(
                b"log\0" as *const _ as *const c_char
            ));
            if let Some(group) = self.group {
                sys::nftnl_expr_set_u32(
                    expr,
                    sys::NFTNL_EXPR_LOG_GROUP as u16,
                    group as u32,
                );
            };
            expr
        }
    }
}

#[macro_export]
macro_rules! nft_expr_log {
    (group $group:ident) => {
        $crate::expr::Log::new($group)
    };
    () => {
        $crate::expr::Log::new(None)
    };
}
