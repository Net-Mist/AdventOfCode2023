use arrayvec::ArrayVec;
use std::{fmt::Debug, str::FromStr};

#[inline(always)]
fn split_parse_iter<T>(line: &str, delimiter: char) -> impl Iterator<Item = T> + '_
where
    T: FromStr + Debug,
    T::Err: std::fmt::Debug,
{
    line.split(delimiter).map(|v| v.parse().expect("can't parse value {v:?}"))
}

#[inline(always)]
fn split_parse_vec<T>(line: &str, delimiter: char) -> Vec<T>
where
    T: FromStr + Debug,
    T::Err: std::fmt::Debug,
{
    split_parse_iter(line, delimiter).collect()
}

#[inline(always)]
fn split_parse_arrayvec<T, const CAP: usize>(line: &str, delimiter: char) -> ArrayVec<T, CAP>
where
    T: FromStr + Debug,
    T::Err: std::fmt::Debug,
{
    split_parse_iter(line, delimiter).collect()
}

#[inline(always)]
fn split_parse_block_vec<T>(block: &str, delimiter: char) -> Vec<Vec<T>>
where
    T: FromStr + Debug,
    T::Err: std::fmt::Debug,
{
    block
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .split(delimiter)
                .filter(|&v| !v.is_empty())
                .map(|v| v.parse().expect("can't parse value {v:?}"))
                .collect()
        })
        .collect::<Vec<Vec<T>>>()
}

#[inline(always)]
fn split_parse_block_arrayvec<T, const CAP: usize, const CAP2: usize>(
    block: &str,
    delimiter: char,
) -> ArrayVec<ArrayVec<T, CAP>, CAP2>
where
    T: FromStr + Debug,
    T::Err: std::fmt::Debug,
{
    block
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .split(delimiter)
                .filter(|&v| !v.is_empty())
                .map(|v| v.parse().expect("can't parse value {v:?}"))
                .collect()
        })
        .collect()
}

///Conversion trait for string to automatically convert to vec, arrayvac, ...
pub trait StrConversion {
    fn to_arrayvec<T, const CAP: usize>(&self, delimiter: char) -> ArrayVec<T, CAP>
    where
        T: FromStr + Debug,
        T::Err: std::fmt::Debug;

    fn to_arrayvec_block<T, const CAP: usize, const CAP2: usize>(
        &self,
        delimiter: char,
    ) -> ArrayVec<ArrayVec<T, CAP>, CAP2>
    where
        T: FromStr + Debug,
        T::Err: std::fmt::Debug;

    fn to_vec<T>(&self, delimiter: char) -> Vec<T>
    where
        T: FromStr + Debug,
        T::Err: std::fmt::Debug;

    fn to_vec_block<T>(&self, delimiter: char) -> Vec<Vec<T>>
    where
        T: FromStr + Debug,
        T::Err: std::fmt::Debug;

    fn to_vec_block_ascii_whitespace<T>(&self) -> Vec<Vec<T>>
    where
        T: FromStr + Debug,
        T::Err: std::fmt::Debug;

    fn to_arrayvec_block_ascii_whitespace<T, const CAP: usize, const CAP2: usize>(
        &self,
    ) -> ArrayVec<ArrayVec<T, CAP>, CAP2>
    where
        T: FromStr + Debug,
        T::Err: std::fmt::Debug;
}

impl StrConversion for str {
    fn to_arrayvec<T, const CAP: usize>(&self, delimiter: char) -> ArrayVec<T, CAP>
    where
        T: FromStr + Debug,
        T::Err: std::fmt::Debug,
    {
        split_parse_arrayvec(self, delimiter)
    }

    fn to_arrayvec_block<T, const CAP: usize, const CAP2: usize>(
        &self,
        delimiter: char,
    ) -> ArrayVec<ArrayVec<T, CAP>, CAP2>
    where
        T: FromStr + Debug,
        T::Err: std::fmt::Debug,
    {
        split_parse_block_arrayvec(self, delimiter)
    }

    fn to_vec<T>(&self, delimiter: char) -> Vec<T>
    where
        T: FromStr + Debug,
        T::Err: std::fmt::Debug,
    {
        split_parse_vec(self, delimiter)
    }

    fn to_vec_block<T>(&self, delimiter: char) -> Vec<Vec<T>>
    where
        T: FromStr + Debug,
        T::Err: std::fmt::Debug,
    {
        split_parse_block_vec(self, delimiter)
    }

    fn to_vec_block_ascii_whitespace<T>(&self) -> Vec<Vec<T>>
    where
        T: FromStr + Debug,
        T::Err: std::fmt::Debug,
    {
        self.lines()
            .map(|l| l.split_ascii_whitespace().map(|v| v.parse().expect("can't parse value {v:?}")).collect())
            .collect::<Vec<Vec<T>>>()
    }

    fn to_arrayvec_block_ascii_whitespace<T, const CAP: usize, const CAP2: usize>(
        &self,
    ) -> ArrayVec<ArrayVec<T, CAP>, CAP2>
    where
        T: FromStr + Debug,
        T::Err: std::fmt::Debug,
    {
        self.trim()
            .lines()
            .map(|l| {
                l.trim_start().split_ascii_whitespace().map(|v| v.parse().expect("can't parse value {v:?}")).collect()
            })
            .collect()
    }
}
