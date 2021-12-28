use serde_json::json;

fn main() {
    let capitals = json!({
        "Cook Islands": "Avarua",
        "Fiji": "Suva",
        "Kiribati": "South Tarawa",
        "Niue": "Alofi",
        "Tonga": "Niku'alofa",
        "Tuvalu": "Funafuti"
    });
    let tongan_capital = &capitals["Tonga"];
    println!("Tongaの首都は: {}", tongan_capital);
}
