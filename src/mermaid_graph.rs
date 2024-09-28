use crate::args::{Args};
use crate::graph::Graph;
use anyhow::{anyhow, Context, Error};
use cargo_metadata::{DependencyKind, Package, PackageId};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use semver::Version;
use std::collections::{HashMap, HashSet};

pub fn generate_mermaid_graph(args: &Args, graph: &Graph) -> Result<(), Error> {

    if args.duplicates {
        for (i, package) in find_duplicates(graph).iter().enumerate() {
            if i != 0 {
                println!();
            }

            let root = &graph.graph[graph.nodes[*package]];
            print_tree(graph, root, args.all);
        }
    } else {
        let root = match &args.package {
            Some(package) => find_package(package, graph)?,
            None => graph.root.as_ref().ok_or_else(|| {
                anyhow!("this command requires running against an actual package in this workspace")
            })?,
        };
        let root = &graph.graph[graph.nodes[root]];

        print_tree(graph, root, args.all);
    }

    Ok(())
}

fn find_package<'a>(package: &str, graph: &'a Graph) -> Result<&'a PackageId, Error> {
    let mut it = package.split(':');
    let name = it.next().unwrap();
    let version = it
        .next()
        .map(Version::parse)
        .transpose()
        .context("error parsing package version")?;

    let mut candidates = vec![];
    for idx in graph.graph.node_indices() {
        let package = &graph.graph[idx];
        if package.name != name {
            continue;
        }

        if let Some(version) = &version {
            if package.version != *version {
                continue;
            }
        }

        candidates.push(package);
    }

    if candidates.is_empty() {
        Err(anyhow!("no crates found for package `{}`", package))
    } else if candidates.len() > 1 {
        let specs = candidates
            .iter()
            .map(|p| format!("{}:{}", p.name, p.version))
            .collect::<Vec<_>>()
            .join(", ");
        Err(anyhow!(
            "multiple crates found for package `{}`: {}",
            package,
            specs,
        ))
    } else {
        Ok(&candidates[0].id)
    }
}

fn find_duplicates(graph: &Graph) -> Vec<&PackageId> {
    let mut packages = HashMap::new();

    for idx in graph.graph.node_indices() {
        let package = &graph.graph[idx];
        packages
            .entry(&package.name)
            .or_insert_with(Vec::new)
            .push(&package.id);
    }

    let mut duplicates = vec![];
    for ids in packages.values() {
        if ids.len() > 1 {
            duplicates.extend(ids.iter().cloned());
        }
    }

    duplicates.sort();
    duplicates
}

fn print_tree<'a>(
    graph: &'a Graph,
    root: &'a Package,
    all: bool,
) {
    let mut visited_deps = HashSet::new();
    let mut levels_continue = vec![];

    println!("");
    println!("flowchart TD");
    print_package(
        graph,
        root,
        all,
        &mut visited_deps,
        &mut levels_continue,
    );
}

fn print_package<'a>(
    graph: &'a Graph,
    package: &'a Package,
    all: bool,
    visited_deps: &mut HashSet<&'a PackageId>,
    levels_continue: &mut Vec<bool>,
) {
    let new = all || visited_deps.insert(&package.id);

    /**
    if let Some((last_continues, rest)) = levels_continue.split_last() {
        for continues in rest {
            let c = if *continues { symbols.down } else { " " };
            print!("{}   ", c);
        }

        let c = if *last_continues {
            symbols.tee
        } else {
            symbols.ell
        };
        print!("{0}{1}{1} ", c, symbols.right);
    }
    **/

    // println!("{:?}", package.name);

    if !new {
        return;
    }

    for kind in &[
        DependencyKind::Normal,
        // DependencyKind::Build,
        // DependencyKind::Development,
    ] {
        print_dependencies(
            graph,
            package,
            all,
            visited_deps,
            levels_continue,
            *kind,
        );
    }
}

fn print_dependencies<'a>(
    graph: &'a Graph,
    package: &'a Package,
    all: bool,
    visited_deps: &mut HashSet<&'a PackageId>,
    levels_continue: &mut Vec<bool>,
    kind: DependencyKind,
) {
    let idx = graph.nodes[&package.id];
    let mut deps = vec![];
    for edge in graph.graph.edges_directed(idx, Direction::Outgoing) {
        if *edge.weight() != kind {
            continue;
        }
        println!("  {} --> {}", &graph.graph[edge.source()].name, &graph.graph[edge.target()].name);
        let dep = &graph.graph[edge.target()];
        deps.push(dep);
    }

    if deps.is_empty() {
        return;
    }

    // ensure a consistent output ordering
    deps.sort_by_key(|p| &p.id);

    let name = match kind {
        DependencyKind::Normal => None,
        DependencyKind::Build => Some("[build-dependencies]"),
        DependencyKind::Development => Some("[dev-dependencies]"),
        _ => unreachable!(),
    };

    let mut it = deps.iter().peekable();
    while let Some(dependency) = it.next() {
        levels_continue.push(it.peek().is_some());
        print_package(
            graph,
            dependency,
            all,
            visited_deps,
            levels_continue,
        );
        levels_continue.pop();
    }
}