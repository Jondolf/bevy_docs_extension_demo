use std::{
    collections::{HashMap, HashSet},
    fs::{read_to_string, File},
    path::PathBuf,
    process::Command,
    sync::LazyLock,
};

use rustdoc_types::{Crate, Item, ItemEnum, Span};

fn main() {
    // Temporary entrypoint, to be replaced if this is integrated in the CI tool
    let args = std::env::args().collect::<Vec<_>>();
    assert!(args.len() <= 2);
    let root_dir = args.get(1).map_or(".", |s| s);

    run(root_dir, package_names(root_dir));
}

fn run(root_dir: &str, packages: Vec<String>) {
    // Generate documentation as json
    Command::new("cargo")
        .args(["doc", "--all-features", "--workspace"])
        .env("RUSTDOCFLAGS", "-Z unstable-options --output-format json")
        .current_dir(root_dir)
        .status()
        .expect("Failed to run cargo doc in {root_dir}");

    for package in packages {
        add_info_to_package_source(root_dir, &package);
    }
}

// Can be removed when this gets run by Bevy's publish.sh,
// as that knows the precise list of crates to publish.
fn package_names(root_dir: &str) -> Vec<String> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .current_dir(root_dir)
        .output()
        .unwrap();
    if !output.status.success() {
        panic!("Failed to read manifest");
    }
    let manifest = serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap();
    manifest
        .get("packages")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|package| package.get("name").unwrap().as_str().unwrap().to_owned())
        .collect()
}

fn add_info_to_package_source(root_dir: &str, package: &str) {
    let root_dir = PathBuf::from(root_dir);
    let file_name = root_dir.join(format!("target/doc/{package}.json"));
    let file = match File::open(&file_name) {
        Ok(file) => file,
        Err(e) => {
            // When we have a precise list of crates, this can just be turned into a panic
            eprintln!("failed to open {file_name:?}: {e}");
            return;
        }
    };
    let crate_doc: Crate = serde_json::from_reader(file).unwrap();
    assert_eq!(crate_doc.format_version, 39);

    for (module_span, info) in info_for_modules(&crate_doc) {
        let source_file = root_dir.join(&module_span.filename);
        let src = read_to_string(&source_file).unwrap();
        let pos = position_in_string(&src, module_span.begin);

        // Insert data about implemented traits as a doc comment.
        // The cfg prevents this from showing up in rust-analyzer,
        // the css prevents this from being visible on the web (including to screen readers),
        // and the id lets the ECMAScript find the data.
        let new_src = format!(
            r#"
            {}
            #![cfg_attr(doc, doc = "<div id=\"bevy-traits-data\" style=\"display:none\">{}</div>")]
            {}
            "#,
            src[..pos].to_owned(),
            serde_json::to_string(&info).unwrap().replace('"', "\\\""),
            &src[pos..]
        );

        std::fs::write(source_file, new_src).unwrap();
    }
}

/// Find the byte index into the string.
/// Line is 1-based, column is 0-based.
fn position_in_string(str: &str, (line, column): (usize, usize)) -> usize {
    let mut pos = 0;
    for _ in 1..line {
        pos += str[pos..].find('\n').unwrap();
    }
    pos + column
}

type BevyTraits<'a> = Vec<&'a str>;
type ModuleInfo<'a> = HashMap<&'a str, BevyTraits<'a>>;

fn info_for_modules(crate_doc: &Crate) -> Vec<(&Span, ModuleInfo)> {
    let mut modules = Vec::new();
    for item in crate_doc.index.values() {
        let ItemEnum::Module(module) = &item.inner else {
            continue;
        };
        let Some(mod_span) = &item.span else { continue };
        let items: ModuleInfo = module
            .items
            .iter()
            .flat_map(|id| {
                let item = &crate_doc.index[id];
                let bevy_traits = bevy_traits_for_item(crate_doc, item);
                (!bevy_traits.is_empty()).then(|| (item.name.as_deref().unwrap(), bevy_traits))
            })
            .collect();
        if !items.is_empty() {
            modules.push((mod_span, items));
        }
    }
    modules
}

fn bevy_traits_for_item<'a>(crate_doc: &'a Crate, item: &Item) -> BevyTraits<'a> {
    let impls = match &item.inner {
        ItemEnum::Struct(def) => &def.impls,
        ItemEnum::Enum(def) => &def.impls,
        ItemEnum::Union(def) => &def.impls,
        _ => return Vec::new(),
    };
    let mut traits = Vec::new();
    for impl_id in impls {
        let ItemEnum::Impl(impl_block) = &crate_doc.index[impl_id].inner else {
            panic!()
        };
        if let Some(trait_) = &impl_block.trait_ {
            let trait_name = trait_.path.as_str();
            if BEVY_TRAITS.contains(trait_name) {
                traits.push(trait_name);
            }
        }
    }
    traits
}

static BEVY_TRAITS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    [
        "Plugin",
        "PluginGroup",
        "Component",
        "Resource",
        "Asset",
        "Event",
        "ScheduleLabel",
        "SystemSet",
        "SystemParam",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
});
