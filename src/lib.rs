use std::os::raw::{c_int, c_void};

use pyo3::ffi;
use pyo3::prelude::*;

// import specific to buffer protocol
use pyo3::exceptions::PyBufferError;
use pyo3::AsPyPointer;

/// # Safety
///
/// `view` must be a valid pointer to ffi::Py_buffer, or null
/// `data` must outlive the Python lifetime of `owner` (i.e. data must be owned by owner, or data
/// must be static data)
///
/// # Note:
///
/// this is copied from the pyo3 test suite w/some minimal mods
unsafe fn fill_view_from_readonly_data(
    view: *mut ffi::Py_buffer,
    flags: c_int,
    data: &[i32],
    owner: &PyAny,
) -> PyResult<()> {
    if view.is_null() {
        return Err(PyBufferError::new_err("View is null"));
    }

    if (flags & ffi::PyBUF_WRITABLE) == ffi::PyBUF_WRITABLE {
        return Err(PyBufferError::new_err("Object is not writable"));
    }

    (*view).obj = ffi::_Py_NewRef(owner.as_ptr());

    (*view).buf = data.as_ptr() as *mut c_void;
    (*view).len = data.len() as isize;
    (*view).readonly = 0;

    // This must be sizeof(T)
    (*view).itemsize = 4;

    (*view).format = if (flags & ffi::PyBUF_FORMAT) == ffi::PyBUF_FORMAT {
        // This presumably follows what the struct module
        // documents, but the Python C API docs are rather vague!
        let msg = std::ffi::CString::new("i").unwrap();
        msg.into_raw()
    } else {
        std::ptr::null_mut()
    };

    (*view).ndim = 1;
    (*view).shape = if (flags & ffi::PyBUF_ND) == ffi::PyBUF_ND {
        &mut (*view).len
    } else {
        std::ptr::null_mut()
    };

    (*view).strides = if (flags & ffi::PyBUF_STRIDES) == ffi::PyBUF_STRIDES {
        &mut (*view).itemsize
    } else {
        std::ptr::null_mut()
    };

    (*view).suboffsets = std::ptr::null_mut();
    (*view).internal = std::ptr::null_mut();

    Ok(())
}

#[pyfunction]
pub fn add(left: usize, right: usize) -> PyResult<usize> {
    Ok(left + right)
}

#[pyclass]
struct HoldsVec {
    data: Vec<i32>,
}

#[pymethods]
impl HoldsVec {
    #[new]
    fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    fn append(&mut self, value: i32) {
        self.data.push(value);
    }

    fn __len__(&self) -> usize {
        self.data.len()
    }

    // copied from pyo3 tests
    unsafe fn __getbuffer__(
        slf: &PyCell<Self>,
        view: *mut ffi::Py_buffer,
        flags: c_int,
    ) -> PyResult<()> {
        fill_view_from_readonly_data(view, flags, &slf.borrow().data, slf)
    }

    // copied from pyo3 tests
    unsafe fn __releasebuffer__(&self, view: *mut ffi::Py_buffer) {
        // Release memory held by the format string
        drop(std::ffi::CString::from_raw((*view).format));
    }
}

#[pymodule]
fn pyo3learning(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_class::<HoldsVec>()?;
    Ok(())
}
