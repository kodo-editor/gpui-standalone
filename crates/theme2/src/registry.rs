use crate::{
    themes::{one_dark, rose_pine, rose_pine_dawn, rose_pine_moon, sandcastle},
    Theme, ThemeMetadata,
};
use anyhow::{anyhow, Result};
use gpui2::SharedString;
use std::{collections::HashMap, sync::Arc};

pub struct ThemeRegistry {
    themes: HashMap<SharedString, Arc<Theme>>,
}

impl ThemeRegistry {
    fn insert_themes(&mut self, themes: impl IntoIterator<Item = Theme>) {
        for theme in themes.into_iter() {
            self.themes
                .insert(theme.metadata.name.clone(), Arc::new(theme));
        }
    }

    pub fn list_names(&self, _staff: bool) -> impl Iterator<Item = SharedString> + '_ {
        self.themes.keys().cloned()
    }

    pub fn list(&self, _staff: bool) -> impl Iterator<Item = ThemeMetadata> + '_ {
        self.themes.values().map(|theme| theme.metadata.clone())
    }

    pub fn get(&self, name: &str) -> Result<Arc<Theme>> {
        self.themes
            .get(name)
            .ok_or_else(|| anyhow!("theme not found: {}", name))
            .cloned()
    }
}

impl Default for ThemeRegistry {
    fn default() -> Self {
        let mut this = Self {
            themes: HashMap::default(),
        };

        this.insert_themes([
            one_dark(),
            rose_pine(),
            rose_pine_dawn(),
            rose_pine_moon(),
            sandcastle(),
        ]);

        this
    }
}
