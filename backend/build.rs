use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;

use cargo_metadata::{MetadataCommand, Package};

use bamboo_entities::prelude::DependencyDetails;

fn get_dependencies_from_cargo_lock(toml: String) -> Vec<Package> {
    let mut cmd = MetadataCommand::new();
    let metadata_command = cmd.manifest_path(toml);
    let metadata = metadata_command.exec().unwrap();

    metadata.packages
}

fn normalize(license_string: &str) -> String {
    let mut list: Vec<&str> = license_string
        .split('/')
        .flat_map(|e| e.split(" OR "))
        .map(str::trim)
        .collect();
    list.sort();
    list.dedup();
    list.join(" OR ")
}

fn main() {
    let frontend_dependencies = get_dependencies_from_cargo_lock(format!(
        "{}/../frontend/Cargo.toml",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    ));
    let backend_dependencies = get_dependencies_from_cargo_lock(format!(
        "{}/../Cargo.toml",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    ));
    let mut dependencies = BTreeMap::new();
    for dep in frontend_dependencies {
        if let Entry::Vacant(entry) = dependencies.entry(dep.name.clone()) {
            entry.insert(dep);
        }
    }
    for dep in backend_dependencies {
        if let Entry::Vacant(entry) = dependencies.entry(dep.name.clone()) {
            entry.insert(dep);
        }
    }

    let dependencies = dependencies.into_values();
    let mut dep_entities = Vec::new();
    for package in dependencies {
        let authors = if package.authors.is_empty() {
            None
        } else {
            Some(package.authors.clone().join(", "))
        };
        let description = package
            .description
            .clone()
            .map(|s| s.trim().replace('\n', " "));
        let license = package.license.as_ref().map(|s| normalize(s));
        let repository = package.repository.clone();
        let name = package.name.clone();

        dep_entities.push(DependencyDetails::new(
            authors.unwrap_or("".into()),
            name,
            repository.unwrap_or("".into()),
            license.unwrap_or("".into()),
            description.unwrap_or("".into()),
        ))
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dependencies.json");
    fs::write(
        dest_path,
        serde_json::to_string(&dep_entities).unwrap().as_bytes(),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../Cargo.toml");
    println!("cargo:rerun-if-changed=../Cargo.lock");
    println!("cargo:rerun-if-changed=../frontend/Cargo.toml");
    println!("cargo:rerun-if-changed=../frontend/Cargo.lock");
}
