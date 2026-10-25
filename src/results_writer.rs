/// this file was vibecoded
///
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

// Adjust this path to wherever SimulationResults lives in your crate.
use crate::simulation_results::SimulationResults;

/// Writes `SimulationResults` to a human-readable .txt file,
/// optionally matching node ids to page names.
pub struct ResultsWriter {
    path: PathBuf,
    names: Option<HashMap<u32, String>>,
}

impl ResultsWriter {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            names: None,
        }
    }

    #[allow(dead_code)]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Loads node names from a file with lines of the form `<id> <name>`,
    /// e.g. "wiki-topcats-page-names.txt". Without calling this, no names are written.
    pub fn with_names(mut self, names_path: impl AsRef<Path>) -> io::Result<Self> {
        self.names = Some(load_names(names_path)?);
        Ok(self)
    }

    /// Uses an already-loaded name map (handy if you load it once and reuse it).
    #[allow(dead_code)]
    pub fn with_name_map(mut self, names: HashMap<u32, String>) -> Self {
        self.names = Some(names);
        self
    }

    /// Creates (or overwrites) the file at `self.path` and writes the results into it.
    pub fn write(&self, results: &SimulationResults) -> io::Result<()> {
        let file = File::create(&self.path)?;
        let mut out = BufWriter::new(file);
        self.write_to(&mut out, results)?;
        out.flush()
    }

    /// Writes the results to any `Write` target (file, stdout, Vec<u8> in tests, ...).
    pub fn write_to<W: Write>(&self, out: &mut W, results: &SimulationResults) -> io::Result<()> {
        writeln!(out, "=== Simulation Results ===")?;
        writeln!(out)?;

        // --- Summary ---
        writeln!(out, "Nodes:                    {}", opt(results.nodes))?;
        writeln!(out, "Threads:                  {}", opt(results.threads))?;
        writeln!(out, "Total steps:              {}", results.total_steps)?;
        writeln!(
            out,
            "Steps per thread:         {}",
            opt(results.steps_per_thread)
        )?;
        writeln!(
            out,
            "Step division remainder:  {}",
            opt(results.step_division_remainder)
        )?;
        writeln!(
            out,
            "Execution time:           {}",
            results
                .execution_time
                .map(|d| format!("{:.3?}", d))
                .unwrap_or_else(|| "n/a".to_string())
        )?;
        writeln!(out, "Distinct nodes recorded:  {}", results.visits.len())?;
        writeln!(out)?;

        // --- Ranking ---
        let show_names = self.names.is_some();

        write!(
            out,
            "{:>6}  {:>10}  {:>14}  {:>12}",
            "Rank", "Node", "Visits", "Score"
        )?;
        if show_names {
            write!(out, "  Name")?;
        }
        writeln!(out)?;

        let separator_len = 6 + 2 + 10 + 2 + 14 + 2 + 12 + if show_names { 6 } else { 0 };
        writeln!(out, "{}", "-".repeat(separator_len))?;

        for (rank, (node, score)) in results.normalized().iter().enumerate() {
            let visits = results.visits.get(node).copied().unwrap_or(0);
            write!(
                out,
                "{:>6}  {:>10}  {:>14}  {:>12.8}",
                rank + 1,
                node,
                visits,
                score
            )?;

            if let Some(names) = &self.names {
                let name = names.get(node).map(String::as_str).unwrap_or("<unknown>");
                write!(out, "  {}", name)?;
            }
            writeln!(out)?;
        }

        Ok(())
    }
}

/// Parses a names file where each line is `<id> <name>`.
/// Invalid UTF-8 is replaced rather than failing, and malformed lines are skipped.
pub fn load_names(path: impl AsRef<Path>) -> io::Result<HashMap<u32, String>> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut names = HashMap::new();
    let mut buf = Vec::new();

    loop {
        buf.clear();
        if reader.read_until(b'\n', &mut buf)? == 0 {
            break;
        }

        let line = String::from_utf8_lossy(&buf);
        let line = line.trim_end_matches(['\r', '\n']);
        if line.is_empty() {
            continue;
        }

        let (id, name) = match line.split_once(' ') {
            Some((id, name)) => (id, name.trim()),
            None => (line, ""), // id with no name
        };

        if let Ok(id) = id.trim().parse::<u32>() {
            names.insert(id, name.to_string());
        }
    }

    Ok(names)
}

fn opt<T: Display>(value: Option<T>) -> String {
    value
        .map(|v| v.to_string())
        .unwrap_or_else(|| "n/a".to_string())
}
