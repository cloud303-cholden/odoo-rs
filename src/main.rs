use serde::Serialize;
use serde_json::json;

use odoorpc::Client;

#[derive(Serialize, Debug)]
struct ResPartner<'a> {
    name: &'a str,
    email: &'a str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut odoo = Client::new(
        "dev".to_string(),
        "admin".to_string(),
        "admin".to_string(),
        "res.users".to_string(),
        "http://127.0.0.1:8069".to_string(),
    ).await?;
    println!("{}", odoo.browse(2));
    println!("{}", odoo.browse(2).get("name").await?);

    let account = odoo
        .env("c.aws.accounts".to_string())
        .browse(1)
        .get("c_aws_account_name")
        .await?;
    println!("{account:?}");

    let v = ResPartner {
        name: "Rust 2",
        email: "rust2@example.com",
    };
    println!("{v:?}");
    let partner = odoo
        .env("res.partner".to_string())
        .create(serde_json::to_value(v)?)
        .await?;
    println!("{partner}");

    let name = partner.get("name").await?;
    println!("{name:?}");

    partner.write(json!({
        "name": "Rust 6",
    })).await?;

    odoo.search(json!([
        ["name", '=', "Rust 6"],
    ])).await?;
    println!("{}", odoo);

    odoo.unlink().await?;
    odoo.unlink().await?;

    odoo.read(json!([
        "name", "id", "email",
    ])).await?;

    odoo.search_read(
        json!([
            ["name", "=", "Rust 2"],
        ]),
        json!([
            "name", "company_id", "email",
        ]),
    ).await?;
    Ok(())
}
