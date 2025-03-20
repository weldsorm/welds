use crate::WeldsModel;

#[derive(Debug, Default, WeldsModel)]
#[welds(table = "products")]
#[welds_path(crate)] // needed only within the welds crate.
#[welds(HasMany(orders, OrderA, "product_id"))]
struct Product {
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Default, WeldsModel)]
#[welds(table = "buyers")]
#[welds_path(crate)] // needed only within the welds crate.
#[welds(HasMany(orders, OrderA, "buyer_id"))]
struct Buyer {
    #[welds(primary_key)]
    pub id: String,
    pub name: String,
}

#[derive(Debug, Default, WeldsModel)]
#[welds(table = "orders")]
#[welds_path(crate)] // needed only within the welds crate.
#[welds(BelongsTo(product, Product, "product_id"))]
#[welds(BelongsTo(buyer, Buyer, "buyer_id"))]
struct OrderA {
    #[welds(primary_key)]
    pub id: i32,
    pub product_id: i32,
    pub buyer_id: String,
    pub price: i32,
}

#[derive(Debug, Default, WeldsModel)]
#[welds(table = "orders")]
#[welds_path(crate)] // needed only within the welds crate.
#[welds(BelongsTo(product, Product, "product_id"))]
#[welds(BelongsTo(product2, Product, "product_id2"))]
struct OrderB {
    #[welds(primary_key)]
    pub id: i32,
    pub product_id: Option<i32>,
    pub product_id2: i32,
}

// use crate::model_traits::CheckRelationship;
// use crate::model_traits::PrimaryKeyValue;
//use crate::model_traits::ForeignKeyPartialEq;

// impl<R, Ship> CheckRelationship<R, Ship> for Order
// where
//     Ship: crate::relations::Relationship<R>,
//     R: PrimaryKeyValue,
//     Self: PrimaryKeyValue,
//     <R as PrimaryKeyValue>::PrimaryKeyType: PartialEq<i32>,
// {
//     fn check(&self, other: &R, relations: &Ship) -> bool {
//         // let pk_value = self.primary_key_value();
//         // let fk_field = "";
//         // other.foreign_key_value_eq(fk_field, pk_value);
//         todo!()
//     }
// }

//impl<Rhs> ForeignKeyPartialEq<Rhs> for Order
//where
//    i32: PartialEq<Rhs>,
//{
//    fn eq(&self, foreign_key_field: &str, other: &Rhs) -> bool {
//        match foreign_key_field {
//            "product_id" => self.product_id.eq(other),
//            //"product_id" => return other == self.product_id,
//            _ => false,
//        }
//    }
//}

// impl crate::model_traits::ForeignKeyPartialEq<i32> for Order {
//     fn eq(&self, foreign_key_field: &str, other: &i32) -> bool {
//         match foreign_key_field {
//             "product_id" => self.product_id.eq(other),
//             //"product_id" => return other == self.product_id,
//             _ => false,
//         }
//     }
// }

//impl ForeignKeyPartialEq<String> for Order {
//    fn eq(&self, foreign_key_field: &str, other: &String) -> bool {
//        match foreign_key_field {
//            _ => false,
//        }
//    }
//}

#[test]
fn should_be_able_to_read_the_pk() {
    futures::executor::block_on(async move {
        let p = Product {
            id: 33,
            name: Default::default(),
        };
        let value = super::PrimaryKeyValue::primary_key_value(&p);
        assert_eq!(33, value);
    });
}

#[test]
fn should_be_able_to_equal_fks() {
    futures::executor::block_on(async move {
        let order = OrderA {
            id: 33,
            product_id: 234,
            buyer_id: "B1".to_string(),
            price: 11,
        };
        assert!(!super::ForeignKeyPartialEq::eq(&order, "product_id", &2),);
        assert!(super::ForeignKeyPartialEq::eq(&order, "product_id", &234),);
        assert!(!super::ForeignKeyPartialEq::eq(
            &order,
            "buyer_id",
            &"B2".to_string()
        ),);
        assert!(super::ForeignKeyPartialEq::eq(
            &order,
            "buyer_id",
            &"B1".to_string()
        ),);
    });
}

#[test]
fn should_be_able_to_equal_fks_optional() {
    futures::executor::block_on(async move {
        let order2 = OrderB {
            id: 33,
            product_id: Some(234),
            product_id2: 234,
        };
        assert!(super::ForeignKeyPartialEq::eq(&order2, "product_id", &234));
        assert!(super::ForeignKeyPartialEq::eq(&order2, "product_id2", &234));
    });
}
