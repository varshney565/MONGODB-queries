use <db-name>       --> create a new db || using existing db

db.<collection-name>.insertOne({})                            -> insert single document in the db.
db.<collection-name>.insertMany({})                           -> insert documents in the db(multiple).
db.createCollection(<collection-name>)                        -> create a uncapped collection.
db.createCollection(<collection-name>,{capped,size,max})      -> create a capped collection.