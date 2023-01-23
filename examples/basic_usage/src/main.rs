use otdb::*;

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    let mut client = Client::new();

    // Set the token so we don't get repeated trivia
    client.set_token(client.generate_token().await.unwrap());

    let mut request = client.trivia();
    request.question_number(20);

    // Print the trivias we received.
    for trivia in request.send().await?.results {
        println!("{trivia:?}");
    }

    // Or we can get information about a category
    let response = client.category_details(Category::Animals).send().await?;
    println!("{response:?}");

    // Or get details of the whole API
    let response = client.global_details().send().await?;
    println!("{response:?}");

    Ok(())
}
