# Granco Planner (Native Rust Edition)

**Granco Planner** es una aplicación de escritorio 100% nativa escrita en **Rust** (`eframe`/`egui` + `rusqlite`) para la planificación de servicios de alabanza, series de sermones, órdenes de culto (Run Sheets), cancionero general con múltiples arreglos y estadísticas de rotación de canciones.

## Características Principales

- **Planes de Servicio (`Services`)**: Agrupación automática por Serie de Sermones o Servicios Independientes, tarjetas con portada gráfica, subtítulos de serie/semana/tema y fecha/hora localizada.
- **Calendario Mensual (`Calendar`)**: Vista mensual interactiva con navegación por meses, indicador del día actual, creación rápida de servicios en cualquier fecha y vista previa de servicios programados.
- **Series de Sermones (`Series`)**: Gestión de series con título, descripción y carga de portada gráfica (mediante selector de archivos nativo o arrastrar y soltar) con extracción automática de color ambiental.
- **Constructor de Orden de Culto (`Service Builder / Run Sheet`)**:
  - Cálculo automático de horas de inicio acumuladas, duración total y hora estimada de finalización.
  - Elementos codificados por color según su tipo: Canción (`Song`), Conferencia (`Conference`), Video (`Video`), Loop (`Loop`), Anuncios (`Announcements`) y Cuenta Regresiva (`Countdown`).
  - Reordenamiento rápido, edición en línea de Notas/Cues técnicos y Voz Principal/Conferencista.
  - **Imprimir Orden de Culto**: Genera una hoja de servicio monocromática (1 tinta) de alto contraste lista para impresión.
  - **Guardar Captura PNG**: Exporta la orden de culto en formato imagen PNG.
- **Cancionero General (`Song Bank`)**: Biblioteca maestra de canciones con soporte para múltiples arreglos por canción, tonalidades musicales (`C`, `Db`, `D`, ..., `Gm`), BPM y duración.
- **Estadísticas y Rotación (`Analytics`)**: Tarjetas KPI (Total de Reuniones, Canciones Programadas, Canciones Únicas, Canción Más Cantada, Tono Más Frecuente), filtros por periodo (Todo el Historial, Últimos 90 días, Este Año), ranking de canciones más usadas y detección de canciones sin rotación reciente.
- **Configuración (`Settings`)**: Modo Oscuro, Modo Claro o Sistema, e interfaz bilingüe completa (**Español** / **English**) con persistencia local en SQLite.

## Compilación y Ejecución en Desarrollo

```bash
# Ejecutar pruebas unitarias
cargo test

# Ejecutar la aplicación en modo desarrollo
cargo run

# Compilar binario optimizado de producción
cargo build --release
```

## Generación de Instaladores

### Linux (`.deb`, `.rpm`, `.sh`, `.tar.gz`)

```bash
chmod +x packaging/linux/build-linux-installers.sh
./packaging/linux/build-linux-installers.sh
```

Los paquetes instaladores se generan en el directorio `release/`:
- `release/granco-planner_1.0.0_amd64.deb` (Debian / Ubuntu / Linux Mint)
- `release/granco-planner-1.0.0-1.x86_64.rpm` (Fedora / RHEL / openSUSE)
- `release/granco-planner-1.0.0-linux-x86_64-installer.sh` (Instalador autoextraíble universal)
- `release/granco-planner-1.0.0-linux-x86_64.tar.gz` (Archivo portátil)

### Windows (`Granco-Planner-Setup-1.0.0-x64.exe`)

En Windows (con Rust `x86_64-pc-windows-msvc` y NSIS instalados) o automáticamente mediante el flujo de **GitHub Actions** ([`.github/workflows/release.yml`](.github/workflows/release.yml)):

```powershell
cargo build --release --target x86_64-pc-windows-msvc
makensis packaging\windows\installer.nsi
```
