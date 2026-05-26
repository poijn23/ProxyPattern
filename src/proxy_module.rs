use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use mysql::*;
use mysql::prelude::*;

#[derive(Clone, PartialEq)]
pub enum UserRole {
    Student,
    Coordinator,
}

pub trait ProjectQuery {
    fn filter_projects_by_category(&mut self, category: &str, role: &UserRole) -> Vec<String>;
}

pub struct RealDatabase {
    connection_pool: Pool,
}

impl RealDatabase {
    pub fn new() -> Self {
        let database_url = "mysql://fei_user:Fei_Password_2026@127.0.0.1:3306/ffei_projects_db";
        let pool = Pool::new(database_url).unwrap();
        Self { connection_pool: pool }
    }
}

impl ProjectQuery for RealDatabase {
    fn filter_projects_by_category(&mut self, category: &str, _role: &UserRole) -> Vec<String> {
        println!("--- [SQL] Conectando y ejecutando Query pesada en MySQL para: {} ---", category);
        sleep(Duration::from_secs(2));

        let mut conn = self.connection_pool.get_conn().unwrap();
        
        let query_result = conn.exec_map(
            "SELECT name FROM projects WHERE category = :category",
            params! { "category" => category },
            |name: String| name,
        ).unwrap();

        query_result
    }
}

pub struct SecureCachedProjectProxy {
    real_database: RealDatabase,
    projects_cache: HashMap<String, Vec<String>>,
}

impl SecureCachedProjectProxy {
    pub fn new() -> Self {
        Self {
            real_database: RealDatabase::new(),
            projects_cache: HashMap::new(),
        }
    }
}

impl ProjectQuery for SecureCachedProjectProxy {
    fn filter_projects_by_category(&mut self, category: &str, role: &UserRole) -> Vec<String> {
        match role {
            UserRole::Student => {
                println!("[🔒 SEGURIDAD] Acceso Denegado: Los estudiantes no tienen permisos para consultar la tabla de proyectos.");
                return vec![];
            }
            UserRole::Coordinator => {
                println!("[🔓 SEGURIDAD] Acceso Concedido para rol Coordinador.");
            }
        }

        if let Some(cached_projects) = self.projects_cache.get(category) {
            println!(">>> [CACHÉ] Retornando resultados desde la memoria interna para: {} <<<", category);
            return cached_projects.clone();
        }

        let projects = self.real_database.filter_projects_by_category(category, role);
        self.projects_cache.insert(category.to_string(), projects.clone());
        projects
    }
}