use crate::errors::Result;
use crate::state::DbState;
use std::sync::Arc;
use welds_connections::Row;

// This is a collection of extensions specific to returned data to make using welds simpler

/// For State collections, these are extensions to remove the DbState.
/// This is helpful when passing to a view type layer.
pub trait VecStateExt<T> {
    fn to_vms(self) -> Arc<Vec<Arc<T>>>;
    fn into_inners(self) -> Vec<T>;
}

impl<T> VecStateExt<T> for Vec<DbState<T>> {
    /// convert from DbState wrapped data to Arc wrapped data.
    /// Very useful when passing to a View layer such as Yew (Server Side)
    fn to_vms(mut self) -> Arc<Vec<Arc<T>>> {
        let vec: Vec<_> = self.drain(..).map(|x| x.into_vm()).collect();
        Arc::new(vec)
    }

    /// convert from DbState wrapped data to Unwrapped Models
    fn into_inners(mut self) -> Vec<T> {
        self.drain(..).map(|x| x.into_inner()).collect()
    }
}

/// For welds_connections::Row collections, these are extensions to
/// Allow for easy mapping into Welds Objects.
pub trait VecRowExt<T> {
    fn collect_into(self) -> Result<Vec<T>>;
}

impl<T> VecRowExt<T> for Vec<Row>
where
    T: TryFrom<Row>,
    T::Error: Into<crate::WeldsError>,
{
    fn collect_into(mut self) -> crate::errors::Result<Vec<T>> {
        let outs: std::result::Result<Vec<T>, _> = self.drain(..).map(T::try_from).collect();
        match outs {
            Err(err) => Err(err.into()),
            Ok(r) => Ok(r),
        }
    }
}
