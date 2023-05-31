use xmlrpc::{Error};
use serde_xmlrpc::Value;
use serde::Serialize;

use odoorpc::{Client, Data};

#[derive(Serialize)]
struct ResPartner<'a> {
    name: &'a str,
    email: &'a str,
}

fn main() -> Result<(), Error> {
    let mut odoo = Client::new(
        "dev",
        "admin",
        "admin",
        "http://127.0.0.1:8069",
    )?;
    println!("{}", odoo.browse(2));
    println!("{:?}", odoo.browse(2).get("name")?);

    let partners = odoo
        .env("c.aws.accounts")
        .browse(1)
        .get("c_aws_account_name")?;
    println!("{partners:?}");

    let partner = odoo
        .env("res.partner")
        // .create(Data::from([
        //     ("name".to_string(), Value::from("Rust 1")),
        //     ("email".to_string(), Value::from("rust1@example.com")),
        // ]).0)?;
        .create2(serde_xmlrpc::to_value(ResPartner {
            name: "Rust 1",
            email: "rust1@example.com",
        }).unwrap())?;
    println!("{partner}");
    let name = partner.get("name")?;
    println!("{name:?}");
    // let res = partner.write(Data::from([
    //     ("name".to_string(), Value::from("Rust 6")),
    // ]).0)?;
    // println!("{res}");

    Ok(())
}
