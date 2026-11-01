use minijinja::Environment;
use tokio::fs;
use tokio::io;

pub(crate) async fn construct_env() -> io::Result<Environment<'static>> {
    let mut env = Environment::new();

    env.set_auto_escape_callback(|_| minijinja::AutoEscape::None); // Chatto escapes for us

    let mut dir = fs::read_dir("./templates/command").await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        if path.is_file() {
            let stem = path
                .file_stem()
                .ok_or_else(|| tokio::io::Error::other("couldnt get stem"))?
                .to_string_lossy()
                .into_owned();

            let contents = fs::read_to_string(&path).await?;

            let contents: String = contents
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect::<Vec<String>>()
                .join("\n");

            env.add_template_owned(format!("command/{stem}"), contents)
                .map_err(|_| tokio::io::Error::other("template has a syntax error"))?;
        }
    }

    let mut dir = fs::read_dir("./templates/reaction_added").await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        if path.is_file() {
            let stem = path
                .file_stem()
                .ok_or_else(|| tokio::io::Error::other("couldnt get stem"))?
                .to_string_lossy()
                .into_owned();

            let contents = fs::read_to_string(&path).await?;

            let contents: String = contents
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect::<Vec<String>>()
                .join("\n");

            env.add_template_owned(format!("reaction_added/{stem}"), contents)
                .map_err(|_| tokio::io::Error::other("template has a syntax error"))?;
        }
    }

    let mut dir = fs::read_dir("./templates/reaction_removed").await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        if path.is_file() {
            let stem = path
                .file_stem()
                .ok_or_else(|| tokio::io::Error::other("couldnt get stem"))?
                .to_string_lossy()
                .into_owned();

            let contents = fs::read_to_string(&path).await?;

            let contents: String = contents
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect::<Vec<String>>()
                .join("\n");

            env.add_template_owned(format!("reaction_removed/{stem}"), contents)
                .map_err(|_| tokio::io::Error::other("template has a syntax error"))?;
        }
    }

    Ok(env)
}
