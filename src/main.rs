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
        "dev",
        "admin",
        "admin",
        "res.users",
        "http://127.0.0.1:8069",
    ).await?;
    println!("{odoo}");
    println!("{}", odoo.browse(1).get("name").await?);

    let user = odoo
        .env("res.users")
        .browse(1)
        .get("email")
        .await?;
    println!("{user:?}");

    odoo.env("res.partner")
        .create(json!([
            {
                "name": "Partner A",
                "email": "partnera@example.com",
            },
            {
                "name": "Partner B",
                "email": "partnerb@example.com",
            },
        ]))
        .await?;
    println!("Recordset is multiple after create: {odoo}");
    odoo.write(json!({
        "name": "Partner X",
    })).await?;
    println!("Recordset is maintained after write: {odoo}");

    let partner_data = ResPartner {
        name: "Partner 2",
        email: "partner2@example.com",
    };
    println!("{partner_data:?}");
    let partner = odoo
        .env("res.partner")
        .create(serde_json::to_value(vec![partner_data])?)
        .await?;
    println!("{partner}");

    let name = partner.get("name").await?;
    println!("{name:?}");

    partner.write(json!({
        "name": "Partner 3",
    })).await?;

    odoo.search(json!([
        ["name", '=', "Partner 3"],
    ])).await?;
    println!("{odoo}");

    odoo.unlink().await?;

    odoo.read(json!([
        "name", "id", "email",
    ])).await?;

    odoo.search_read(
        json!([
            ["name", "=", "Partner 2"],
        ]),
        json!([
            "name", "company_id", "email",
        ]),
    ).await?;
    Ok(())
}
