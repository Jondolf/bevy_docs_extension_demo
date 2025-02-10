use std::{
    collections::{HashMap, HashSet},
    fs::{File, read_to_string},
    path::PathBuf,
    process::Command,
    sync::LazyLock,
};

use rustdoc_types::{Crate, Item, ItemEnum, Span};

pub fn run(root_dir: &str, packages: Vec<String>) {
    // Generate documentation as json
    Command::new("cargo")
        .args(["doc", "--all-features", "--workspace", "--no-deps"])
        .env("RUSTDOCFLAGS", "-Z unstable-options --output-format json")
        .current_dir(root_dir)
        .status()
        .expect("Failed to run cargo doc in {root_dir}");

    for package in packages {
        add_info_to_package_source(root_dir, &package);
    }
}

fn add_info_to_package_source(root_dir: &str, package: &str) {
    let root_dir = PathBuf::from(root_dir);
    let file_name = root_dir.join(format!("target/doc/{package}.json"));
    let file = match File::open(&file_name) {
        Ok(file) => file,
        Err(e) => panic!("failed to open {file_name:?}: {e}"),
    };
    let crate_doc: Crate = serde_json::from_reader(file).unwrap();

    for (module_span, info) in info_for_modules(&crate_doc) {
        let source_file = root_dir.join(&module_span.filename);
        let src = read_to_string(&source_file).unwrap();
        let pos = position_in_string(&src, module_span.begin);

        // Insert data about implemented traits as a doc comment.
        // The id lets the ECMAScript find the data.
        let json = serde_json::to_string(&info).unwrap();
        let new_src = format!(
            r#"
            {}
            #![cfg_attr(docsrs_dep, doc = "<div id=\"bevy-traits-data\" style=\"display:none\">{}</div>")]
            {}
            "#,
            src[..pos].to_owned(),
            json.replace('"', "\\\""),
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
