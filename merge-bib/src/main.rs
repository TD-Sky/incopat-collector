mod cli;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use calamine::{deserialize_as_date_or_string, open_workbook, Reader, Xlsx};
use chrono::NaiveDate;
use clap::Parser;
use cli::Cli;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rfd::FileDialog;
use rust_xlsxwriter::Workbook;
use serde::{Deserialize, Serialize};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let dir = if let Some(dir) = &cli.dir {
        dir
    } else {
        &FileDialog::new()
            .set_directory(env::current_dir().context("current directory")?)
            .pick_folder()
            .context("需要指定目录")?
    };
    let bibs = read_all(dir)?;
    write("bibs.xlsx", &bibs)?;

    Ok(())
}

fn read_all(dir: impl AsRef<Path>) -> anyhow::Result<Vec<Bib>> {
    let paths: Vec<PathBuf> = fs::read_dir(dir)?
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            if p.extension().is_some_and(|s| s == "xlsx") {
                Some(p)
            } else {
                None
            }
        })
        .collect();

    let mut bibss = paths
        .par_iter()
        .map(|p| read(p))
        .collect::<Result<Vec<_>, _>>()?;
    bibss.sort_by_key(|bibs| bibs[0].id);

    Ok(bibss.into_iter().flatten().collect())
}

fn read(file: &Path) -> anyhow::Result<Vec<Bib>> {
    let mut wb: Xlsx<_> = open_workbook(file)?;
    let ws = &wb.worksheets()[0].1;
    Ok(ws.deserialize::<Bib>()?.flatten().collect())
}

fn write(file: impl AsRef<Path>, data: &[Bib]) -> anyhow::Result<()> {
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.deserialize_headers::<Bib>(0, 0)?.serialize(&data)?;
    wb.save(file)?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Bib {
    #[serde(rename = "序号")]
    id: usize,
    #[serde(rename = "公开（公告）号")]
    pub_no: String,
    #[serde(rename = "申请人")]
    applicant: String,
    #[serde(rename = "申请号")]
    appli_no: String,
    #[serde(rename = "申请日")]
    appli_date: ValueOrString<NaiveDate>,
    #[serde(rename = "专利类型")]
    kind: String,
    #[serde(rename = "公开（公告）日")]
    date: ValueOrString<NaiveDate>,
    #[serde(rename = "国民经济行业(主)")]
    main_sectors: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum ValueOrString<T> {
    Value(T),
    String(String),
}

impl<'de> Deserialize<'de> for ValueOrString<NaiveDate> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ret = match deserialize_as_date_or_string(deserializer)? {
            Ok(v) => Self::Value(v),
            Err(s) => Self::String(s),
        };
        Ok(ret)
    }
}
