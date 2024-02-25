
/// RESP is Redis Serialization Protocol.
/// https://redis.io/docs/reference/protocol-spec/
#[derive(Debug)]
pub struct RespParsingError {
    pub message: String,
}

pub fn parse_resp_format(input: &[u8]) -> Result<Vec<String>, RespParsingError> {
    if input.is_empty() || input[0] != b'*' {
        return Err(RespParsingError { message: "Input does not start with *".to_string() });
    }
    let input_str = std::str::from_utf8(input).unwrap();
    let mut lines = input_str.split("\r\n");
    let mut result: Vec<String> = Vec::new();
    while let Some(line) = lines.next() {
        if line.starts_with('*') || line.starts_with('$') || line.is_empty() {
            // This line indicates the number of arguments or the length of the next line, so we skip it
            continue;
        }
        result.push(line.to_string());
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_redis_protocol() {
        let input = b"*2\r\n$4\r\necho\r\n$3\r\nhey\r\n";
        let result = parse_resp_format(input).unwrap();
        assert_eq!(result, vec!["echo", "hey"]);

        let input = b"*1\r\n$4\r\nping\r\n";
        let result = parse_resp_format(input).unwrap();
        assert_eq!(result, vec!["ping"]);
    }

    #[test]
    fn test_parse_redis_protocol_error() {
        let input = b"2\r\n$4\r\necho\r\n$3\r\nhey\r\n";
        let result = parse_resp_format(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_get_command() {
        let input = b"*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n";
        let result = parse_resp_format(input).unwrap();
        assert_eq!(result, vec!["GET", "foo"]);
    }

    #[test]
    fn test_parse_set_command() {
        let input = b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";
        let result = parse_resp_format(input).unwrap();
        assert_eq!(result, vec!["SET", "foo", "bar"]);
    }
}