use alloc::vec::Vec;
use core::num::TryFromIntError;
use thiserror::Error;

mod fact;
mod specification;

#[derive(Error, Debug)]
#[error("AST error")]
enum AstError {
    PoolCapacityExceeded(TryFromIntError),
    PoolIndexNotFound(PoolIndex),
    #[error("usize was outside of pool size range: {0}")]
    PoolArchitectureError(TryFromIntError)
}

struct Label(Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct PoolIndex(u32);
impl From<u32> for PoolIndex {
    fn from(value: u32) -> Self {
        PoolIndex(value)
    }
}

impl TryFrom<usize> for PoolIndex {
    type Error = AstError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        u32::try_from(value).map(PoolIndex).map_err(AstError::PoolArchitectureError)
    }
}
impl TryFrom<PoolIndex> for usize {
    type Error = AstError;
    fn try_from(value: PoolIndex) -> Result<Self, Self::Error> {
        usize::try_from(value.0).map_err(AstError::PoolArchitectureError)
    }
}

struct Pool<TAst> (Vec<TAst>);
impl<TAst> Pool<TAst> {
    pub fn new() -> Self {
        Pool(Vec::new())
    }

    pub fn add(&mut self, ast: TAst) -> Result<PoolIndex, AstError> {
        let index: PoolIndex = self.0.len().try_into()?;
        self.0.push(ast);
        Ok(index)
    }

    pub fn get(&self, index: PoolIndex) -> Result<&TAst, AstError> {
        let idx: usize = index.try_into()?;
        self.0.get(idx).ok_or(AstError::PoolIndexNotFound(index))
    }
}