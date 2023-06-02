// use serde::Serialize;

use odoorpc::Client;

// #[derive(Serialize, Debug)]
// struct ResPartner<'a> {
//     name: &'a str,
//     email: &'a str,
// }

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

    // let partners = odoo
    //     .env("c.aws.accounts")
    //     .browse(1)
    //     .get("c_aws_account_name")?;
    // println!("{partners:?}");
    //
    // let v = ResPartner {
    //     name: "Rust 1",
    //     email: "rust1@example.com",
    // };
    // println!("{v:?}");
    //
    // let partner = odoo
    //     .env("res.partner")
    //     .create(v)?;
    // println!("{partner}");
    // let name = partner.get("name")?;
    // println!("{name:?}");
    // let res = partner.write(Data::from([
    //     ("name".to_string(), Value::from("Rust 6")),
    // ]).0)?;
    // println!("{res}");

    Ok(())
}
