show dbs;                           --> list all the dbs
show collections;                   --> show the available collections


copy all the documents from one collection to another collection
db.<collection1>.find().forEach((doc) => {
    db.<collection2>.insert(doc)
});


Schema validator
db.createCollection("students", {
    validator: {
        $jsonSchema: {
            properties: {
                name: {
                    description: "string and is mandatory",
                    bsonType: "string"
                },
                year: {
                    description: "integer between [ 2017, 3017 ] and mandatory",
                    maximum: 3017,
                    bsonType: "int",
                    minimum: 2017,
                },
                major: {
                    description: "can only be one of the enum values and mandatory",
                    enum: ["Maths", "English", "Computer Science", "History", null
                    ]
                },
                gpa: {
                    bsonType: ["double"],
                    description: "double if the field exists"
                },
                address: {
                    bsonType: "object",
                    required: ["city"],
                    properties: {
                        street: {
                            bsonType: "string",
                            description: "string if the field exists"
                        },
                        city: {
                            bsonType: "string",
                            description: "string and mandatory"
                        }
                    }
                }
            },

            required: ["name", "year", "major", "address"],
            bsonType: "object"
        }
    }
});