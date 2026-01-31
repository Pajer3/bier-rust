pub const API_URL: &str = if cfg!(debug_assertions) {
    "http://10.0.2.2:3000/graphql"
} else {
    "https://api.biertjeapp.nl/graphql"
};
