use crate::error::datajoint_core_set_last_error;
use crate::results::table_row_vector::TableRowVector;
use crate::util;
use datajoint_core::results::TableRow;
use datajoint_core::{
    connection::Cursor,
    error::{DataJointError, ErrorCode},
};

#[no_mangle]
pub unsafe extern "C" fn cursor_free(this: *mut Cursor) {
    if !this.is_null() {
        Box::from_raw(this);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cursor_next(this: *mut Cursor, out: *mut *mut TableRow) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let cursor = &mut *this;
    match std::pin::Pin::as_mut(cursor).get_unchecked_mut().try_next() {
        Err(error) => datajoint_core_set_last_error(error) as i32,
        Ok(value) => {
            util::mem::handle_output_ptr(out, value);
            ErrorCode::Success as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cursor_rest(this: *mut Cursor, out: *mut *mut TableRowVector) -> i32 {
    if this.is_null() {
        return datajoint_core_set_last_error(DataJointError::new(ErrorCode::NullNotAllowed))
            as i32;
    }
    let cursor = &mut *this;
    match std::pin::Pin::as_mut(cursor).get_unchecked_mut().try_rest() {
        Err(error) => datajoint_core_set_last_error(error) as i32,
        Ok(value) => {
            util::mem::handle_output_ptr(out, TableRowVector::new(value));
            ErrorCode::Success as i32
        }
    }
}
