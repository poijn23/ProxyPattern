mod proxy_module;
use proxy_module::{ProjectQuery, CachedProjectProxy};

fn main() {
    let mut service_proxy = CachedProjectProxy::new();
    
    println!("Solicitud 1 (Desarrollo):");
    service_proxy.filter_projects_by_category("Desarrollo");
    
    println!("\nSolicitud 2 (Desarrollo - Repetida):");
    service_proxy.filter_projects_by_category("Desarrollo");

    println!("\nSolicitud 3 (Infraestructura):");
    service_proxy.filter_projects_by_category("Infraestructura");
}