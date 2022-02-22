use clap::Parser;
use emoji;
use dialoguer;
use copypasta_ext::display::DisplayServer;

/// An emoji fuzzy finder cli written in rust 🦀✨
#[derive(Parser)]
#[clap(name = "Moji")]
#[clap(version = "0.1")]
#[clap(author = "Syed Ahkam <smahkam57@gmail.com>")]
struct Cli {
    /// the emoji name as query
    query: Option<String>,

    /// Add the chosen emoji to clipboard
    #[clap(short, long)]
    clipboard: bool,

    /// Return results in raw text format
    #[clap(short, long)]
    raw: bool
}

fn search_query(query: String) -> Vec<&'static emoji::Emoji> {
    emoji::search::search_annotation(&query, "en") // TODO: support more languages using the feature flag
}

fn get_query(cli: &Cli) -> std::io::Result<String> {
    if let Some(query) = &cli.query {
        return Ok(query.to_string())
    }

    dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Emoji name")
        .interact_text()
}

fn clean_results(results: &Vec<&'static emoji::Emoji>) -> Vec<String> {
    results.iter()
        .map(|item| format!("{}     {}", item.glyph, item.name))
        .collect()
}

fn explore_and_pick_one(results: Vec<&'static emoji::Emoji>) -> Option<&'static emoji::Emoji> {
    let cleaned = clean_results(&results);
    let selection_idx = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .items(&cleaned)
        .default(0)
        .interact_opt()
        .unwrap();

    if !selection_idx.is_some() {
        return None
    }

    Some(results[selection_idx.unwrap()])
}

fn main() {
    let cli = Cli::parse();
   
    if let Ok(query) = get_query(&cli) {
        let results = search_query(query);
        if results.is_empty() { return }

        if cli.raw {
            for item in clean_results(&results) {
                println!("{}", item);
            }
            return
        }

        if let Some(chosen_emoji) = explore_and_pick_one(results) {
            println!("You chose {}!", chosen_emoji.glyph);

            if cli.clipboard {
                let mut clipboard_provider = DisplayServer::select().try_context().unwrap();
                clipboard_provider.set_contents(chosen_emoji.glyph.to_owned()).unwrap();

                println!("Added to clipboard!");
            }
        };
    }
}
