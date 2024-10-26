use welds_connections::Param;

/// This is a nice little wrapper to make sending
/// Params to where_custom() simpler
#[derive(Default)]
pub struct ManualWhereParam(Vec<Box<dyn Param + Send + Sync>>);

impl ManualWhereParam {
    pub fn new() -> Self {
        ManualWhereParam::default()
    }

    pub fn push<P>(mut self, p: P) -> Self
    where
        P: Param + Send + Sync,
        P: 'static,
    {
        self.0.push(Box::new(p));
        self
    }

    pub(crate) fn into_inner(self) -> Vec<Box<dyn Param + Send + Sync>> {
        self.0
    }
}
