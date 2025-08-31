use crate::{select::multi_select, style};

use super::{circom::Circom, halo2::Halo2, noir::Noir, ProvingSystem};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Adapter {
    Circom,
    Halo2,
    Noir,
    NoneOfTheAbove,
}

pub struct AdapterInfo {
    adapter: Adapter,
    str: &'static str,
}

const ADAPTERS_INFO: [AdapterInfo; 4] = [
    AdapterInfo {
        adapter: Adapter::Circom,
        str: "circom",
    },
    AdapterInfo {
        adapter: Adapter::Halo2,
        str: "halo2",
    },
    AdapterInfo {
        adapter: Adapter::Noir,
        str: "noir",
    },
    AdapterInfo {
        adapter: Adapter::NoneOfTheAbove,
        str: "none of the above",
    },
];

impl Adapter {
    pub fn as_str(&self) -> &'static str {
        ADAPTERS_INFO
            .iter()
            .find(|info| info.adapter == *self)
            .map(|info| info.str)
            .expect("Unsupported Adapter")
    }

    pub fn all_strings() -> Vec<&'static str> {
        ADAPTERS_INFO.iter().map(|info| info.str).collect()
    }

    pub fn from_idx(idx: usize) -> Self {
        ADAPTERS_INFO[idx].adapter
    }
}

pub struct AdapterSelector {
    pub adapters: Vec<Adapter>,
}

impl AdapterSelector {
    pub fn construct(selections: Vec<usize>) -> Self {
        let mut adapters: Vec<Adapter> = vec![];
        for s in selections {
            adapters.push(Adapter::from_idx(s));
        }
        Self { adapters }
    }

    pub fn select() -> Self {
        let adapters = multi_select(
            "Pick the adapters you want to use (multiple selection with space)",
            "No adapters selected. Use space to select an adapter or \"none of the above\" to skip",
            Adapter::all_strings(),
            vec![],
        );

        if adapters.contains(&(Adapter::NoneOfTheAbove as usize)) {
            style::print_yellow(
                "\"none of the above\" options apply, you can bring in additional Rust crates and define your own bindings to suit your needs.".to_string(),
            );
            return Self { adapters: vec![] };
        }

        Self {
            adapters: adapters
                .iter()
                .map(|&i| Adapter::from_idx(i))
                .collect::<Vec<Adapter>>(),
        }
    }

    pub fn dep_template(&self, cargo_toml_path: &str) {
        if self.contains(Adapter::Circom) {
            Circom::dep_template(cargo_toml_path).unwrap();
        }
        if self.contains(Adapter::Halo2) {
            Halo2::dep_template(cargo_toml_path).unwrap();
        }
        if self.contains(Adapter::Noir) {
            Noir::dep_template(cargo_toml_path).unwrap();
        }
    }

    pub fn build_dep_template(&self, cargo_toml_path: &str) {
        if self.contains(Adapter::Circom) {
            Circom::build_dep_template(cargo_toml_path).unwrap();
        }
        if self.contains(Adapter::Halo2) {
            Halo2::build_dep_template(cargo_toml_path).unwrap();
        }
        if self.contains(Adapter::Noir) {
            Noir::build_dep_template(cargo_toml_path).unwrap();
        }
    }

    pub fn lib_template(&self, lib_rs_path: &str) {
        if self.contains(Adapter::Circom) {
            Circom::lib_template(lib_rs_path).unwrap();
        }
        if self.contains(Adapter::Halo2) {
            Halo2::lib_template(lib_rs_path).unwrap();
        }
        if self.contains(Adapter::Noir) {
            Noir::lib_template(lib_rs_path).unwrap();
        }
    }

    pub fn mod_template(&self, lib_rs_path: &str) {
        if self.contains(Adapter::Circom) {
            Circom::mod_template(lib_rs_path).unwrap();
        }
        if self.contains(Adapter::Halo2) {
            Halo2::mod_template(lib_rs_path).unwrap();
        }
        if self.contains(Adapter::Noir) {
            Noir::mod_template(lib_rs_path).unwrap();
        }
    }

    pub fn build_template(&self, build_rs_path: &str) {
        if self.contains(Adapter::Circom) {
            Circom::build_template(build_rs_path).unwrap();
        }
        if self.contains(Adapter::Halo2) {
            Halo2::build_template(build_rs_path).unwrap();
        }
        if self.contains(Adapter::Noir) {
            Noir::build_template(build_rs_path).unwrap();
        }
    }

    pub fn contains(&self, adapter: Adapter) -> bool {
        self.adapters.contains(&adapter)
    }
}
