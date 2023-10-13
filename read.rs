db.<collection-name>.find();                                        --> it will find all the documents
db.<collection-name>.find({key1 : "value1",key2 : "value2"})        --> it will give all the documents matching the parameter
db.<collection-name>.find({key : {$in : ["v1","v2"]}})              --> it will give all the documents that's value is either v1 or v2
db.<collection-name>.find({key : {$gte : 100}})                     --> it will give all the documents who has the value GreaterorEqualto100

db.<collection-name>.find().limit(number)                           --> it will limit the number of resultant documents.
db.<collection-name>.find().sort({_id : -1})                        --> give all the documents in the desending order of the creation.   
db.<collection-name>.find({key : /^a.*o$/})                         --> starts with 'a' and end at 'o'
db.<collection-name>.find().count()                                 --> it will give the count of the documents
