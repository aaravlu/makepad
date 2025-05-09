use{
    std::fmt,
    crate::{
   //     makepad_live_tokenizer::Position,
        live_token::LiveTokenId,
        live_ptr::LiveFileId
    }
};

#[derive(Clone, Copy, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct TextPos {
    pub line: u32,
    pub column: u32
}

impl TextPos{
    pub fn to_byte_offset(&self, s:&str)->Option<usize>{
        let mut line = 0;
        let mut col = 0;
        for (byte_index, c) in s.char_indices(){
            if line == self.line as usize && col == self.column as usize{
                return Some(byte_index)
            }
            col += 1;
            if c == '\n'{
                line += 1;
                col = 0;
            }
        }
        None
    }
}
/*
impl From<TextPos> for Position {
    fn from(text_pos: TextPos) -> Position {
        Position{
            line:text_pos.line as usize,
            column:text_pos.column as usize
        }
    }*/

#[derive(Clone, Copy, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct TextSpan {
    pub file_id: LiveFileId,
    pub start: TextPos,
    pub end: TextPos
}

#[derive(Clone, Copy, Default, Debug,  Eq, Ord, PartialOrd, PartialEq)]
pub struct TokenSpan {
    pub token_id: LiveTokenId,
    pub len: usize
}

impl From<LiveTokenId> for TokenSpan {
    fn from(val: LiveTokenId) -> Self {
        TokenSpan { token_id: val, len: 1 }
    }
}

impl fmt::Display for TextSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Span(start:{}, end:{}, file_id:{})", self.start, self.end, self.file_id.to_index())
    }
}

impl fmt::Debug for TextSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Span(start:{}, end:{}, file_id:{})", self.start, self.end, self.file_id.to_index())
    }
}

impl fmt::Display for TextPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl fmt::Debug for TextPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

