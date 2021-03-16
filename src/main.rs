use anyhow::*;

struct Item {
    title: Option<String>,
    description: Option<String>,
    metadata: Option<String>,
    photographer: Option<String>,
    copyright: Option<String>,
    url_large: Option<String>,
    url_normal: Option<String>,
    url_thumb: Option<String>,
}

struct SubPage {
    id: String,
    name: Option<String>,
    description: Option<String>,
    items: Option<Vec<Item>>,
}

fn main() -> Result<()> {
    // Get the galleries to download from the commandline
    let mut subpages: Vec<_> = std::env::args().skip(1).collect();
    while let Some(gallery) = subpages.pop() {
        let res = fetch_gallery_source(gallery)?;

        // Parse the XML document received
        let doc = roxmltree::Document::parse(&*res).unwrap();
        let elem = doc.descendants().find(|n| n.tag_name().name() == "mediaGroup").context("Couldn't find proper root element")?;
        //assert!(elem.is);
        dbg!(elem);
    }

    Ok(())
}

fn fetch_gallery_source(gallery: String) -> Result<String> {
    println!("Fetching data for gallery '{}'", gallery);
    let path = std::path::Path::new("cache").join(format!("{}.xml", gallery));

    // Load XML from disk if it exists, otherwise download and save it
    if let Ok(content) = std::fs::read_to_string(&path) {
        println!("Loaded gallery from cache!");
        Ok(content)
    } else {
        let url = format!("https://www.lahulotte.fr/expo_photos/{}/resources/group.xml", gallery);
        let res = reqwest::blocking::get(url)?;
        let res = res.text().context(format!("Could not fetch gallery XML for gallery {}", gallery))?;

        std::fs::create_dir_all(&path.parent().unwrap())?;
        std::fs::write(&path, &res).context("Failed to save file on the hard drive")?;

        println!("Downloaded gallery metadata");
        Ok(res)
    }
}
