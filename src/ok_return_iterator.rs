fn split_feature_string(features: &'a str) -> I
where
    I: IntoIterator,
    I::Item: Into<&'a str>,
{
    features.to_lowercase().split_whitespace().rev()
}
