mod proxy_module;
use proxy_module::{ProjectQuery, SecureCachedProjectProxy, UserRole};

fn main() {
    let mut secure_proxy = SecureCachedProjectProxy::new();

    println!("=== ESCENARIO 1: Intento de acceso de Estudiante ===");
    let p1 = secure_proxy.filter_projects_by_category("Desarrollo", &UserRole::Student);
    println!("Resultados recibidos: {:?}\n", p1);

    println!("=== ESCENARIO 2: Consulta inicial de Coordinador (Sin Caché) ===");
    let p2 = secure_proxy.filter_projects_by_category("Desarrollo", &UserRole::Coordinator);
    println!("Resultados desde MySQL: {:?}\n", p2);

    println!("=== ESCENARIO 3: Consulta repetida de Coordinador (Con Caché) ===");
    let p3 = secure_proxy.filter_projects_by_category("Desarrollo", &UserRole::Coordinator);
    println!("Resultados desde Caché: {:?}\n", p3);

    println!("=== ESCENARIO 4: Nueva categoría de Coordinador ===");
    let p4 = secure_proxy.filter_projects_by_category("Infraestructura", &UserRole::Coordinator);
    println!("Resultados desde MySQL: {:?}", p4);
}