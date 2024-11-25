use clap::{Subcommand, ValueEnum};
use objio::{ObjectReader, ObjectWriter};
use rdftk_core::{
    error::Error,
    model::{data_set::DataSet, graph::Graph},
};
use rdftk_io::{json, nq, nt, trig, turtle, xml};
use std::process::ExitCode;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub(crate) trait Command {
    fn execute(self) -> Result<ExitCode, Error>;
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    Convert(ConvertCmd),
    Draw(DrawCmd),
    Namespace(NamespaceCmd),
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Representation {
    Json,
    JsonLd,
    N3,
    NQuads,
    NTriples,
    Trig,
    Turtle,
    Xml,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Command for Commands {
    fn execute(self) -> Result<ExitCode, Error> {
        match self {
            Commands::Namespace(cmd) => cmd.execute(),
            Commands::Convert(cmd) => cmd.execute(),
            Commands::Draw(cmd) => cmd.execute(),
        }
    }
}

impl Representation {
    pub(crate) fn is_dataset_representation(&self) -> bool {
        matches!(self, Self::NQuads | Self::Trig)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn read_graph(
    input: &mut clio::Input,
    from_repr: Representation,
) -> Result<Graph, Error> {
    let graph = match from_repr {
        Representation::Json => {
            let reader = json::JsonReader::default();
            reader.read(input)?
        }
        Representation::JsonLd => todo!(),
        Representation::N3 => todo!(),
        Representation::NTriples => {
            let reader = nt::NTripleReader::default();
            reader.read(input)?
        }
        Representation::Turtle => {
            let reader = turtle::TurtleReader::default();
            reader.read(input)?
        }
        Representation::Xml => {
            let reader = xml::XmlReader::default();
            reader.read(input)?
        }
        Representation::NQuads | Representation::Trig => unreachable!(),
    };
    Ok(graph)
}

pub(crate) fn read_dataset(
    input: &mut clio::Input,
    from_repr: Representation,
) -> Result<DataSet, Error> {
    let graph = match from_repr {
        Representation::NQuads => {
            let reader = nq::NQuadReader::default();
            reader.read(input)?
        }
        Representation::Trig => {
            let reader = trig::TrigReader::default();
            reader.read(input)?
        }
        Representation::Json
        | Representation::JsonLd
        | Representation::N3
        | Representation::NTriples
        | Representation::Turtle
        | Representation::Xml => unreachable!(),
    };
    Ok(graph)
}

pub(crate) fn write_graph(
    graph: &Graph,
    output: &mut clio::Output,
    to_repr: Representation,
) -> Result<(), Error> {
    match to_repr {
        Representation::Json => {
            let writer = json::JsonWriter::default();
            writer.write(output, graph)?;
        }
        Representation::JsonLd => todo!(),
        Representation::N3 => todo!(),
        Representation::NQuads => {
            let writer = nq::NQuadWriter::default();
            writer.write(output, graph)?;
        }
        Representation::NTriples => {
            let writer = nt::NTripleWriter::default();
            writer.write(output, graph)?;
        }
        Representation::Trig => {
            let writer = trig::TrigWriter::default();
            writer.write(output, graph)?;
        }
        Representation::Turtle => {
            let writer = turtle::TurtleWriter::default();
            writer.write(output, graph)?;
        }
        Representation::Xml => {
            let writer = xml::XmlWriter::default();
            writer.write(output, graph)?;
        }
    };
    Ok(())
}

pub(crate) fn write_dataset(
    dataset: &DataSet,
    output: &mut clio::Output,
    to_repr: Representation,
) -> Result<(), Error> {
    match to_repr {
        Representation::NQuads => {
            let writer = nq::NQuadWriter::default();
            writer.write(output, dataset)?;
        }
        Representation::Trig => {
            let writer = trig::TrigWriter::default();
            writer.write(output, dataset)?;
        }
        Representation::Json
        | Representation::JsonLd
        | Representation::N3
        | Representation::NTriples
        | Representation::Turtle
        | Representation::Xml => unreachable!(),
    };
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod namespaces;
use namespaces::NamespaceCmd;

mod representations;
use representations::ConvertCmd;

mod queries;

mod draw;
use draw::DrawCmd;
