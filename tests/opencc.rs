use opencc::{Config, OpenCC, OpenCCError};
use pretty_assertions::assert_eq;

#[test]
fn tw2sp() -> Result<(), OpenCCError> {
    let opencc = OpenCC::new(Config::TW2SP)?;
    assert_eq!("凉风有讯，秋月无边，亏我思娇的情绪好比度日如年。虽然我不是玉树临风，潇洒倜傥，但我有广阔的胸襟，加强劲的臂弯。",
               &opencc.convert("涼風有訊，秋月無邊，虧我思嬌的情緒好比度日如年。雖然我不是玉樹臨風，瀟灑倜儻，但我有廣闊的胸襟，加強勁的臂彎。")?);
    Ok(())
}

#[test]
fn s2twp() -> Result<(), OpenCCError> {
    let opencc = OpenCC::new(Config::S2TWP)?;
    assert_eq!("涼風有訊，秋月無邊，虧我思嬌的情緒好比度日如年。雖然我不是玉樹臨風，瀟灑倜儻，但我有廣闊的胸襟，加強勁的臂彎。",
               &opencc.convert("凉风有讯，秋月无边，亏我思娇的情绪好比度日如年。虽然我不是玉树临风，潇洒倜傥，但我有广阔的胸襟，加强劲的臂弯。")?);
    Ok(())
}
