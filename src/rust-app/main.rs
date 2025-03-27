use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://nginx";
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    println!("Stiahnutý obrázok má veľkosť: {} bajtov", bytes.len());

    // Ak chceš zistiť rozmery obrázku
    let img = image::load_from_memory(&bytes)?;
    println!("Rozmery obrázku: {}x{}", img.width(), img.height());

    Ok(())
}
