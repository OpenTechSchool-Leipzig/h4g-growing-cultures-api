use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, multispace0, not_line_ending, space0},
    sequence::{delimited, terminated},
    IResult,
};
use pulldown_cmark::{html, Event, Parser, Tag};
use url::Url;

use crate::{Content, InfoNode};

#[derive(PartialEq, Debug)]
pub enum ParseError<'a> {
    InvalidUrl(url::ParseError),
    GenericParseError(nom::Err<nom::error::Error<&'a str>>),
}

impl<'a> From<nom::Err<nom::error::Error<&'a str>>> for ParseError<'a> {
    fn from(source: nom::Err<nom::error::Error<&'a str>>) -> Self {
        Self::GenericParseError(source)
    }
}

impl<'a> From<url::ParseError> for ParseError<'a> {
    fn from(source: url::ParseError) -> Self {
        Self::InvalidUrl(source)
    }
}

impl<'a> TryFrom<(&str, &'a str)> for InfoNode {
    type Error = ParseError<'a>;

    fn try_from((filename, input): (&str, &'a str)) -> Result<Self, Self::Error> {
        let (input, front_matter) = front_matter(input)?;
        let (_, title) = yaml_value("title")(front_matter)?;

        let mut items = Vec::new();
        for section in markdown_sections(input) {
            items.push(Content::try_from(section)?);
        }

        Ok(Self {
            ty: filename.to_string(),
            title: title.to_string(),
            items,
        })
    }
}

impl<'a> TryFrom<Vec<Event<'a>>> for Content {
    type Error = ParseError<'a>;

    fn try_from(source: Vec<Event<'a>>) -> Result<Self, Self::Error> {
        use Event::{End, Start, Text};
        use Tag::{Image, Link, Paragraph};

        match &source[..] {
            [Start(Paragraph), Start(Image(_, url, _)), Text(title), End(Image(_, _, _)), End(Paragraph)] => {
                Ok(Content::Image(Url::parse(url)?, title.to_string()))
            }
            [Start(Paragraph), Start(Link(_, url, _)), Text(title), End(Link(_, _, _)), End(Paragraph)] => {
                Ok(Content::from_url_and_title(
                    Url::parse(url)?,
                    title.to_string(),
                ))
            }
            _ => {
                let mut buf = String::new();
                html::push_html(&mut buf, source.into_iter());
                Ok(Content::Text(buf.trim().to_string()))
            }
        }
    }
}

pub(crate) fn front_matter(input: &str) -> IResult<&str, &str> {
    delimited(tag("---"), take_until("---"), tag("---"))(input)
}

// TODO: Use a proper yaml parser. I'm currently on a train without internet
// access. Sometimes you have to use what you got ;-).
pub(crate) fn yaml_value<'a>(key: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    move |i| {
        let (i, _) = multispace0(i)?;
        let (i, _) = tag(key)(i)?;
        let (i, _) = tag(":")(i)?;
        let (i, _) = space0(i)?;
        terminated(not_line_ending, line_ending)(i)
    }
}

// TODO: Check name, maybe "section" has a specific meaning in Markdown.
pub(crate) fn markdown_sections(input: &str) -> Vec<Vec<Event<'_>>> {
    let parser = Parser::new(input);
    let mut result = Vec::new();
    let mut cur_events = Vec::new();
    for event in parser {
        match event {
            Event::Rule => result.push(cur_events.drain(..).collect::<Vec<_>>()),
            e => cur_events.push(e),
        }
    }
    if !cur_events.is_empty() {
        result.push(cur_events);
    }
    result
}
