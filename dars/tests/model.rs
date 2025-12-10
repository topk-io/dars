use dars::{Field, Model};

#[test]
fn test_model_fields() {
    #[Model]
    struct Struct {
        #[field(desc = "a field")]
        a: String,

        #[field]
        b: i32,
    }

    #[Model]
    struct Model {
        #[field(desc = "string field")]
        str_field: String,

        #[field]
        int_field: i32,

        #[field(desc = "boolean field")]
        bool_field: bool,

        #[field(desc = "vector field")]
        vec_field: Vec<String>,

        #[field]
        struct_field: Struct,
    }

    assert_eq!(
        Model::fields(),
        &[
            Field {
                name: "str_field",
                description: Some("string field")
            },
            Field {
                name: "int_field",
                description: None
            },
            Field {
                name: "bool_field",
                description: Some("boolean field")
            },
            Field {
                name: "vec_field",
                description: Some("vector field")
            },
            Field {
                name: "struct_field",
                description: None
            }
        ]
    );
}
