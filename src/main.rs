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
        /// Incluir archivos Docker (Dockerfile, docker-compose.yml)
        #[arg(long)]
        with_docker: bool,
        /// Incluir servicio MongoDB en docker-compose.yml
        #[arg(long)]
        with_db: bool,
    },
}

fn main() -> Result<()> {
    // Parseamos los argumentos de la terminal
    let cli = Cli::parse();

    match &cli.command {
        Commands::New {
            name,
            with_docker,
            with_db,
        } => {
            println!(
                "{} Iniciando la creación del microservicio '{}'...",
                "🚀".cyan(),
                name.bold()
            );

            if let Err(e) = create_microservice(&name, *with_docker, *with_db) {
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

fn create_microservice(name: &str, with_docker: bool, with_db: bool) -> Result<()> {
    let base_path = PathBuf::from(name);

    let include_mongodb = with_db;

    // 1. Prevención de sobreescritura
    if base_path.exists() {
        anyhow::bail!(
            "El directorio '{}' ya existe. Abortando para proteger tus archivos.",
            name
        );
    }

    // 2. Crear estructura de directorios
    let dirs_to_create = vec![
        base_path.join(".agents"),
        base_path.join(".agents").join("skills"),
        base_path.join("app").join("routers"),
        base_path.join("app").join("agents"),
        base_path.join("app").join("agents").join("agents"),
        base_path.join("app").join("database"),
        base_path.join("app").join("database").join("repositories"),
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
        base_path
            .join("app")
            .join("agents")
            .join("agents")
            .join("__init__.py"),
        base_path.join("app").join("database").join("__init__.py"),
        base_path
            .join("app")
            .join("database")
            .join("repositories")
            .join("__init__.py"),
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

    let mut template_files = vec![
        ("main.py.j2", "main.py.j2"),
        ("env.example.j2", "env.example.j2"),
        ("requirements.txt.j2", "requirements.txt.j2"),
        ("dependencies.py.j2", "dependencies.py.j2"),
        ("gitignore.j2", "gitignore.j2"),
        ("response_exception.py.j2", "response_exception.py.j2"),
        ("agent_factory.py.j2", "agent_factory.py.j2"),
        ("base_agent.py.j2", "base_agent.py.j2"),
        ("custom_agent.py.j2", "custom_agent.py.j2"),
        ("database.py.j2", "database.py.j2"),
        ("base_repository.py.j2", "base_repository.py.j2"),
    ];

    if with_docker {
        template_files.push(("Dockerfile.j2", "Dockerfile.j2"));
        template_files.push(("docker-compose.yml.j2", "docker-compose.yml.j2"));
    }

    for (template_key, template_name) in template_files {
        if let Some(content) = Asset::get(template_name) {
            let template_str = std::str::from_utf8(content.data.as_ref())?.to_string();
            let owned_name: &'static str = Box::leak(template_key.to_string().into_boxed_str());
            let owned_source: &'static str = Box::leak(template_str.into_boxed_str());
            env.add_template(owned_name, owned_source)?;
        }
    }

    // 3.5. Copiar skills al directorio .agents/skills
    let skills_to_copy = vec!["agno-agent-creator", "fastapi-mongo-skill"];

    for skill_name in skills_to_copy {
        let skill_path = base_path.join(".agents").join("skills").join(skill_name);

        fs::create_dir_all(&skill_path)
            .with_context(|| format!("Falló al crear el directorio {:?}", skill_path))?;

        // Copiar SKILL.md (quitando extensión .j2)
        let skill_md = format!("{}/SKILL.md.j2", skill_name);
        if let Some(content) = Asset::get(&skill_md) {
            let dest = skill_path.join("SKILL.md");
            fs::write(&dest, content.data.as_ref())?;
            println!("  {} Copiado {:?}", "✔".green(), dest);
        }

        // Copiar subdirectorios (assets, references, scripts)
        for subdir in &["assets", "references", "scripts"] {
            let subdir_path = skill_path.join(subdir);
            fs::create_dir_all(&subdir_path)?;

            let subdir_base = format!("{}/{}", skill_name, subdir);
            for item in Asset::iter() {
                if item.starts_with(&subdir_base) {
                    let file_name = item
                        .strip_prefix(&format!("{}/", subdir_base))
                        .unwrap_or(&item);
                    if !file_name.is_empty() {
                        if let Some(content) = Asset::get(item.as_ref()) {
                            // Quitar extensión .j2 del nombre de archivo
                            let dest_name = file_name.trim_end_matches(".j2");
                            let dest = subdir_path.join(dest_name);
                            fs::write(&dest, content.data.as_ref())?;
                            println!("  {} Copiado {:?}", "✔".green(), dest);
                        }
                    }
                }
            }
        }
    }

    // 4. Mapeo de archivos a generar: (Nombre del template, Ruta de destino final)
    // Asegúrate de que estos archivos existan en tu carpeta /templates
    let mut files_to_generate = vec![
        ("main.py.j2", base_path.join("app").join("main.py")),
        ("env.example.j2", base_path.join(".env.example")),
        ("gitignore.j2", base_path.join(".gitignore")),
        ("requirements.txt.j2", base_path.join("requirements.txt")),
        (
            "dependencies.py.j2",
            base_path.join("app").join("dependencies.py"),
        ),
        (
            "response_exception.py.j2",
            base_path
                .join("app")
                .join("utils")
                .join("response_exception.py"),
        ),
        (
            "agent_factory.py.j2",
            base_path
                .join("app")
                .join("agents")
                .join("agent_factory.py"),
        ),
        (
            "base_agent.py.j2",
            base_path
                .join("app")
                .join("agents")
                .join("agents")
                .join("base_agent.py"),
        ),
        (
            "custom_agent.py.j2",
            base_path
                .join("app")
                .join("agents")
                .join("agents")
                .join("custom_agent.py"),
        ),
        (
            "database.py.j2",
            base_path.join("app").join("database").join("database.py"),
        ),
        (
            "base_repository.py.j2",
            base_path
                .join("app")
                .join("database")
                .join("repositories")
                .join("base_repository.py"),
        ),
    ];

    if with_docker {
        files_to_generate.push(("Dockerfile.j2", base_path.join("Dockerfile")));
        files_to_generate.push((
            "docker-compose.yml.j2",
            base_path.join("docker-compose.yml"),
        ));
    }

    for (template_name, dest_path) in files_to_generate {
        // Obtenemos el template del motor
        let tmpl = env.get_template(template_name).with_context(|| {
            format!(
                "No se encontró el template '{}' en la carpeta embebida",
                template_name
            )
        })?;

        // Renderizamos inyectando la variable `project_name` y `include_mongodb`
        let rendered =
            tmpl.render(context!(project_name => name, include_mongodb => include_mongodb))?;

        // Escribimos el resultado en el disco
        fs::write(&dest_path, rendered)
            .with_context(|| format!("Falló al escribir el archivo {:?}", dest_path))?;

        println!("  {} Creado {:?}", "✔".green(), dest_path);
    }

    Ok(())
}
