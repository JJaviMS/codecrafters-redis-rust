use bytes::Buf;
use std::{
    fmt::Write,
    io::Cursor,
    str::{from_utf8, Utf8Error},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameParseError {
    #[error("Incomplete sequence")]
    Incomplete,
    #[error("Invalid utf-8 sequence")]
    InvalidString(#[from] Utf8Error),
    #[error("Invalid data")]
    InvalidData,
    #[error("Invalid number")]
    InvalidNumber,
}

#[derive(Debug)]
pub enum Frame {
    SimpleString(String),
    Error(String),
    BulkString(String),
    Array(Vec<Frame>),
    Integer(u64),
    Null,
}

impl Frame {
    pub(crate) fn parse_from_buf(src: &mut Cursor<&[u8]>) -> Result<Frame, FrameParseError> {
        if !src.has_remaining() {
            println!("Cursor is empty");
            return Err(FrameParseError::Incomplete);
        }
        let byte = src.get_u8();
        match byte {
            b'+' => {
                let line = find_line(src)
                    .map(|f| f.to_vec())
                    .ok_or(FrameParseError::Incomplete)?;
                let line = String::from_utf8(line).map_err(|e| e.utf8_error())?;

                return Ok(Frame::SimpleString(line));
            }
            b'-' => {
                let line = find_line(src)
                    .map(|f| f.to_vec())
                    .ok_or(FrameParseError::Incomplete)?;
                let line = String::from_utf8(line).map_err(|e| e.utf8_error())?;

                return Ok(Frame::Error(line));
            }
            b':' => {
                let number = find_line(src)
                    .map(|f| get_number_from_line(f))
                    .ok_or(FrameParseError::Incomplete)??;

                return Ok(Frame::Integer(number));
            }
            b'$' => {
                let expecting_size = find_line(src)
                    .map(|f| get_number_from_line(f))
                    .ok_or(FrameParseError::Incomplete)??;

                let mut final_string = String::with_capacity(expecting_size as usize);

                let string_line = find_line(src)
                    .map(|l| from_utf8(l))
                    .ok_or(FrameParseError::Incomplete)??;

                final_string.push_str(string_line);

                return Ok(Frame::BulkString(final_string));
            }

            b'*' => {
                let expecting_size = find_line(src)
                    .map(|f| get_number_from_line(f))
                    .ok_or(FrameParseError::Incomplete)??
                    as usize;

                let mut final_vec = Vec::with_capacity(expecting_size);

                for _ in 0..expecting_size {
                    let result = Self::parse_from_buf(src)?;
                    final_vec.push(result);
                }

                return Ok(Self::Array(final_vec));
            }

            b'_' => {
                find_line(src).ok_or(FrameParseError::Incomplete)?;

                return Ok(Self::Null);
            }

            _ => return Err(FrameParseError::InvalidData),
        }
    }

    pub(crate) fn extract_string_from_frame(&self) -> Option<&str> {
        return match self {
            Frame::BulkString(s) | Frame::SimpleString(s) => Some(s),

            _ => None,
        };
    }
}

impl ToString for Frame {
    fn to_string(&self) -> String {
        return match self {
            Frame::BulkString(data) => {
                format!("${}\r\n{}\r\n", data.as_bytes().len(), data)
            }

            Frame::Array(content) => {
                let mut result_string = String::with_capacity(2048);

                write!(&mut result_string, "*{}\r\n", content.len()).unwrap();

                for frame in content {
                    result_string.push_str(&frame.to_string());
                }

                result_string
            }

            Frame::Error(error) => {
                format!("-{}\r\n", error)
            }

            Frame::Null => "_\r\n".to_string(),

            Frame::SimpleString(data) => {
                format!("+{}\r\n", data)
            }

            Frame::Integer(value) => {
                format!(":{}\r\n", value)
            }
        };
    }
}

fn get_number_from_line(src: &[u8]) -> Result<u64, FrameParseError> {
    use atoi::atoi;

    return atoi(src).ok_or(FrameParseError::InvalidNumber);
}

fn find_line<'a>(src: &mut Cursor<&'a [u8]>) -> Option<&'a [u8]> {
    let start_position = src.position() as usize;
    let end = src.get_ref().len() - 1;

    for i in start_position..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            src.set_position((i + 2) as u64);

            return Some(&src.get_ref()[start_position..i]);
        }
    }

    None
}
