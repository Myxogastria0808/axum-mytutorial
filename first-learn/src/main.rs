use axum::{response::Html, routing::get, Router};
use serde::{Deserialize, Serialize};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    //Router
    let app = Router::new()
        .route("/", get(top_handler))
        .route("/hello", get(hello_handler))
        .route("/id_param/:id", get(id_param_handler))
        .route("/id_user_param/:id/:user", get(id_user_param_handler))
        .route("/query", get(query_handler))
        .route("/json/:id", get(json_handler));

    //Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

//*ハンドラ関数 */
// async fn handler() -> Html<&'static str> {
//     Html("<h1>Hello, World!</h1>")
// }
async fn top_handler() -> String {
    "This is a root page".to_string()
}

async fn hello_handler() -> String {
    "This is a hello page".to_string()
}

//パスパラメータ―
//1つのとき
async fn id_param_handler(axum::extract::Path(id): axum::extract::Path<u32>) -> String {
    format!("User ID: {}", id)
}

//複数のとき
async fn id_user_param_handler(
    axum::extract::Path((id, user)): axum::extract::Path<(u32, String)>,
) -> String {
    format!("User ID: {}, User: {}", id, user)
}

//クエリパラメータ
///query?id={u32}&name={String}
async fn query_handler(
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> String {
    format!(
        "Query param, ID: {}, Name: {}",
        params["id"], params["name"]
    )
}

//json
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MyData {
    name: String,
    mail: String,
    age: u32,
}

async fn json_handler(
    axum::extract::Path(id): axum::extract::Path<usize>,
) -> axum::Json<serde_json::Value> {
    let data: Vec<MyData> = vec![
        MyData {
            name: "Hello".to_string(),
            mail: "hello@sample.com".to_string(),
            age: 12,
        },
        MyData {
            name: "World".to_string(),
            mail: "world@sample.com".to_string(),
            age: 12,
        },
        MyData {
            name: "Unko".to_string(),
            mail: "unko@sample.com".to_string(),
            age: 12,
        },
    ];
    let item = &data[id];
    let result = serde_json::json!(item);
    axum::Json(result)
}
