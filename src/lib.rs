use std::io;

use io::BufRead;
use io::Read;
use io::Seek;

use io::BufWriter;
use io::Write;

use std::fs::File;

use serde_json::Map;
use serde_json::Value;

use zip::ZipArchive;
use zip::read::ZipFile;

pub use serde_json;
pub use zip;

pub fn zip2items2jsons<R, J>(
    mut zfile: ZipArchive<R>,
    item2json: &mut J,
    //mut buf: Vec<u8>,
    buf: &mut Vec<u8>,
) -> impl Iterator<Item = Result<Map<String, Value>, io::Error>>
where
    R: Read + Seek,
    J: FnMut(&[u8]) -> Result<Map<String, Value>, io::Error>,
{
    let zsz: usize = zfile.len();

    let mut i: usize = 0;

    std::iter::from_fn(move || match i < zsz {
        false => None,
        true => {
            let rjson: Result<Map<String, Value>, _> = zfile
                .by_index(i)
                .map_err(io::Error::other)
                .and_then(|mut zitem: ZipFile<_>| {
                    buf.clear();
                    io::copy(&mut zitem, buf)?;
                    item2json(buf)
                });
            i += 1;
            Some(rjson)
        }
    })
}

pub fn zips2items2jsons2writer<I, Z, R, J, O, W>(
    znames: I,
    zname2zip: Z,
    mut item2json: J,
    json2writer: O,
    mut writer: W,
) -> Result<(), io::Error>
where
    I: Iterator<Item = String>,
    Z: Fn(String) -> Result<ZipArchive<R>, io::Error>,
    R: Read + Seek,
    J: FnMut(&[u8]) -> Result<Map<String, Value>, io::Error>,
    O: Fn(&Map<String, Value>, &mut W) -> Result<(), io::Error>,
    W: Write,
{
    let mut buf: Vec<u8> = vec![];

    for zname in znames {
        let zarc: ZipArchive<R> = zname2zip(zname)?;
        let jsons = zip2items2jsons(zarc, &mut item2json, &mut buf);
        for rjobj in jsons {
            let jobj: Map<_, _> = rjobj?;
            json2writer(&jobj, &mut writer)?;
        }
    }

    writer.flush()
}

pub fn reader2znames<R>(rdr: R) -> impl Iterator<Item = String>
where
    R: BufRead,
{
    let lines = rdr.lines();
    lines.map_while(Result::ok)
}

pub fn zipfilename2zip(zname: String) -> Result<ZipArchive<File>, io::Error> {
    let f: File = File::open(zname)?;
    ZipArchive::new(f).map_err(io::Error::other)
}

pub fn slice2jobj(s: &[u8]) -> Result<Map<String, Value>, io::Error> {
    serde_json::from_slice(s).map_err(io::Error::other)
}

pub fn slice2zcat2jobj(s: &[u8], buf: &mut Vec<u8>) -> Result<Map<String, Value>, io::Error> {
    let mut zcat = flate2::bufread::GzDecoder::new(s);
    buf.clear();
    io::copy(&mut zcat, buf)?;
    slice2jobj(buf)
}

pub fn slice2zcat2jobj_new(
    mut buf: Vec<u8>,
) -> impl FnMut(&[u8]) -> Result<Map<String, Value>, io::Error> {
    move |s: &[u8]| slice2zcat2jobj(s, &mut buf)
}

pub fn jobj2writer<W>(jobj: &Map<String, Value>, mut w: &mut W) -> Result<(), io::Error>
where
    W: Write,
{
    serde_json::to_writer(&mut w, jobj).map_err(io::Error::other)?;
    writeln!(&mut w)
}

pub fn stdin2znames2zips2items2jsons2stdout_default() -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();
    let bw = BufWriter::new(&mut ol);
    zips2items2jsons2writer(
        reader2znames(io::stdin().lock()),
        zipfilename2zip,
        slice2jobj,
        jobj2writer,
        bw,
    )?;
    ol.flush()
}

pub fn stdin2znames2zips2items2zcat2jsons2stdout_default() -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();
    let bw = BufWriter::new(&mut ol);
    zips2items2jsons2writer(
        reader2znames(io::stdin().lock()),
        zipfilename2zip,
        slice2zcat2jobj_new(vec![]),
        jobj2writer,
        bw,
    )?;
    ol.flush()
}
