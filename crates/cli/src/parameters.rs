use domain::parameters;
use infrastructure::output::Output;
use std::io;

/// Execute the parameter-search command through the output pipeline.
pub fn search(output: &Output, query: &str) -> io::Result<()> {
    let result = parameters::parameter_search(query);
    output.success("parameter-search", &result, &mut io::stdout())
}

/// Execute the formulate-contradiction command through the output pipeline.
pub fn formulate(output: &Output, improving: &str, worsening: &str) -> io::Result<()> {
    let result = parameters::formulate_contradiction(improving, worsening);
    output.success("formulate-contradiction", &result, &mut io::stdout())
}
