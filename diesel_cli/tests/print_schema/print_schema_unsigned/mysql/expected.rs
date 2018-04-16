table! {
    /// Representation of the `users1` table.
    ///
    /// (Automatically generated by Diesel.)
    users1 (id) {
        /// The `id` column of the `users1` table.
        ///
        /// Its SQL type is `Unsigned<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Unsigned<Integer>,
        /// The `x` column of the `users1` table.
        ///
        /// Its SQL type is `Nullable<Unsigned<Integer>>`.
        ///
        /// (Automatically generated by Diesel.)
        x -> Nullable<Unsigned<Integer>>,
    }
}

table! {
    /// Representation of the `users2` table.
    ///
    /// (Automatically generated by Diesel.)
    users2 (id) {
        /// The `id` column of the `users2` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    users1,
    users2,
);
