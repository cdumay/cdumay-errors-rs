#![feature(try_trait)]
extern crate serde;
extern crate serde_value;

#[macro_use]
extern crate serde_derive;

pub trait ErrorProps {
    fn code(&self) -> &u16;
    fn code_mut(&mut self) -> &mut u16;
    fn extra(&self) -> &Option<std::collections::HashMap<String, serde_value::Value>>;
    fn extra_mut(&mut self) -> &mut Option<std::collections::HashMap<String, serde_value::Value>>;
    fn message(&self) -> &String;
    fn message_mut(&mut self) -> &mut String;
    fn msgid(&self) -> &String;
    fn msgid_mut(&mut self) -> &mut String;
    fn stack(&self) -> &Option<String>;
    fn stack_mut(&mut self) -> &mut Option<String>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorRepr {
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<std::collections::HashMap<String, serde_value::Value>>,
    message: String,
    msgid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack: Option<String>,
}


impl Default for ErrorRepr {
    fn default() -> ErrorRepr {
        ErrorRepr {
            msgid: "Err-00000".to_string(),
            message: String::new(),
            code: 500,
            extra: None,
            stack: None,
        }
    }
}

impl ErrorProps for ErrorRepr {
    fn code(&self) -> &u16 { &self.code }
    fn code_mut(&mut self) -> &mut u16 { &mut self.code }

    fn extra(&self) -> &Option<std::collections::HashMap<String, serde_value::Value>> { &self.extra }
    fn extra_mut(&mut self) -> &mut Option<std::collections::HashMap<String, serde_value::Value>> { &mut self.extra }

    fn message(&self) -> &String { &self.message }
    fn message_mut(&mut self) -> &mut String { &mut self.message }

    fn msgid(&self) -> &String { &self.msgid }
    fn msgid_mut(&mut self) -> &mut String { &mut self.msgid }

    fn stack(&self) -> &Option<String> { &self.stack }
    fn stack_mut(&mut self) -> &mut Option<String> { &mut self.stack }
}


impl std::error::Error for ErrorRepr {
    fn description(&self) -> &str {
        self.message().as_str()
    }
}

impl std::fmt::Display for ErrorRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.msgid(), self.message())
    }
}


impl From<std::option::NoneError> for ErrorRepr {
    fn from(_err: std::option::NoneError) -> ErrorRepr {
        ErrorRepr {
            msgid: "Err-08414".to_string(),
            message: "Not Found".to_string(),
            code: 404,
            extra: None,
            stack: None,
        }
    }
}