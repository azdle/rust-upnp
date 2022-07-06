/*!
What's this all about then?
*/

use crate::description::xml::*;
use crate::utils::xml::*;
use crate::SpecVersion;
use quick_xml::{Error, Writer};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Direction {
    In,
    Out,
}

#[derive(Clone, Debug)]
pub struct Argument {
    pub name: String,
    pub direction: Direction,
    pub return_value: bool,
    pub related_state_variable: String,
}

#[derive(Clone, Debug)]
pub struct Action {
    pub name: String,
    pub argument_list: Vec<Argument>,
}

#[derive(Clone, Debug)]
pub enum AllowedValue {
    List {
        values: Vec<String>,
    },
    Range {
        minimum: String,
        maximum: String,
        step: Option<String>,
    },
}

#[derive(Clone, Debug)]
pub struct StateVariable {
    pub send_events: bool,
    pub name: String,
    pub data_type: String,
    pub default_value: Option<String>,
    pub allowed_values: Option<AllowedValue>,
}

#[derive(Clone, Debug)]
pub struct Spcd {
    pub spec_version: SpecVersion,
    pub action_list: Vec<Action>,
    pub service_state_table: Vec<StateVariable>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn to_writer<T: Write>(root: &Spcd, writer: T) -> Result<(), quick_xml::Error> {
    let mut xml = Writer::new(writer);

    start(&mut xml)?;

    root.write(&mut xml)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<T: Write> Writable<T> for Argument {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error> {
        let argument = start_element(writer, X_ELEM_ARGUMENT)?;

        text_element(writer, X_ELEM_NAME, &self.name.as_bytes())?;

        text_element(
            writer,
            X_ELEM_DIRECTION,
            match &self.direction {
                Direction::In => "in".as_bytes(),
                Direction::Out => "out".as_bytes(),
            },
        )?;

        if self.return_value {
            element(writer, X_ELEM_RETVAL)?;
        }

        text_element(
            writer,
            X_ELEM_REL_STATE_VARIABLE,
            &self.related_state_variable.as_bytes(),
        )?;

        argument.end(writer)
    }
}

impl<T: Write> Writable<T> for Action {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error> {
        let action = start_element(writer, X_ELEM_ACTION)?;

        text_element(writer, X_ELEM_NAME, &self.name.as_bytes())?;

        if !&self.argument_list.is_empty() {
            let list = start_element(writer, X_ELEM_ARGUMENT_LIST)?;
            for argument in &self.argument_list {
                argument.write(writer)?;
            }
            list.end(writer)?;
        }

        action.end(writer)
    }
}

impl<T: Write> Writable<T> for AllowedValue {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error> {
        match self {
            AllowedValue::List { values } => {
                let list = start_element(writer, X_ELEM_ALLOWED_LIST)?;
                for value in values {
                    text_element(writer, X_ELEM_ALLOWED_VALUE, value.as_bytes())?;
                }
                list.end(writer)
            }
            AllowedValue::Range {
                minimum,
                maximum,
                step,
            } => {
                let range = start_element(writer, X_ELEM_ALLOWED_RANGE)?;

                text_element(writer, X_ELEM_MINIMUM, minimum.as_bytes())?;

                text_element(writer, X_ELEM_MAXIMUM, maximum.as_bytes())?;

                if let Some(step) = step {
                    text_element(writer, X_ELEM_STEP, step.as_bytes())?;
                }
                range.end(writer)
            }
        }
    }
}

impl<T: Write> Writable<T> for StateVariable {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error> {
        let variable = start_element_with(
            writer,
            X_ELEM_STATE_VARIABLE,
            vec![(
                X_ATTR_SEND_EVENTS,
                if self.send_events { "yes" } else { "no" },
            )],
        )?;

        text_element(writer, X_ELEM_NAME, &self.name.as_bytes())?;

        text_element(writer, X_ELEM_DATA_TYPE, &self.data_type.as_bytes())?;

        if let Some(default_value) = &self.default_value {
            text_element(writer, X_ELEM_DEFAULT_VALUE, default_value.as_bytes())?;
        }

        if let Some(allowed) = &self.allowed_values {
            allowed.write(writer)?;
        }

        variable.end(writer)
    }
}

impl<T: Write> Writable<T> for Spcd {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error> {
        let root = start_ns_element(writer, X_ELEM_SPCD, X_NS_SERVICE, None)?;

        &self.spec_version.write(writer)?;

        if !&self.action_list.is_empty() {
            let list = start_element(writer, X_ELEM_ACTION_LIST)?;
            for action in &self.action_list {
                action.write(writer)?;
            }
            list.end(writer)?;
        }

        let list = start_element(writer, X_ELEM_STATE_TABLE)?;
        for variable in &self.service_state_table {
            variable.write(writer)?;
        }
        list.end(writer)?;

        root.end(writer)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------
