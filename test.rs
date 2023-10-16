use mongodm::{ToRepository, Model, CollectionConfig, Indexes, Index, IndexOption, sync_indexes};
use mongodm::mongo::{Client, options::ClientOptions, bson::doc};
use serde::{Serialize, Deserialize};
use mongodm::prelude::*;
// field! is used to make sure at compile time that some field exists in a given structure
use mongodm::field;
use std::time::Instant;
struct UserCollConf;

impl CollectionConfig for UserCollConf {
    fn collection_name() -> &'static str {
        "user"
    }

    fn indexes() -> Indexes {
        let index1 = Index::new_with_text("description"); // --> text based indexing.
        let mut index2 = Index::new(field!(username in User));
        index2.add_key(field!(last_seen in User));
        Indexes::new()
            .with(index1)
            .with(index2.with_option(IndexOption::Unique))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    username: String,
    last_seen: i64,
    description : String
}

impl Model for User {
    type CollConf = UserCollConf;
}

#[tokio::main]
async fn main() {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("mongodm_wayk_demo");

    sync_indexes::<UserCollConf>(&db).await.unwrap();
    // indexes are now synced in backend for user collection

    let repository = db.repository::<User>(); // method provided by `ToRepository` trait

    // let users = vec![
    //     User {
    //         username : "Shivam".to_string(),
    //         last_seen : 1000,
    //         description : "Hello".to_string()
    //     },
    //     User {
    //         username : "Ramu".to_string(),
    //         last_seen : 2000,
    //         description : "Hello fsk my name shivam".to_string()
    //     },
    //     User {
    //         username : "Divyank".to_string(),
    //         last_seen : 3000,
    //         description : "Hefdsllo nfksnfkjsdb fdsfd sf adsf dsfsdf sd f dsf dsfs".to_string()
    //     },
    //     User {
    //         username : "IP_ADDD".to_string(),
    //         last_seen : 4000,
    //         description : "fdsafd hjfdsyfgayugfyadsgjfdsafjhdsafdsf".to_string()
    //     },
    // ];

    // repository.insert_many(users, None).await.unwrap();

    let start_time = Instant::now();
    let mut handles = Vec::new();
    for i in 1..9 {
        let repos = repository.clone();
        let handle = tokio::spawn(async move {
            let mut futures = Vec::new();
            for j in 1..=100000 {
                let repo = repos.clone();
                let future = async move {
                    let mut user = User {
                        username : format!("Shivam{}{}",i,j),
                        last_seen : 1000,
                        description : format!("desc{}",i)
                    };
                    if j%4345 == 0 {
                        user.description = "Hello Ben stocks".to_string()
                    }
                    repo.insert_one(user, None).await
                };
                futures.push(future);
            }
            futures::future::join_all(futures).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Execution time: {} ms", elapsed_time.as_millis());
    use mongodm::f;
    use mongodm::operator::*;

    // let res = repository.find_one(
    //     doc! {
    //         f!{username in User} : {
    //             In : ["David"]
    //         }
    //     },
    //     None
    // ).await.unwrap().unwrap();
    // println!("{:?}",res);


    // let bulk_update_res = repository
    // .bulk_update(&vec![
    //     &BulkUpdate {
    //         query: doc! { f!(last_seen in User) : {LesserThan : 3200} },
    //         update: doc! { Set: { f!(last_seen in User) : 12000 } },
    //         options: None,
    //     }
    // ])
    // .await
    // .unwrap();
    let _update = repository.update_many(
        doc! { f!(last_seen in User) : 2000 },
        doc! { Set: { f!(last_seen in User) : 0 } },
        None,
    ).await.unwrap();

    /*
     * insert_one(user,None)
     * insert_many(users,None)
     * 
     * delete_one(query,None)
     * deleteMany(query,None)
     * 
     * multiple insert_one => bulk_update(&vec![&BulkUpdate{},&BulkUpdate{}])
     * insert one          => insert_one(query,update,None)
     * insert many         => insert_many(query,update,None)
     * 
     * find_one(query)     => find a single document
     * find(query)         => find all the docs
     * 
     */

    let mut res: MongoCursor<User> = repository.find(doc! {
        Text : {
            "$search" : "Hello"
        }
    }, None).await.unwrap();

    while let Some(result) = res.next().await {
        match result {
            Ok(user) => {
                println!("{:?}", user);
            }
            Err(error) => {
                eprintln!("Error iterating through results: {}", error);
                break;
            }
        }
    }
}

