#![allow(missing_docs)]
// TODO: unfinished functionality

use crate::proto::ToArguments;
use std::fmt;
use std::result::Result as StdResult;

pub enum Term<'a> {
    Any,
    File,
    Base,
    LastMod,
    Tag(&'a str),
}

pub struct Filter<'a> {
    typ: Term<'a>,
    what: &'a str,
}

impl<'a> Filter<'a> {
    fn new(typ: Term<'a>, what: &'a str) -> Filter<'a> {
        Filter { typ, what }
    }
}

pub struct Window(Option<(u32, u32)>);

impl From<(u32, u32)> for Window {
    fn from(window: (u32, u32)) -> Window {
        Window(Some(window))
    }
}

impl From<Option<(u32, u32)>> for Window {
    fn from(window: Option<(u32, u32)>) -> Window {
        Window(window)
    }
}

#[derive(Default)]
pub struct FilterQuery<'a> {
    filters: Vec<Filter<'a>>,
}

pub enum Query<'a> {
    Filters(FilterQuery<'a>),
    Expression(String),
}

impl<'a> FilterQuery<'a> {
    pub fn new() -> FilterQuery<'a> {
        FilterQuery { filters: Vec::new() }
    }

    pub fn and<'b: 'a>(&mut self, term: Term<'b>, value: &'b str) -> &mut FilterQuery<'a> {
        self.filters.push(Filter::new(term, value));
        self
    }
}

impl<'a> fmt::Display for Term<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Term::Any => "any",
            Term::File => "file",
            Term::Base => "base",
            Term::LastMod => "modified-since",
            Term::Tag(tag) => tag,
        })
    }
}

impl<'a> ToArguments for &'a Term<'a> {
    fn to_arguments<F, E>(&self, f: &mut F) -> StdResult<(), E>
    where
        F: FnMut(&str) -> StdResult<(), E>,
    {
        f(&self.to_string())
    }
}

impl<'a> ToArguments for &'a Filter<'a> {
    fn to_arguments<F, E>(&self, f: &mut F) -> StdResult<(), E>
    where
        F: FnMut(&str) -> StdResult<(), E>,
    {
        (&self.typ).to_arguments(f)?;
        f(&self.what)
    }
}

impl<'a> ToArguments for &'a FilterQuery<'a> {
    fn to_arguments<F, E>(&self, f: &mut F) -> StdResult<(), E>
    where
        F: FnMut(&str) -> StdResult<(), E>,
    {
        for filter in &self.filters {
            filter.to_arguments(f)?
        }
        Ok(())
    }
}

impl<'a> ToArguments for &'a Query<'a> {
    fn to_arguments<F, E>(&self, f: &mut F) -> StdResult<(), E>
    where
        F: FnMut(&str) -> StdResult<(), E>,
    {
        match self {
            Query::Filters(filters) => (&filters).to_arguments(f),
            Query::Expression(e) => e.to_arguments(f),
        }
    }
}

impl ToArguments for Window {
    fn to_arguments<F, E>(&self, f: &mut F) -> StdResult<(), E>
    where
        F: FnMut(&str) -> StdResult<(), E>,
    {
        if let Some(window) = self.0 {
            f("window")?;
            f(&format! {"{}:{}", window.0, window.1})?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::proto::ToArguments;

    fn collect<I: ToArguments>(arguments: I) -> Vec<String> {
        let mut output = Vec::<String>::new();
        arguments
            .to_arguments::<_, ()>(&mut |arg| Ok(output.push(arg.to_string())))
            .unwrap();
        output
    }

    #[test]
    fn find_window_format() {
        let window: Window = (0, 2).into();
        let output = collect(window);
        assert_eq!(output, vec!["window", "0:2"]);
    }

    #[test]
    fn find_query_format() {
        let mut query = FilterQuery::new();
        let finished = query
            .and(Term::Tag("albumartist".into()), "Mac DeMarco")
            .and(Term::Tag("album".into()), "Salad Days");
        let output = collect(&*finished);
        assert_eq!(output, vec!["albumartist", "Mac DeMarco", "album", "Salad Days"]);
    }

    #[test]
    fn multiple_and() {
        let mut query = FilterQuery::new();
        query.and(Term::Tag("albumartist".into()), "Mac DeMarco");
        query.and(Term::Tag("album".into()), "Salad Days");
        let output = collect(&query);
        assert_eq!(output, vec!["albumartist", "Mac DeMarco", "album", "Salad Days"]);
    }
}
