use otdb::blocking::Client;
use otdb::Category;

fn main() -> Result<(), otdb::HttpError> {
    let mut client = Client::new();
    client.set_token(client.generate_token()?);

    // We can get some trivia and print them
    for trivia in client.trivia().send()?.results {
        println!("{trivia:?}");
    }

    // Or we can get information about a category
    let response = client.category_details(Category::Animals).send()?;
    println!("{response:?}");

    // Or get details of the whole API
    let response = client.global_details().send()?;
    println!("{response:?}");

    Ok(())
}
