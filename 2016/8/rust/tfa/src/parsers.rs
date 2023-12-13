use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::map_res,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

use crate::{ColumnRotation, Instruction, Rectangle, RowRotation};

fn parse_usize(s: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(s)
}

pub fn parse_rect(s: &str) -> IResult<&str, Instruction> {
    let (s, (width, height)) = preceded(
        terminated(tag("rect"), multispace1),
        separated_pair(parse_usize, tag("x"), parse_usize),
    )(s)?;
    Ok((s, Instruction::Rect(Rectangle { width, height })))
}

pub fn parse_row_rotation(s: &str) -> IResult<&str, Instruction> {
    let (s, (row, by)) = preceded(
        tag("rotate row y="),
        separated_pair(parse_usize, tag(" by "), parse_usize),
    )(s)?;
    Ok((s, Instruction::RotateRow(RowRotation { row, by })))
}

pub fn parse_column_rotation(s: &str) -> IResult<&str, Instruction> {
    let (s, (column, by)) = preceded(
        tag("rotate column x="),
        separated_pair(parse_usize, tag(" by "), parse_usize),
    )(s)?;
    Ok((s, Instruction::RotateColumn(ColumnRotation { column, by })))
}
