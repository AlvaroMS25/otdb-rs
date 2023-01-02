use crate::prelude::*;

#[tokio::test]
async fn get_trivia() -> crate::OTDBResult<()> {
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
async fn owned_request() -> crate::OTDBResult<()> {
    let client = Client::new();
    let request = client.trivia_request().into_owned();

    println!("{:#?}", request);
    let res = request.send().await?;

    println!("{:#?}", res);

    Ok(())
}

#[tokio::test]
async fn custom_endpoint() -> crate::OTDBResult<()> {
    let client = Client::new();
    let res: OwnedRequest<crate::model::TokenRequest> = client.new_request(String::from("https://opentdb.com/api_token.php?command=request"));

    println!("{:?}", res);

    println!("{:#?}", res.send().await?);

    Ok(())
}

#[tokio::test]
async fn category_details() -> crate::OTDBResult<()> {
    let client = Client::new();
    let res = client.category_details(Category::Computers).send().await?;

    println!("{:?}", res);

    Ok(())
}

#[tokio::test]
async fn global_details() -> crate::OTDBResult<()> {
    let client = Client::new();
    let res = client.global_details().send().await?;

    println!("{:?}", res);

    Ok(())
}
