pub type CtrResult<T> = Result<T, i32>;

pub fn parse_res(res: i32) -> CtrResult<()> {
    if res >= 0 {
        return Ok(());
    }

    Err(res)
}
