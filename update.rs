db.<collection-name>.updateOne()                              -> updates a single document
db.<collection-name>.updateMany()                             -> updates multiple documents
db.<collection-name>.update({find},{update},{options})

db.<collection-name>.replaceOne({query},{replace})            -> it will replace the single document.
db.<collection-name>.replaceMany({query},{replace})           -> it will replace all the documents that are specified.