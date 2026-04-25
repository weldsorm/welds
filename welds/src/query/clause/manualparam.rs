use welds_connections::Param;

/// This is a nice little wrapper to make sending
/// Params to where_custom() and set_custom simpler
#[derive(Default)]
pub struct ManualParam(Vec<Box<dyn Param + Send + Sync>>);

impl ManualParam {
    pub fn new() -> Self {
        ManualParam::default()
    }

    /// Add a value to the list of Params to send to the database.
    #[deprecated(
        note = "Push will be changed in a future version of welds to behave like std push. Use `push_mut` or `with` instread"
    )]
    pub fn push<P>(self, p: P) -> Self
    where
        P: Param + Send + Sync,
        P: 'static,
    {
        self.with(p)
    }

    /// Add a value to the list of Params to send to the database.
    /// returns a self reference so that pushes can be chained
    pub fn push_mut<P>(&mut self, p: P) -> &mut Self
    where
        P: Param + Send + Sync,
        P: 'static,
    {
        self.0.push(Box::new(p));
        self
    }

    /// Add a value to the list of Params to send to the database.
    /// Part of the builder constructor
    pub fn with<P>(mut self, p: P) -> Self
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

impl From<()> for ManualParam {
    fn from(_value: ()) -> Self {
        ManualParam::default()
    }
}

impl<T1> From<(T1,)> for ManualParam
where
    T1: 'static + Param + Send + Sync,
{
    fn from(p: (T1,)) -> Self {
        ManualParam::new().with(p.0)
    }
}

impl<T1, T2> From<(T1, T2)> for ManualParam
where
    T1: 'static + Param + Send + Sync,
    T2: 'static + Param + Send + Sync,
{
    fn from(p: (T1, T2)) -> Self {
        ManualParam::new().with(p.0).with(p.1)
    }
}

impl<T1, T2, T3> From<(T1, T2, T3)> for ManualParam
where
    T1: 'static + Param + Send + Sync,
    T2: 'static + Param + Send + Sync,
    T3: 'static + Param + Send + Sync,
{
    fn from(p: (T1, T2, T3)) -> Self {
        ManualParam::new().with(p.0).with(p.1).with(p.2)
    }
}

impl<T1, T2, T3, T4> From<(T1, T2, T3, T4)> for ManualParam
where
    T1: 'static + Param + Send + Sync,
    T2: 'static + Param + Send + Sync,
    T3: 'static + Param + Send + Sync,
    T4: 'static + Param + Send + Sync,
{
    fn from(p: (T1, T2, T3, T4)) -> Self {
        ManualParam::new().with(p.0).with(p.1).with(p.2).with(p.3)
    }
}

impl<T1, T2, T3, T4, T5> From<(T1, T2, T3, T4, T5)> for ManualParam
where
    T1: 'static + Param + Send + Sync,
    T2: 'static + Param + Send + Sync,
    T3: 'static + Param + Send + Sync,
    T4: 'static + Param + Send + Sync,
    T5: 'static + Param + Send + Sync,
{
    fn from(p: (T1, T2, T3, T4, T5)) -> Self {
        ManualParam::new()
            .with(p.0)
            .with(p.1)
            .with(p.2)
            .with(p.3)
            .with(p.4)
    }
}

impl<T1, T2, T3, T4, T5, T6> From<(T1, T2, T3, T4, T5, T6)> for ManualParam
where
    T1: 'static + Param + Send + Sync,
    T2: 'static + Param + Send + Sync,
    T3: 'static + Param + Send + Sync,
    T4: 'static + Param + Send + Sync,
    T5: 'static + Param + Send + Sync,
    T6: 'static + Param + Send + Sync,
{
    fn from(p: (T1, T2, T3, T4, T5, T6)) -> Self {
        ManualParam::new()
            .with(p.0)
            .with(p.1)
            .with(p.2)
            .with(p.3)
            .with(p.4)
            .with(p.5)
    }
}

impl<T1, T2, T3, T4, T5, T6, T7> From<(T1, T2, T3, T4, T5, T6, T7)> for ManualParam
where
    T1: 'static + Param + Send + Sync,
    T2: 'static + Param + Send + Sync,
    T3: 'static + Param + Send + Sync,
    T4: 'static + Param + Send + Sync,
    T5: 'static + Param + Send + Sync,
    T6: 'static + Param + Send + Sync,
    T7: 'static + Param + Send + Sync,
{
    fn from(p: (T1, T2, T3, T4, T5, T6, T7)) -> Self {
        ManualParam::new()
            .with(p.0)
            .with(p.1)
            .with(p.2)
            .with(p.3)
            .with(p.4)
            .with(p.5)
            .with(p.6)
    }
}
