/// # Example
/// ```ignore
/// let content_object: Result<Object, Error> = jsonvalue_unwrap_or_return!(JsonValue::Object, object)
/// ```
macro_rules! jsonvalue_unwrap_or_return {
    ($variant:path, $o:expr) => {
        if let $variant(x) = $o {
            Ok(x)
        } else {
            Err(Error(
                format!("Cannot cast to {}. $o: {}", stringify!($variant), $o),
                None,
            ))
        }
    };
}

pub(crate) use jsonvalue_unwrap_or_return;

/// ```ignore
/// (&json::object::Object, &str, identifer) -> Result(&JsonValue, String)
/// ```
/// # Example
/// ```ignore
/// let attribute_value = simple_get!(object, "attribute", function_name)
/// ```
macro_rules! simple_get {
    ($object: expr, $key: expr) => {
        $object
            .get($key)
            .ok_or(Error(format!("wrong {} (not an attr). {:?}", $key, $object), None))
    };
}

pub(crate) use simple_get;

macro_rules! simple_get_as {
    ($object: expr, $key: expr, $as_type: ident) => {
        $object.get($key).and_then(|v| v.$as_type()).ok_or(Error(
            format!(
                "wrong {} (not an attr or invalid cast). {:?}",
                $key, $object
            ),
            None,
        ))
    };
}

pub(crate) use simple_get_as;

macro_rules! unwrap_content {
    ($response_object:expr) => {{
        let content = simple_get!($response_object, "content")?;
        jsonvalue_unwrap_or_return!(JsonValue::Object, content)?
    }};
}

pub(crate) use unwrap_content;
