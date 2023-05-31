use xmlrpc::Error;

use odoorpc::Proxy;

fn main() -> Result<(), Error> {
    let mut odoo = Proxy::new(
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

    Ok(())
}
