use std::env;
use std::process;
use std::path::PathBuf;
use std::path::Component;
use std::borrow::Cow;

fn basename(s: &str) -> &str {
    match s.rsplit('/').next() {
        Some(v) => v,
        _ => s,
    }
}

// cononicalize but not traverse symlink
fn canonicalize(p: &PathBuf) -> PathBuf {
    let mut q = PathBuf::new();

    for v in p.components() {
        if v == Component::ParentDir {
            if q.parent() != None {   //not rootdir or empty
                q.pop();
            }
        }
        else {
            q.push(v);
        }
    }
    q
}

fn main() -> std::io::Result<()> {
    let mut level = 0;
    let mut cwd = env::current_dir()?;
    let mut filepath  = match env::args().skip(1).next(){
        Some(v) => PathBuf::from(&v),
        _ => {
            let prog = env::args().next().unwrap();
            println!("usage: {} <path>", basename(&prog));
            process::exit(1);
        }
    };

    loop {
        let mut done = true;

        let abspath = canonicalize(&cwd.join(&filepath));  // join() return just @filepath if @filepath is abs; otherwise @cwd/@filepaqth
        let filetype: Cow<'static,str> = match abspath.symlink_metadata() {  // exists() is not suitable here since it travels symlinks, as well as metadata()
            Ok(meta) => {
                if meta.is_dir() { "d".into() }
                else if meta.is_symlink() { done = false; "l".into() }   // to run the next iteration!
                else if meta.is_file() { "f".into() }
                else { "?".into() }
            },
            Err(err) => {
                err.kind().to_string().into()
            }
        };
        println!("{} {}", if level > 0 { "->" } else { " *" }, filepath.display());
        println!("   {} [{}]", abspath.display(), filetype);

        if done { break; }

        filepath = abspath.read_link()?;
        cwd = match abspath.parent() {
            Some(v) => v.to_path_buf(),
            None => PathBuf::from("/"),
        };
        level += 1;
    }

    Ok(())
}
