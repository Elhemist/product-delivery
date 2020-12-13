table! {
    use diesel::sql_types::*;
    use crate::models::*;

    addresses (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        street -> Text,
        home -> Text,
        apartment -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    courier (user_id) {
        user_id -> Int4,
        transport_id -> Nullable<Int4>,
        status -> Courier_status,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    orders (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        courier_id -> Nullable<Int4>,
        address_id -> Nullable<Int4>,
        status -> Order_status,
        total_sum -> Numeric,
        comment -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    product_category (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    products (id) {
        id -> Int4,
        category -> Int4,
        name -> Text,
        description -> Text,
        price -> Numeric,
        image_url -> Text,
        available -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    products_in_orders (id) {
        id -> Int4,
        order_id -> Nullable<Int4>,
        product_id -> Nullable<Int4>,
        quantity -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    transport (id) {
        id -> Int4,
        number -> Text,
        color -> Transport_colors,
        model -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    users (id) {
        id -> Int4,
        phone -> Text,
        password_hash -> Text,
        role -> User_role,
        first_name -> Nullable<Text>,
        middle_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
    }
}

joinable!(addresses -> users (user_id));
joinable!(courier -> transport (transport_id));
joinable!(courier -> users (user_id));
joinable!(orders -> addresses (address_id));
joinable!(orders -> courier (courier_id));
joinable!(orders -> users (user_id));
joinable!(products -> product_category (category));
joinable!(products_in_orders -> orders (order_id));
joinable!(products_in_orders -> products (product_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    courier,
    orders,
    product_category,
    products,
    products_in_orders,
    transport,
    users,
);