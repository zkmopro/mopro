use super::{circom::Circom, halo2::Halo2, noir::Noir};
use crate::init::proving_system::ProvingSystem;
use crate::{select::multi_select, style};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Adapter {
    Circom,
    Halo2,
    Noir,
    NoneOfTheAbove,
}

pub(super) const ADAPTERS: [Adapter; 4] = [
    Adapter::Circom,
    Adapter::Halo2,
    Adapter::Noir,
    Adapter::NoneOfTheAbove,
];

impl Adapter {
    pub fn as_str(&self) -> &'static str {
        match self {
            Adapter::Circom => "circom",
            Adapter::Halo2 => "halo2",
            Adapter::Noir => "noir",
            Adapter::NoneOfTheAbove => "none of the above",
        }
    }

    pub fn all_strings() -> Vec<&'static str> {
        ADAPTERS.iter().map(|info| info.as_str()).collect()
    }

    pub fn from_idx(idx: usize) -> Self {
        ADAPTERS[idx]
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

    pub fn dev_dep_template(&self, cargo_toml_path: &str) {
        if self.contains(Adapter::Circom) {
            Circom::dev_dep_template(cargo_toml_path).unwrap();
        }
        if self.contains(Adapter::Halo2) {
            Halo2::dev_dep_template(cargo_toml_path).unwrap();
        }
        if self.contains(Adapter::Noir) {
            Noir::dev_dep_template(cargo_toml_path).unwrap();
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

    pub fn test_template(&self, lib_rs_path: &str) {
        if self.contains(Adapter::Circom) {
            Circom::test_template(lib_rs_path).unwrap();
        }
        if self.contains(Adapter::Halo2) {
            Halo2::test_template(lib_rs_path).unwrap();
        }
        if self.contains(Adapter::Noir) {
            Noir::test_template(lib_rs_path).unwrap();
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
