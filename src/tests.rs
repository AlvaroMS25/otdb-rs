use crate::prelude::*;
use crate::blocking;

#[tokio::test]
async fn get_trivia() -> OTDBResult<()> {
    let client = Client::new();

    //client.set_token(client.generate_token().await?);

    let mut res = client.trivia_request();
    res
        .question_number(50)
        .kind(Kind::Any)
        .category(Category::Any)
        .difficulty(Difficulty::Any);

    println!("{:#?}", res);
    let res = res.send().await?;

    println!("{:#?}", res);
    Ok(())
}

#[tokio::test]
async fn owned_request() -> OTDBResult<()> {
    let client = Client::new();
    let request = client.trivia_request().into_owned();

    println!("{:#?}", request);
    let res = request.send().await?;

    println!("{:#?}", res);

    Ok(())
}

#[tokio::test]
async fn custom_endpoint() -> OTDBResult<()> {
    let client = Client::new();
    let res: Request<crate::model::TokenRequest> = client.new_request(String::from("https://opentdb.com/api_token.php?command=request"));

    println!("{:?}", res);

    println!("{:#?}", res.send().await?);

    Ok(())
}

#[tokio::test]
async fn category_details() -> OTDBResult<()> {
    let client = Client::new();
    let res = client.category_details(Category::Computers).send().await?;

    println!("{:?}", res);

    Ok(())
}

#[tokio::test]
async fn global_details() -> OTDBResult<()> {
    let client = Client::new();
    let res = client.global_details().send().await?;

    println!("{:?}", res);

    Ok(())
}

#[test]
fn blocking_trivia() -> OTDBResult<()> {
    let client = blocking::Client::new();
    let mut req = client.trivia_request();
    req.kind(Kind::Any)
        .category(Category::Computers);

    println!("{:?}", req.send()?);


    Ok(())
}

#[test]
fn blocking_trivia_owned() -> OTDBResult<()> {
    let client = blocking::Client::new();
    let mut req = client.trivia_request().into_owned();
    req.kind(Kind::Any)
        .category(Category::Computers);

    println!("{:?}", req.send()?);


    Ok(())
}

#[test]
fn blocking_multiple_threads() -> OTDBResult<()> {
    let client = blocking::Client::new();
    let mut handles = Vec::with_capacity(8);

    for _ in 0..8 {
        let clone = client.clone();
        handles.push(std::thread::spawn(move || {
            clone.trivia_request().send()
        }));
    }

    for i in handles {
        i.join().unwrap().unwrap();
    }

    Ok(())
}

#[test]
fn blocking_use_token() -> OTDBResult<()> {
    let mut client = blocking::Client::new();
    client.set_token(client.generate_token()?);

    Ok(())
}

#[test]
fn blocking_category_details() -> OTDBResult<()> {
    let client = blocking::Client::new();
    println!("{:?}", client.category_details(Category::BoardGames).send()?);

    Ok(())
}

#[test]
fn blocking_global_details() -> OTDBResult<()> {
    let client = blocking::Client::new();
    println!("{:?}", client.global_details().send()?);

    Ok(())
}
