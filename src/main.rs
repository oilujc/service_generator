use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use minijinja::{context, Environment};
use rust_embed::RustEmbed;
use std::fs;
use std::path::PathBuf;

// Indicamos a RustEmbed dónde están nuestras plantillas.
// Esta carpeta debe existir en la raíz de tu proyecto Rust.
#[derive(RustEmbed)]
#[folder = "src/templates/"]
struct Asset;

#[derive(Parser)]
#[command(name = "fastapi-cli")]
#[command(about = "Generador ultrarrápido de microservicios FastAPI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Crea la estructura base para un nuevo microservicio FastAPI
    New {
        /// Nombre del microservicio (ej. users-api)
        name: String,
    },
}

fn main() -> Result<()> {
    // Parseamos los argumentos de la terminal
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            println!(
                "{} Iniciando la creación del microservicio '{}'...",
                "🚀".cyan(),
                name.bold()
            );

            // Llamamos a la función principal y manejamos cualquier error
            if let Err(e) = create_microservice(&name) {
                eprintln!("{} Error fatal: {}", "❌".red(), e);
                return Err(e);
            }

            println!(
                "\n{} ¡Microservicio '{}' generado con éxito!",
                "✨".green(),
                name.bold()
            );
            println!("Siguientes pasos recomendados:");
            println!("  cd {}", name);
            println!("  python -m venv venv");
            println!("  source venv/bin/activate  # En Windows usa: venv\\Scripts\\activate");
            println!("  pip install -r requirements.txt");
        }
    }

    Ok(())
}

fn create_microservice(name: &str) -> Result<()> {
    let base_path = PathBuf::from(name);

    // 1. Prevención de sobreescritura
    if base_path.exists() {
        anyhow::bail!(
            "El directorio '{}' ya existe. Abortando para proteger tus archivos.",
            name
        );
    }

    // 2. Crear estructura de directorios
    let dirs_to_create = vec![
        base_path.join("app").join("routers"),
        base_path.join("app").join("agents"),
        base_path.join("app").join("agents").join("agents"),
        base_path.join("app").join("database"),
        base_path.join("app").join("models"),
        base_path.join("app").join("services"),
        base_path.join("app").join("utils"),
        base_path.join("tests"),
    ];

    for dir in dirs_to_create {
        fs::create_dir_all(&dir)
            .with_context(|| format!("Falló al crear el directorio {:?}", dir))?;
    }

    // 2.5. Crear archivos __init__.py para que sean paquetes Python
    let init_files = vec![
        base_path.join("app").join("__init__.py"),
        base_path.join("app").join("routers").join("__init__.py"),
        base_path.join("app").join("agents").join("__init__.py"),
        base_path.join("app").join("database").join("__init__.py"),
        base_path.join("app").join("models").join("__init__.py"),
        base_path.join("app").join("services").join("__init__.py"),
        base_path.join("app").join("utils").join("__init__.py"),
    ];

    for init_file in init_files {
        fs::write(&init_file, "")?;
        println!("  {} Creado {:?}", "✔".green(), init_file);
    }

    // 3. Configurar el motor de templates (MiniJinja)
    let mut env = Environment::new();

    // Cargamos iterativamente todos los archivos embebidos en el entorno de Jinja
    let template_files = vec![
        ("main.py.j2", "main.py.j2"),
        ("env.example.j2", "env.example.j2"),
        ("requirements.txt.j2", "requirements.txt.j2"),
        ("dependencies.py.j2", "dependencies.py.j2"),
    ];

    for (template_key, template_name) in template_files {
        if let Some(content) = Asset::get(template_name) {
            let template_str = std::str::from_utf8(content.data.as_ref())?.to_string();
            let owned_name: &'static str = Box::leak(template_key.to_string().into_boxed_str());
            let owned_source: &'static str = Box::leak(template_str.into_boxed_str());
            env.add_template(owned_name, owned_source)?;
        }
    }

    // 4. Mapeo de archivos a generar: (Nombre del template, Ruta de destino final)
    // Asegúrate de que estos archivos existan en tu carpeta /templates
    let files_to_generate = vec![
        ("main.py.j2", base_path.join("app").join("main.py")),
        ("env.example.j2", base_path.join(".env.example")),
        ("requirements.txt.j2", base_path.join("requirements.txt")),
        (
            "dependencies.py.j2",
            base_path.join("app").join("dependencies.py"),
        ),
        ("response_exception.py.j2", base_path.join("app").join("utils").join("response_exception.py")),
        ("agent_factory.py.j2", base_path.join("app").join("agents").join("agent_factory.py")),
        (
            "base_agent.py.j2",
            base_path.join("app").join("agents").join("agents").join("base_agent.py"),
        ),
        (
            "custom_agent.py.j2",
            base_path.join("app").join("agents").join("agents").join("custom_agent.py"),
        ),
        (
            "database.py.j2",
            base_path.join("app").join("database").join("database.py"),
        ),
    ];

    for (template_name, dest_path) in files_to_generate {
        // Obtenemos el template del motor
        let tmpl = env.get_template(template_name).with_context(|| {
            format!(
                "No se encontró el template '{}' en la carpeta embebida",
                template_name
            )
        })?;

        // Renderizamos inyectando la variable `project_name`
        let rendered = tmpl.render(context!(project_name => name))?;

        // Escribimos el resultado en el disco
        fs::write(&dest_path, rendered)
            .with_context(|| format!("Falló al escribir el archivo {:?}", dest_path))?;

        println!("  {} Creado {:?}", "✔".green(), dest_path);
    }

    Ok(())
}
