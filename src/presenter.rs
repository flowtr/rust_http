use crate::response::CompletedResponse;
use ansi_term::Color::RGB;
use anyhow::Result;
use std::fmt;

pub struct Presenter {
    colorize: bool,
    response: Result<CompletedResponse>,
}

impl Presenter {
    pub fn new(response: Result<CompletedResponse>) -> Presenter {
        Presenter {
            colorize: false,
            response,
        }
    }

    pub fn colorize(mut self, colorize: bool) -> Presenter {
        self.colorize = colorize;
        self
    }
}

impl fmt::Display for Presenter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Presenter {
                response: Ok(response),
                ..
            } => {
                // We're ignoring colorizing for now
                // write the headers
                for (name, value) in response.headers().iter() {
                    writeln!(f, "{}: {}", name, value.to_str().unwrap_or(""))?;
                }
                writeln!(f, "---")?;
                write!(
                    f,
                    "{}",
                    if self.colorize {
                        RGB(32, 227, 64).paint(response.text()).to_string()
                    } else {
                        response.text()
                    }
                )
            }
            Presenter {
                response: Err(err), ..
            } => write!(
                f,
                "{}",
                if self.colorize {
                    RGB(227, 4, 32).paint(err.to_string()).to_string()
                } else {
                    err.to_string()
                }
            ),
        }
    }
}

impl From<Result<CompletedResponse>> for Presenter {
    fn from(res: Result<CompletedResponse>) -> Presenter {
        Presenter {
            colorize: false,
            response: res,
        }
    }
}
