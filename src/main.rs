use color_eyre::{
    self,
    eyre::{Report, Result},
};
use graph_ext::{Elem, GraphExt};
use graphviz_rust as gv;
use gv::{
    cmd::{CommandArg, Format, Layout},
    printer::PrinterContext,
};

mod graph_ext;
mod xdot;

const TEST: &'static str = "graph { a -- b }";

fn main() -> Result<()> {
    color_eyre::install()?;
    let graph = gv::parse(TEST).map_err(Report::msg)?;
    let mut ctx = PrinterContext::default();
    let layed_out = gv::exec(
        graph,
        &mut ctx,
        vec![
            CommandArg::Layout(Layout::Dot),
            CommandArg::Format(Format::Xdot),
        ],
    )?;
    let graph = gv::parse(&layed_out).map_err(Report::msg)?;
    graph.iter_elems().for_each(handle_elem);
    Ok(())
}

fn handle_elem(elem: Elem) {
    match elem {
        Elem::Edge(edge) => {
            dbg!(edge);
        }
        Elem::Node(node) => {
            dbg!(node);
        }
    }
}
