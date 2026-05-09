#  Implementación de Patrón de Diseño GoF: Proxy (Rust)

Este repositorio contiene la implementación práctica del patrón estructural **Proxy**, desarrollado para optimizar el acceso a datos en sistemas de gestión de proyectos.

##  Patrón Implementado: Proxy (Caché)

### Problema que resuelve
El filtrado de tablas de proyectos mediante consultas simuladas es costoso en tiempo (latencia de 2 segundos). Realizar consultas repetidas por la misma categoría degrada la experiencia del usuario y sobrecarga el sistema.

### Solución aplicada
Se utiliza un **Proxy de Caché**. Este intercepta la petición del cliente; si el resultado de la categoría ya existe en un `HashMap` interno, lo devuelve instantáneamente. Si no existe, delega la consulta al objeto real, obtiene los datos y los almacena para futuras solicitudes.

---

## 🛠️ Requisitos e Instalación

### Requisitos del Sistema
* **Lenguaje:** Rust (Edición 2021)
* **Versión mínima:** 1.70+
* **Herramientas:** Cargo y Chocolatey.

### Instalación de Herramientas (Vía Chocolatey)
```bash
choco install rust visualstudio2022-workload-vctools -y

---

Cómo ejecutar el proyecto
Siga estos pasos desde la terminal dentro de la carpeta raíz del proyecto (proyecto_proxy_rust):

1. Compilación
Para verificar que el código es correcto y preparar el binario:

Bash
cargo build


2. Ejecución de los ejemplos
Para correr la demostración interactiva que muestra el funcionamiento de la caché:

Bash
cargo run


Comportamiento esperado en consola:
Petición 1: Verá un mensaje de "Ejecutando consulta SQL pesada" con una pausa real de 2 segundos.

Petición 2 (Misma categoría): Verá un mensaje de "Retornando resultados desde la CACHE" de forma inmediata, demostrando que el patrón Proxy resolvió el problema de latencia.


---

Organización del Código
src/proxy_module.rs: Contiene la definición del Trait, el objeto Real y la lógica del Proxy.

src/main.rs: Punto de entrada que simula las peticiones del cliente.

.gitignore: Configurado para excluir la carpeta target/ y mantener el repositorio limpio.