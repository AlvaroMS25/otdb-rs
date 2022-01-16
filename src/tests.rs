use crate::prelude::*;

#[tokio::test]
async fn test() -> crate::OTDBResult<()> {
    let mut client = Client::new();

    //client.set_token(client.generate_token().await?);

    let mut res = client.trivia_request();
    res
        .question_number(15)
        .kind(Kind::MultipleChoice)
        .category(Category::Computers)
        .difficulty(Difficulty::Hard);

    println!("{:#?}", res);
    let res = res.await?;

    println!("{:#?}", res);
    Ok(())
}

#[tokio::test]
async fn owned_request() -> crate::OTDBResult<()> {
    let mut client = Client::new();
    let request = client.trivia_request().into_owned();

    println!("{:#?}", request);
    let res = request.await?;

    println!("{:#?}", res);

    Ok(())
}

#[tokio::test]
async fn custom_endpoint() -> crate::OTDBResult<()> {
    let mut client = Client::new();
    let res: Request<crate::model::TokenRequestResponse> = client.new_request(String::from("https://opentdb.com/api_token.php?command=request"));

    println!("{:?}", res);

    println!("{:#?}", res.await?);

    Ok(())
}
