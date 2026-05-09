use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

pub trait ProjectQuery {
    fn filter_projects_by_category(&mut self, category: &str) -> Vec<String>;
}

pub struct RealDatabase;

impl ProjectQuery for RealDatabase {
    fn filter_projects_by_category(&mut self, category: &str) -> Vec<String> {
        println!("--- Ejecutando consulta SQL pesada para categoria: {} ---", category);
        sleep(Duration::from_secs(2));
        
        match category {
            "Desarrollo" => vec!["Sistema ERP".to_string(), "App Movil".to_string()],
            "Infraestructura" => vec!["Migracion Cloud".to_string(), "Setup Servidores".to_string()],
            _ => vec!["Proyecto Generico".to_string()],
        }
    }
}

pub struct CachedProjectProxy {
    real_database: RealDatabase,
    projects_cache: HashMap<String, Vec<String>>,
}

impl CachedProjectProxy {
    pub fn new() -> Self {
        Self {
            real_database: RealDatabase,
            projects_cache: HashMap::new(),
        }
    }
}

impl ProjectQuery for CachedProjectProxy {
    fn filter_projects_by_category(&mut self, category: &str) -> Vec<String> {
        if let Some(cached_projects) = self.projects_cache.get(category) {
            println!(">>> Retornando resultados desde la CACHE para: {} <<<", category);
            return cached_projects.clone();
        }

        let projects = self.real_database.filter_projects_by_category(category);
        self.projects_cache.insert(category.to_string(), projects.clone());
        projects
    }
}