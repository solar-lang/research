pub(crate) use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
pub struct Located<'a, T> {
    pub span: Span<'a>,
    pub value: T,
}

impl<'a, T> Located<'a, T> {
    pub fn new(span: Span<'a>, value: T) -> Self {
        Self { span, value }
    }
}

impl<'a, T> std::cmp::PartialEq for Located<'a, T>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<'a, T> Eq for Located<'a, T> where T: Eq {}

impl<'a, T> std::cmp::PartialOrd for Located<'a, T>
where
    T: PartialOrd<T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<'a, T> std::cmp::Ord for Located<'a, T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<'a, T> Clone for Located<'a, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Located::new(self.span, self.value.clone())
    }
}
