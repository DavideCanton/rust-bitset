use pyo3::{exceptions::{PyValueError, PyIndexError}, prelude::*, types::{PyTuple, PyFrozenSet}};

/// Bitset implementation in Rust.
///
/// Args:
///     size (int): size of the bitset. Must be greater than zero.
#[pyclass(sequence)]
struct BitSet {
    size: usize,
    blocks: Vec<u8>,
}

#[pymethods]
impl BitSet {
    #[new]
    pub fn new(size: usize) -> PyResult<Self> {
        if size == 0 {
            Err(PyValueError::new_err("size cannot be zero"))
        } else {
            Ok(BitSet {
                size,
                blocks: vec![0; ((size - 1) >> 3) + 1],
            })
        }
    }

    pub fn __getitem__(&self, index: isize) -> PyResult<bool> {
        match self.check_index(index) {
            Err(err) => Err(err),
            Ok(adj_ind) => {
                let pos = adj_ind >> 3;
                let ind = adj_ind & 0x7;
                let mask = 1 << ind;
                Ok((self.blocks[pos] & mask) != 0)
            }
        }
    }

    pub fn __setitem__(&mut self, index: isize, value: bool) -> PyResult<()> {
        match self.check_index(index) {
            Err(err) => Err(err),
            Ok(adj_ind) => {
                let pos = adj_ind >> 3;
                let ind = adj_ind & 0x7;
                let mask = 1 << ind;
                if value {
                    self.blocks[pos] |= mask;
                } else {
                    self.blocks[pos] &= !mask;
                }
                Ok(())
            }
        }
    }

    pub fn __contains__(&self, index: usize) -> bool {
        self.__getitem__(index as isize).unwrap_or(false)
    }

    pub fn __len__(&self) -> usize {
        self.size
    }

    pub fn __repr__(&self) -> String {
        let mut vals = vec![];
        for i in 0..self.size {
            if self.__getitem__(i as isize).unwrap() {
                vals.push(i);
            }
        }
        format!("{:?}", vals)
    }

    #[getter]
    pub fn _blocks<'s>(&'s self, py: Python<'s>) -> PyResult<&PyTuple> {
        let t = PyTuple::new(py, self.blocks.iter().copied());
        Ok(t)
    }

    pub fn elements<'s>(&'s self, py: Python<'s>) -> PyResult<&PyFrozenSet> {
        let mut elements = Vec::with_capacity(self.size);
        for i in 0..self.size {
            if self.__getitem__(i as isize).unwrap() {
                elements.push(i);
            }
        }
        PyFrozenSet::new(py, elements.iter())
    }
}

impl BitSet {
    fn check_index(&self, index: isize) -> Result<usize, PyErr> {
        let size = self.size as isize;
        if index < 0 {
            Ok((index + size) as usize)
        } else if index >= size {
            let s = format!(
                "index can be between 0 and {}, found {}",
                self.size - 1,
                index
            );
            Err(PyIndexError::new_err(s))
        } else {
            Ok(index as usize)
        }
    }
}

#[pymodule]
fn rust_bitset(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BitSet>()?;
    Ok(())
}
